use crate::services::config::CONFIG;
use sqlx::Pool;
use std::sync::OnceLock;

static DB_POOL: OnceLock<Pool<sqlx::MySql>> = OnceLock::new();

pub async fn init() -> Result<(), sqlx::Error> {
    let host = &*CONFIG.database.mysql_host;
    let port = &*CONFIG.database.mysql_port;
    let user = &*CONFIG.database.mysql_user;
    let password = &*CONFIG.database.mysql_password;
    let dbname = &*CONFIG.database.mysql_database;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
        user, password, host, port, dbname
    );

    let pool: Pool<sqlx::MySql> = Pool::connect(&url).await?;

    DB_POOL.set(pool).expect("TODO: panic message");

    Ok(())
}

pub fn instance() -> &'static Pool<sqlx::MySql> {
    DB_POOL.get().expect("failed to get mysql pool")
}
