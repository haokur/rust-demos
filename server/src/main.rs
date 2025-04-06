mod handlers;
pub mod routes;
mod services;

#[tokio::main]
async fn main() {
    services::logger::init_logger();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = routes::app();
    axum::serve(listener, app).await.unwrap();
}
