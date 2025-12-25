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
        tracing::info!("HTTP: GET from {}", self.url);
        let res = self.client.get(&self.url)
            .json(&data)
            .send()
            .await?;
        
        if res.status().is_success() {
            tracing::info!("HTTP: Successfully sent data to {}", self.url);
            Ok(())
        } else {
            let err_msg = format!("HTTP: Failed to send data, status: {}", res.status());
            tracing::error!("{}", err_msg);
            Err(anyhow::anyhow!(err_msg))
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
        tracing::info!("Kafka: Sending data to broker {} topic {}: {:?}", self.broker, self.topic, data);
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

pub async fn run_periodic(sender: Box<dyn Sender>, frequency: u64, duration_str: String) -> Result<()> {
    if frequency == 0 {
        return Err(anyhow::anyhow!("Frequency must be greater than 0"));
    }
    
    let duration: Duration = duration_str.parse::<humantime::Duration>()?.into();
    let start_time = tokio::time::Instant::now();
    
    let interval_ms = (60 * 1000) / frequency;
    let mut interval = tokio::time::interval(Duration::from_millis(interval_ms));
    
    tracing::info!("Starting sender with frequency: {}/min, duration: {}", frequency, duration_str);

    loop {
        interval.tick().await;
        
        if start_time.elapsed() >= duration {
            tracing::info!("Duration {} reached. Stopping worker.", duration_str);
            break;
        }

        let data = serde_json::json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "message": "periodic trigger"
        });
        if let Err(e) = sender.send(data).await {
            tracing::error!("Error sending message: {}", e);
        }
    }
    Ok(())
}
