use crate::services::config::CONFIG;
use redis::aio::MultiplexedConnection;
use std::sync::{Mutex, OnceLock};

static REDIS_POOL: OnceLock<Mutex<MultiplexedConnection>> = OnceLock::new();

pub async fn init() -> Result<(), redis::RedisError> {
    let redis_url = &*CONFIG.redis.url;
    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let con = client.get_multiplexed_async_connection().await?;
    REDIS_POOL.set(Mutex::new(con)).ok();

    Ok(())
}

pub fn instance() -> MultiplexedConnection {
    let pool = REDIS_POOL.get().unwrap().lock().unwrap().clone();
    pool
}
