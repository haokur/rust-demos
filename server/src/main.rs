use crate::services::config::CONFIG;

mod handlers;
mod helpers;
mod macros;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    services::logger::init_logger();

    let server_host = &*CONFIG.server.host;
    let server_port = CONFIG.server.port;
    let server_url = format!("{}:{}", server_host, server_port);
    let listener = tokio::net::TcpListener::bind(server_url).await.unwrap();

    helpers::mysql_helper::init()
        .await
        .expect("mysql init failed");

    helpers::redis_helper::init()
        .await
        .expect("redis init failed");

    let app = routes::app();
    axum::serve(listener, app).await.unwrap();
}
