use async_trait::async_trait;
use anyhow::Result;
use crate::config::{TaskConfig, TaskType};
use std::time::Duration;

#[async_trait]
pub trait Sender: Send + Sync {
    async fn send(&self, data: serde_json::Value) -> Result<()>;
}

pub struct HttpSender {
    client: reqwest::Client,
    url: String,
}

impl HttpSender {
    pub fn new(host: String, path: Option<String>) -> Self {
        let url = match path {
            Some(p) => format!("{}/{}", host.trim_end_matches('/'), p.trim_start_matches('/')),
            None => host,
        };
        Self {
            client: reqwest::Client::new(),
            url,
        }
    }
}

#[async_trait]
impl Sender for HttpSender {
    async fn send(&self, data: serde_json::Value) -> Result<()> {
        let res = self.client.post(&self.url)
            .json(&data)
            .send()
            .await?;
        
        if res.status().is_success() {
            println!("HTTP: Successfully sent data to {}", self.url);
            Ok(())
        } else {
            Err(anyhow::anyhow!("HTTP: Failed to send data, status: {}", res.status()))
        }
    }
}

pub struct KafkaSender {
    broker: String,
    topic: String,
}

impl KafkaSender {
    pub fn new(broker: String, topic: String) -> Self {
        Self { broker, topic }
    }
}

#[async_trait]
impl Sender for KafkaSender {
    async fn send(&self, data: serde_json::Value) -> Result<()> {
        println!("Kafka: Sending data to broker {} topic {}: {:?}", self.broker, self.topic, data);
        Ok(())
    }
}

pub fn create_sender(config: &TaskConfig) -> Box<dyn Sender> {
    match config.task_type {
        TaskType::Http => {
            Box::new(HttpSender::new(config.host.clone(), config.path.clone()))
        },
        TaskType::Kafka => {
            Box::new(KafkaSender::new(
                config.host.clone(),
                config.topic.clone().unwrap_or_default(),
            ))
        }
    }
}

pub async fn run_periodic(sender: Box<dyn Sender>, frequency: u64) -> Result<()> {
    if frequency == 0 {
        return Err(anyhow::anyhow!("Frequency must be greater than 0"));
    }
    
    let interval_ms = (60 * 1000) / frequency;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    
    println!("Starting periodic sender with frequency: {}/min (interval: {}ms)", frequency, interval_ms);

    loop {
        interval.tick().await;
        let data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "message": "periodic trigger"
        });
        if let Err(e) = sender.send(data).await {
            eprintln!("Error sending message: {}", e);
        }
    }
}
