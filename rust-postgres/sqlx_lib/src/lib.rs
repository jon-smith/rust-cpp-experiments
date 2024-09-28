use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono;
use sqlx::types::Json;
use std::env;

pub struct RowData {
    pub id: i32,
    pub info: String,
    pub data: Json<serde_json::Value>,
    pub time: Option<chrono::NaiveDateTime>,
}

pub async fn test_db() -> Result<Vec<RowData>, sqlx::Error> {
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
