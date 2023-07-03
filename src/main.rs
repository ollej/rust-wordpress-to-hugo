use html2md::parse_html;
use regex::Regex;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use std::io::prelude::*;
use std::{fs, io};
use unidecode::unidecode;

static DIR: &str = "./content";

#[derive(Debug, sqlx::FromRow)]
struct Term {
    name: String,
    slug: String,
}

#[derive(Debug, sqlx::FromRow)]
struct Post {
    #[sqlx(rename = "ID")]
    id: u64,
    #[sqlx(rename = "user_nicename")]
    author: String,
    #[sqlx(rename = "post_date")]
    date: NaiveDateTime,
    #[sqlx(rename = "post_content")]
    content: String,
    #[sqlx(rename = "post_title")]
    title: String,
    #[sqlx(rename = "post_name")]
    name: String,
    post_type: String,
    #[sqlx(rename = "post_mime_type")]
    mime_type: String,
    #[sqlx(rename = "post_parent")]
    parent: u64,
    #[sqlx(rename = "post_status")]
    status: String,
}

#[derive(Debug)]
struct HugoContent {
    category: String,
    title: String,
    author: String,
    date: NaiveDateTime,
    content: String,
    draft: bool,
}

impl HugoContent {
    fn filename(&self) -> String {
        format!(
            "{DIR}/{}/{}.md",
            sanitize(self.category.to_owned()),
            sanitize(self.title.to_owned())
        )
    }

    fn write(&self) -> Result<(), io::Error> {
        let mut file = fs::File::create(self.filename())?;
        file.write_all(self.content().as_bytes())?;
        Ok(())
    }

    fn content(&self) -> String {
        format!(
            r#"+++
Title = "{}"
Date = "{}"
Author = "{}"
Draft = {}
+++

{}"#,
            self.title,
            self.date,
            self.author,
            self.draft,
            self.content.replace("\r", ""),
        )
    }
}

fn sanitize(filename: String) -> String {
    let re = Regex::new(r"[^A-Za-z0-9_-]").unwrap();
    let str = unidecode(&filename.trim().to_lowercase()).replace(" ", "_");
    re.replace_all(&str, "").to_string()
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:password@127.0.0.1:3306/wordpressdb")
        .await?;

    // Read wordpress data
    // TODO: Read pages
    // TODO: Handle attachments
    // TODO: Handle terms
    let posts = sqlx::query_as::<_, Post>(
        "SELECT p.ID, user_nicename, post_date, post_content, post_title, post_name, post_type, post_mime_type, post_parent, post_status FROM wp_posts AS p, wp_users AS u WHERE p.post_author = u.ID AND post_type = 'post' AND post_status = 'publish'",
    )
    .fetch_all(&pool)
    .await?;

    // Create Hugo content
    fs::create_dir_all(&DIR)?;
    for post in posts {
        let content = HugoContent {
            category: "blog".to_string(),
            title: post.title,
            author: post.author,
            date: post.date,
            content: parse_html(&post.content),
            draft: post.status == "publish",
        };
        content.write()?;
    }

    Ok(())
}
