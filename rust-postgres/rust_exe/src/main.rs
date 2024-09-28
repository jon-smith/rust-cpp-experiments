#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Testing db access...");

    sqlx_lib::read_all_rows_async().await?;

    Ok(())
}
