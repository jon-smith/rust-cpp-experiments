use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono;
use sqlx::types::Json;
use std::env;
use tokio::runtime::Runtime;

#[cxx::bridge]
mod ffi {

    pub struct RowData {
        pub id: i32,
        pub info: String,
        pub data: String,
        pub time: String,
    }

    extern "Rust" {
        fn read_all_rows_ffi() -> Result<Vec<RowData>>;
    }
}

fn to_ffi_struct(row_data: RowData) -> ffi::RowData {
    ffi::RowData {
        id: row_data.id,
        info: row_data.info,
        data: row_data.data.to_string(),
        time: row_data
            .time
            .map(|t| t.to_string())
            .unwrap_or(String::from("")),
    }
}

pub struct RowData {
    pub id: i32,
    pub info: String,
    pub data: Json<serde_json::Value>,
    pub time: Option<chrono::NaiveDateTime>,
}

pub async fn read_all_rows_async() -> Result<Vec<RowData>, sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let rows = sqlx::query_as!(RowData, "SELECT * FROM datatable")
        .fetch_all(&pool)
        .await?;

    Ok(rows)
}

pub fn read_all_rows() -> Result<Vec<RowData>, sqlx::Error> {
    Runtime::new().unwrap().block_on(read_all_rows_async())
}

fn read_all_rows_ffi() -> Result<Vec<ffi::RowData>, sqlx::Error> {
    read_all_rows().map(|rows| rows.into_iter().map(to_ffi_struct).collect())
}
