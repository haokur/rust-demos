use axum::Json;
use axum::extract::{Path, Query};
use std::collections::HashMap;

pub async fn root() -> &'static str {
    "Hello,This Server Root!"
}

// http://localhost:3000/path/123456
pub async fn get_path(Path(user_id): Path<u32>) -> String {
    format!("user_id is {}", user_id)
}

// http://localhost:3000/query?username=haokur&job=web
pub async fn query(Query(params): Query<HashMap<String, String>>) -> String {
    let username = params.get("username").unwrap();
    let job = params.get("job").unwrap();
    let result = format!("username is {},job is {}", username, job);

    result
}

// [POST] http://localhost:3000/post_data
pub async fn post_data(Json(payload): Json<serde_json::Value>) -> String {
    let name = payload.get("name").unwrap().as_str().unwrap();
    let result = format!("Payload: {:?},name is {}", payload, name);

    result
}
