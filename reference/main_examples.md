<span style="font-family:'Amiri'">

Just a place for me to store different main.rs programs and examples from this project.
# Main Examples
```
pub mod aws_stuff;

#[tokio::main]
async fn main() -> Result<(), aws_sdk_dynamodb::Error> {
    let _test = aws_stuff::dynamo_db_ops::list_tables().await;
    Ok(())
}
```

</span>