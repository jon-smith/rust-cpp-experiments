#[tokio::main]
async fn main() -> Result<(), sqlx_lib::Error> {
    println!("Testing db access...");

    sqlx_lib::read_all_rows_async().await?;

    Ok(())
}
