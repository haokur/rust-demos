use crate::grpc::client;
use crate::kafka::consumer::KafkaConsumer;
use crate::kafka::producer::KafkaProducer;
use crate::pb::hello_service_server::HelloService;
use crate::pb::{HelloRequest, HelloResponse};
use crate::services::config::CONFIG;
use std::thread;
use tonic::{Request, Response, Status};
use tracing::info;

mod grpc;
mod handlers;
mod helpers;
mod kafka;
mod macros;
mod models;
mod routes;
mod services;
mod utils;
mod socket;

mod pb {
    tonic::include_proto!("hello");
}

#[allow(dead_code)]
async fn test_kafka() {
    let producer = KafkaProducer::new("localhost:9092");
    producer
        .send("test_topic", "my_key", "hello from rust".as_ref())
        .await;

    let consumer = KafkaConsumer::new("localhost:9092", "my_group", &["test_topic"]);

    consumer
        .run(|key, value| {
            println!("Got key: {:?}", key);
        })
        .await;
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
