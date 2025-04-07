use crate::models::user::User;
use crate::services::config::CONFIG;
use sqlx::{Pool, query_as};

pub async fn connect() -> Result<(), sqlx::Error> {
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

    let rows: Vec<User> = query_as!(User, "SELECT * FROM user")
        .fetch_all(&pool)
        .await
        .expect("Error getting users");

    for user in rows {
        println!(
            "User ID: {},user name is {:?}",
            user.id,
            String::from_utf8(user.username.unwrap()).unwrap()
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_connect() {
    let _ = connect().await.unwrap();
}
