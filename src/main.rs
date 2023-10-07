use std::io::{Error, ErrorKind, Result};

use redis::{cluster::ClusterClient, AsyncCommands};

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting up!");
    let nodes = vec!["redis://127.0.0.1:7000"];
    let client = ClusterClient::new(nodes).map_err(|e| Error::new(ErrorKind::Other, e))?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|e| Error::new(ErrorKind::NotConnected, e))?;

    println!("connection establish - test set and get methods next");

    conn.set("test", "test_data")
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    let result: String = conn
        .get("test")
        .await
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    println!("retrieved result from redis: {result}");

    Ok(())
}
