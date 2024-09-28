# Rust/C++ PostgreSQL

The aim of this project is to have compile-time checked SQL statements for a PostgreSQL database, with resulting structures available to both Rust and C++.

We use the crate [`sqlx`](https://github.com/launchbadge/sqlx) for creating the database and checking queries at compile time. The resulting structures are converted and exposed to C++ using [`cxx`](https://github.com/dtolnay/cxx).

Currently the main limitation is that we define the `RowData` struct twice: once with data types matching the database, and once with types suitable to pass to C++ using `cxx`. Here we represent both json and nullable date-time as strings.

## Setup

1. Install the `sqxl` cli:

```sh
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

2. Run Postgres in the background using docker compose:

```sh
docker-compose up -d
```

3. Create the database:

```sh
sqlx db create
```

4. Run sql migrations

```sh
(cd sqlx_lib && sqlx migrate run)
```

## Build/Run

### Rust

`cargo build` to build the lib and exe, `cargo run` to run the exe.

### C++

todo
