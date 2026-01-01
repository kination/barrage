use async_trait::async_trait;
use anyhow::Result;
use crate::config::{TaskConfig, TaskType};
use std::time::Duration;
use crate::req_type::{HttpSender, KafkaSender};

#[async_trait]
pub trait Sender: Send + Sync {
    async fn send(&self, data: serde_json::Value) -> Result<()>;
}

pub async fn create_sender(config: &TaskConfig) -> Result<Box<dyn Sender>> {
    match config.task_type {
        TaskType::Http => {
            Ok(Box::new(HttpSender::new(config.host.clone(), config.path.clone())))
        },
        TaskType::Kafka => {
            let sender = KafkaSender::new(
                config.host.clone(),
                config.topic.clone().unwrap_or_default(),
            ).await?;
            Ok(Box::new(sender))
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
