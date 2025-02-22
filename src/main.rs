pub mod types;
pub mod aws_stuff;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_dynamodb::Error> {
    let _test = aws_stuff::dynamo_db_ops::list_tables().await;
    Ok(())
}
