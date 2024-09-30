# Rust/C++ PostgreSQL

The aim of this project is to have compile-time checked SQL statements for a PostgreSQL database, with resulting structures available to both Rust and C++.

We use the crate [`sqlx`](https://github.com/launchbadge/sqlx) for creating the database and checking queries at compile time. The resulting structures are converted and exposed to C++ using [`cxx`](https://github.com/dtolnay/cxx).

We demonstrate calling into the Rust library from C++ and inspecting the rows read from the database in [`cpp_exe/main.cpp`](cpp_exe/main.cpp), which builds using CMake and [Corrosion](https://github.com/AndrewGaspar/corrosion). There is also a Rust executable in [`rust_exe/src/main.rs`](rust_exe/src/main.rs) which adds a single row to the database and then reads the data back out, printing the same output as the C++ executable.

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

If the database structure changes, the query metadata will need to be updated by running `cargo sqlx prepare --workspace`.

## Build/Run

### Rust only

`cargo build` to build the lib and exe, `cargo run` to run the exe (this will clear the database table and add a single row back in).

### C++

```sh
preset=gcc-release
cmake --preset ${preset}
cmake --build out/build/${preset}
out/build/${preset}/RustCxxPostgres
```

If you have previously run the Rust executable and have added no additional data to the database, the C++ executable should print the same output.
