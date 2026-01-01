use async_trait::async_trait;
use anyhow::Result;
use crate::sender::Sender;

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
