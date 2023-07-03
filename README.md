# Wordpress 2 Hugo

Convert blog posts from an old Wordpress site to Hugo.

## Setup MariaDB docker

```bash
$ docker run --detach --name wordpress-db -p 3306:3306 --env MARIADB_USER=wordpress-user --env MARIADB_PASSWORD=password --env MARIADB_ROOT_PASSWORD=password mariadb:latest
```

## Create MariaDB database

```bash
$ docker exec -i wordpress-db sh -c 'exec mariadb -uroot -p"password" -e "create database wordpressdb"'
```

## Import MariaDB database

```bash
$ docker exec -i wordpress-db sh -c 'exec mariadb -uroot -p"password" -D wordpressdb' < wordpress.sql
```

## Connect to MariaDB database

```bash
$ docker exec -it wordpress-db sh -c 'exec mariadb -uroot -p"password" -D wordpressdb'
```
