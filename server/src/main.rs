use crate::services::config::CONFIG;

mod grpc;
mod handlers;
mod helpers;
mod kafka;
mod macros;
mod models;
mod routes;
mod services;
mod socket;
mod utils;

mod pb {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() {
    let _guard = helpers::logger_helper::init_logger("server");

    let server_host = &*CONFIG.server.host;
    let server_port = CONFIG.server.port;
    let server_url = format!("{}:{}", server_host, server_port);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    helpers::mysql_helper::init()
        .await
        .expect("mysql init failed");

    helpers::redis_helper::init()
        .await
        .expect("redis init failed");

    let app = routes::app();
    axum::serve(listener, app).await.unwrap();
}
