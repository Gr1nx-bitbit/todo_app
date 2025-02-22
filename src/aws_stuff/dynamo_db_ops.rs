use aws_sdk_dynamodb::Error;

pub async fn list_tables() -> Result<(), Error> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .load()
        .await;

    let client_config = aws_sdk_dynamodb::config::Builder::from(&config)
        .endpoint_url("http://localhost:4566")
        .build();
    let client = aws_sdk_dynamodb::Client::from_conf(client_config);

    let request = client.list_tables();
    let response = request.send().await;
    match response {
        Ok(resp) => {
            println!("Table names: {:?}", resp.table_names())
        }
        Err(err) => eprintln!("Err {err:?}"),
    }

    Ok(())
}