use std::{
    io::{Error, ErrorKind, Result},
    time::Duration,
};

use bb8_redis_cluster::{bb8::Pool, RedisConnectionManager};
use redis::AsyncCommands;

#[tokio::main]
async fn main() -> Result<()> {
    println!("starting up!");
    let manager = RedisConnectionManager::new(vec!["redis://localhost:7000"]).unwrap();
    let conn_pool = Pool::builder()
        .max_size(15)
        .connection_timeout(Duration::from_secs(1))
        .build(manager)
        .await
        .map_err(|e| Error::new(ErrorKind::NotConnected, e))?;

    let mut conn = conn_pool
        .get()
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
