use rdkafka::Message;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};

pub struct KafkaConsumer {
    inner: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(brokers: &str, group_id: &str, topics: &[&str]) -> Self {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("enable.partition.eof", "false")
            .set("session.timeout.ms", "6000")
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Consumer creation failed");

        consumer
            .subscribe(topics)
            .expect("Can't subscribe to specified topics");

        KafkaConsumer { inner: consumer }
    }

    pub async fn run<F>(&self, mut handler: F)
    where
        F: FnMut(String, String) + Send + 'static,
    {
        use tokio_stream::StreamExt;

        let mut message_stream = self.inner.stream();

        while let Some(result) = message_stream.next().await {
            match result {
                Ok(m) => {
                    let key = m
                        .key()
                        .map(|k| String::from_utf8_lossy(k).to_string())
                        .unwrap_or_default();
                    let payload = m
                        .payload()
                        .map(|p| String::from_utf8_lossy(p).to_string())
                        .unwrap_or_default();

                    handler(key, payload);
                    self.inner.commit_message(&m, CommitMode::Async).unwrap();
                }
                Err(e) => {
                    println!("Kafka error: {}", e);
                }
            }
        }
    }
}
