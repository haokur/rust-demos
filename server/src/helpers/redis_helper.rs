use crate::services::config::CONFIG;
use redis::AsyncCommands;
use redis::aio::MultiplexedConnection;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;

static REDIS_POOL: OnceLock<Arc<Mutex<MultiplexedConnection>>> = OnceLock::new();

pub async fn init() -> Result<(), redis::RedisError> {
    let redis_url = &*CONFIG.redis.url;
    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let con = client.get_multiplexed_async_connection().await?;
    let arc_con = Arc::new(Mutex::new(con));
    REDIS_POOL.set(arc_con).ok();

    Ok(())
}

pub async fn get<T: redis::FromRedisValue>(key: &str) -> Result<T, redis::RedisError> {
    let arc_con = REDIS_POOL.get().unwrap().clone();
    let pool = arc_con.lock().await;
    let value: T = pool.clone().get(key).await?;

    Ok(value)
}

pub async fn set<T: redis::ToRedisArgs + Send + Sync>(
    key: &str,
    value: T,
) -> Result<(), redis::RedisError> {
    let arc_con = REDIS_POOL.get().unwrap().clone();
    let pool = arc_con.lock().await;

    let _: () = pool.clone().set(key, value).await?;

    Ok(())
}

pub async fn delete(key: &str) -> Result<(), redis::RedisError> {
    let arc_con = REDIS_POOL.get().unwrap().clone();
    let pool = arc_con.lock().await;
    let _: () = pool.clone().del(key).await?;

    Ok(())
}

#[tokio::test]
async fn test_get() {
    init().await.expect("TODO: panic message");
    let result: String = get("my_key").await.unwrap();
    println!("my_key is {}", result);
}

#[tokio::test]
async fn test_set() {
    init().await.unwrap();
    set("my_key", "my_value2").await.unwrap();
    let result: String = get("my_key").await.unwrap();
    println!("my_key is {}", result);
}

#[tokio::test]
async fn test_delete() {
    init().await.unwrap();
    delete("my_test_key").await.unwrap();
}
