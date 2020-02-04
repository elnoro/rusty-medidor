**Medidor** is a project to collect business metrics on legacy projects.

Limitations: currently only MySQL is supported.

How to build:

Without docker: 
1. [Install Rust](https://www.rust-lang.org/tools/install)
2. `cargo build`

With docker:
1. `cp .medidor.env.dist .medidor.env` 
2. `docker-compose up -d` 
3. `docker-compose exec app /bin/bash`
4. `docker-compose exec app /bin/bash`

How to use:

1. Set up connection to the database and SMTP server (see `.medidor.env.dist`)
2. Create table `medidor_metrics` table (id: int, name: varchar, query: varchar) with metrics you want to see

`docker-compose.yml` provided with the project should only be used in development!
