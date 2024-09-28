#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Testing db access...");

    sqlx_lib::test_db().await?;

    Ok(())
}
