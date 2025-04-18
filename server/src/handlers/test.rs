use crate::helpers::mysql_helper;
use crate::helpers::redis_helper;
use crate::kafka::consumer::KafkaConsumer;
use crate::kafka::producer::KafkaProducer;
use crate::models::user::User;
use axum::Json;
use axum::extract::{Path, Query};
use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use std::collections::HashMap;
use tracing::info;

#[derive(Serialize, Deserialize)]
struct MyResponse {
    message: String,
}

pub async fn root() -> &'static str {
    "Hello,This Server Root!"
}

pub async fn get_users() -> String {
    let pool = mysql_helper::instance();
    let rows: Vec<User> = query_as!(User, "select * from user order by id")
        .fetch_all(pool)
        .await
        .unwrap();
    let names = rows
        .into_iter()
        .map(|item| String::from_utf8(item.username.unwrap()).unwrap())
        .collect::<Vec<String>>()
        .join(";");

    names
}

// http://localhost:3000/get_my_redis_key
pub async fn get_my_redis_key() -> String {
    let result: String = redis_helper::get("my_key").await.unwrap();
    result
}

// http://localhost:3000/set_my_redis_key?value=1123
pub async fn set_my_redis_key(Query(params): Query<HashMap<String, String>>) -> String {
    let value = params.get("value").unwrap();

    redis_helper::set("my_test_key", value)
        .await
        .expect("TODO: panic message");

    let result: String = redis_helper::get("my_test_key").await.unwrap();

    result.to_string()
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

// 生产kafka消息
pub async fn producer_kafka_message() -> String {
    let producer = KafkaProducer::new("localhost:9092");
    let message_content = format!(
        "hello from rust time is {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    producer
        .send("demo-topic", "my_key", message_content.as_ref())
        .await;

    info!("send kafka message:{}", message_content);

    message_content
}

// 消费kafka消息
// 这里的consumer内部是一个while循环，即开启后，就一直监听新的消息了（在timeout时间内）
pub async fn consumer_kafka_message() {
    let consumer = KafkaConsumer::new("localhost:9092", "my_group", &["demo-topic"]);
    consumer
        .run(move |key, value| info!("receive kafka message,key:{},value:{}", key, value))
        .await;
}

pub async fn put_some() -> String {
    "can't do put".to_string()
}
