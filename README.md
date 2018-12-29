# JSONAPI with Rust

The goal of this project is to implement a [JSON:API](https://jsonapi.org/) API in Rust.

`JSON:API` is a specification that defines a standard JSON structure returned by RESTful APIs.

This project uses [michiel/jsonapi-rust](https://github.com/michiel/jsonapi-rust) as a third-party library.

---
## Running the project.

### Prerequisites 

- Cargo 1.31.*
- PostgreSQL

## Steps

1. Create a database in PostgreSQL named `imdb`

```
$: psql -U postgres
$: CREATE DATABASE imdb
$: \q
```

2. Populate the imdb database using the provided `tar` archive. (Kudos to [raosaif/sample_postgresql_database](https://github.com/raosaif/sample_postgresql_database) )

```
pg_restore -U postgres -n public -d databasename '/path/to/imdb.tar' 
```

> **HINT**: You can use [psequal](http://www.psequel.com/) to visualise the database

3. Run the application providing the database connecting string as an environment variable

```
CONN_STRING=http://<username>:<password>@localhost:5432/imdb cargo run
```