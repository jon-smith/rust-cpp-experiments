# Rust PostgreSQL

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

3. Run sql migrations

```sh
(cd sqlx_lib && sqlx migrate run)
```
