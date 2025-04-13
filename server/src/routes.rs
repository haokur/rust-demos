use crate::handlers::test::{
    consumer_kafka_message, get_my_redis_key, get_path, get_users, post_data,
    producer_kafka_message, put_some, query, root, set_my_redis_key,
};
use crate::socket::socket::handle_socket;
use axum::Router;
use axum::http::HeaderName;
use axum::routing::{get, post, put};
use reqwest::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    let allow_methods = vec![Method::GET, Method::POST, Method::OPTIONS];
    let allow_headers: Vec<HeaderName> = vec![
        HeaderName::from_static("content-type"),
        HeaderName::from_static("x-token"),
    ];

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许任意 Origin,或一个http域名数组
        .allow_methods(allow_methods)
        .allow_headers(allow_headers);

    Router::new()
        .route("/ws", get(handle_socket))
        .route("/producer_kafka_message", get(producer_kafka_message))
        .route("/consumer_kafka_message", get(consumer_kafka_message))
        .route("/", get(root))
        .route("/path/{user_id}", get(get_path))
        .route("/query", get(query))
        .route("/post_data", post(post_data))
        .route("/put_some", put(put_some))
        .route("/get_users", get(get_users))
        .route("/get_my_redis_key", get(get_my_redis_key))
        .route("/set_my_redis_key", get(set_my_redis_key))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
