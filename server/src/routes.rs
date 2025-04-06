use crate::handlers::test::{get_path, post_data, query, root};
use axum::Router;
use axum::routing::{get, post};
use tower_http::trace::TraceLayer;

pub fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/path/{user_id}", get(get_path))
        .route("/query", get(query))
        .route("/post_data", post(post_data))
        .layer(TraceLayer::new_for_http())
}
