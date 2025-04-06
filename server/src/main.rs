use axum::extract::{Path, Query};
use axum::{Json, Router, routing::get, routing::post};
use std::collections::HashMap;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn root() -> &'static str {
    "Hello,This Server Root!"
}

// http://localhost:3000/path/123456
async fn get_path(Path(user_id): Path<u32>) -> String {
    format!("user_id is {}", user_id)
}

// http://localhost:3000/query?username=haokur&job=web
async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    let username = params.get("username").unwrap();
    let job = params.get("job").unwrap();
    let result = format!("username is {},job is {}", username, job);

    result
}

// [POST] http://localhost:3000/post_data
async fn post_data(Json(payload): Json<serde_json::Value>) -> String {
    let name = payload.get("name").unwrap().as_str().unwrap();
    let result = format!("Payload: {:?},name is {}", payload, name);

    result
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root))
        .route("/path/{user_id}", get(get_path))
        .route("/query", get(query))
        .route("/post_data", post(post_data))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
