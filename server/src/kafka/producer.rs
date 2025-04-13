use chrono::Local;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt::format;

// 测试生产消息发布给kafka
#[tokio::test]
async fn test_kafka_producer() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    // 获取当前时间
    let message_content = format!(
        "hello from rust time is {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    let record = FutureRecord::to("demo-topic")
        .payload(&message_content)
        .key("key");

    match producer.send(record, Duration::from_secs(0)).await {
        Ok((partition, offset)) => {
            println!(
                "Message delivered to partition {} at offset {},message content is {}",
                partition, offset, message_content
            );
        }
        Err((e, _)) => {
            eprintln!("Failed to deliver message: {}", e);
        }
    }
}

// 测试使用封装的producer生产消息
#[tokio::test]
async fn test_producer_send() {
    let producer = KafkaProducer::new("localhost:9092");
    let message_content = format!(
        "hello from rust time is {}",
        Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    producer
        .send("demo-topic", "my_key", message_content.as_ref())
        .await;
}

#[derive(Clone)]
pub struct KafkaProducer {
    inner: FutureProducer,
}

impl KafkaProducer {
    pub fn new(brokers: &str) -> Self {
        let producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()
            .expect("Producer creation error");

        KafkaProducer { inner: producer }
    }

    pub async fn send(&self, topic: &str, key: &str, payload: &str) {
        match self
            .inner
            .send(
                FutureRecord::to(topic).key(key).payload(payload),
                Duration::from_secs(0),
            )
            .await
        {
            Ok(delivery) => {
                info!("Message delivered:{:?}", delivery);
            }
            Err((err, _)) => {
                info!("Kafka error: {}", err);
            }
        }
    }
}
