use html2md::parse_html;
use regex::Regex;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::types::chrono::NaiveDateTime;
use std::io::prelude::*;
use std::{fs, io};
use unidecode::unidecode;

static DATABASE_URL: &str = "mysql://root:password@127.0.0.1:3306/wordpressdb";
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
    terms: String,
}

#[derive(Debug)]
struct HugoContent {
    category: String,
    title: String,
    author: String,
    date: NaiveDateTime,
    content: String,
    draft: bool,
    tags: Vec<String>,
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
Tags = {:?}
+++

{}"#,
            self.title,
            self.date,
            self.author,
            self.draft,
            self.tags,
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
        .max_connections(1)
        .connect(DATABASE_URL)
        .await?;

    // Read wordpress data
    // TODO: Read pages
    // TODO: Handle attachments
    let posts = sqlx::query_as::<_, Post>(
        "SELECT p.ID, user_nicename, post_date, post_content, post_title, post_name, post_type, post_mime_type, post_parent, post_status, GROUP_CONCAT(t.name) AS terms
        FROM wp_posts AS p
        INNER JOIN wp_users AS u ON p.post_author = u.ID 
        INNER JOIN wp_term_relationships AS tr ON p.ID = tr.object_id
        INNER JOIN wp_term_taxonomy AS tt ON tr.term_taxonomy_id = tt.term_taxonomy_id
        INNER JOIN wp_terms AS t ON tt.term_id = t.term_id
        WHERE post_type = 'post' AND post_status = 'publish'
        GROUP BY p.ID",
    )
    .fetch_all(&pool)
    .await?;

    // Create Hugo content
    fs::create_dir_all(&DIR)?;
    for post in posts {
        let content = HugoContent {
            category: "blog".to_string(),
            title: post.title,
            author: "Olle Wreede".to_string(),
            date: post.date,
            content: textwrap::wrap(&parse_html(&post.content), 75).join("\n"),
            draft: post.status != "publish",
            tags: post.terms.split(",").map(String::from).collect(),
        };
        content.write()?;
    }

    Ok(())
}
