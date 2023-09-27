# Wordpress 2 Hugo

Convert blog posts from an old Wordpress site to Hugo.

## Usage

Update the `DATABASE_URL` in `src/main.rs` to the MySQL/MariaDB Wordpress
database. The Hugo content files will be created in the directory `content`.

## Setup MariaDB locally

Some helpful commands to setup a local MariaDB database server using Docker.

### Setup MariaDB docker

```bash
docker run --detach --name wordpress-db -p 3306:3306 --env MARIADB_USER=wordpress-user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=password mariadb:latest
```

### Create MariaDB database

```bash
docker exec -i wordpress-db sh -c 'exec mariadb -uroot -p"password" -e "create database wordpressdb"'
```

### Import MariaDB database

```bash
docker exec -i wordpress-db sh -c 'exec mariadb -uroot -p"password" -D wordpressdb' < wordpress.sql
```

### Connect to MariaDB database

```bash
docker exec -it wordpress-db sh -c 'exec mariadb -uroot -p"password" -D wordpressdb'
```

### Fix column with incorrect encoding

An SQL command that might be helpful if the encoding of a column is incorrect
to convert from latin1/ISO-8859-1 to UTF-8.

```sql
UPDATE wp_terms SET name = CONVERT(CONVERT(CONVERT(name USING latin1) USING binary) USING UTF8);
```
