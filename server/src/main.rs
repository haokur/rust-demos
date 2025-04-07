use crate::bootstrap::init_db_pool;
use crate::services::config::CONFIG;

mod bootstrap;
mod handlers;
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

    init_db_pool().await.unwrap();

    let app = routes::app();
    axum::serve(listener, app).await.unwrap();
}
