use crate::services::config::CONFIG;
use sqlx::Pool;
use std::sync::OnceLock;

static DB_POOL: OnceLock<Pool<sqlx::MySql>> = OnceLock::new();

pub async fn init_db_pool() -> Result<(), sqlx::Error> {
    let host = &*CONFIG.database.mysql_host;
    let port = &*CONFIG.database.mysql_port;
    let user = &*CONFIG.database.mysql_user;
    let password = &*CONFIG.database.mysql_password;
    let dbname = &*CONFIG.database.mysql_database;

    let url = format!(
        "mysql://{}:{}@{}:{}/{}?charset=utf8mb4",
        user, password, host, port, dbname
    );

    let pool: Pool<sqlx::MySql> = Pool::connect(&url)
        .await
        .expect("failed to connect to mysql database");

    DB_POOL.set(pool).expect("failed to init mysql database");

    Ok(())
}

pub fn get_pool() -> &'static Pool<sqlx::MySql> {
    DB_POOL.get().expect("failed to get mysql pool")
}
