use std::io::{Error, ErrorKind, Result};

use redis::{cluster::ClusterClient, AsyncCommands};

#[tokio::main]
async fn main() -> Result<()> {
    let nodes = vec!["redis://bitnami@localhost:6379/"];
    let client = ClusterClient::new(nodes).map_err(|e| Error::new(ErrorKind::Other, e))?;
    let mut conn = client
        .get_async_connection()
        .await
        .map_err(|e| Error::new(ErrorKind::NotConnected, e))?;

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
