use async_trait::async_trait;
use anyhow::Result;
use crate::sender::Sender;
use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;

pub struct KafkaSender {
    producer: FutureProducer,
    topic: String,
}

impl KafkaSender {
    pub async fn new(broker: String, topic: String) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &broker)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self { producer, topic })
    }
}

#[async_trait]
impl Sender for KafkaSender {
    async fn send(&self, data: serde_json::Value) -> Result<()> {
        let payload = serde_json::to_string(&data)?;
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key("");

        self.producer
            .send(record, Duration::from_secs(0))
            .await
            .map_err(|(e, _)| anyhow::anyhow!("Kafka send error: {}", e))?;
        
        tracing::info!("Kafka: Successfully sent data to topic {}", self.topic);
        Ok(())
    }
}
