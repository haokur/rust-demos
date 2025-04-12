use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tracing::info;

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
