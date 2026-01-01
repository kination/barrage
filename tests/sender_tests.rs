use barrage::sender::{Sender, run_periodic};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{pause, advance, Duration};

struct MockSender {
    sent_count: Arc<Mutex<usize>>,
}

#[async_trait]
impl Sender for MockSender {
    async fn send(&self, _data: serde_json::Value) -> anyhow::Result<()> {
        let mut count = self.sent_count.lock().await;
        *count += 1;
        Ok(())
    }
}

#[tokio::test]
async fn test_run_periodic_stops_after_duration() {
    pause();
    let sent_count = Arc::new(Mutex::new(0));
    let sender = Box::new(MockSender {
        sent_count: Arc::clone(&sent_count),
    });

    // Run for 10 seconds, frequency 60/min (1 per sec)
    let handle = tokio::spawn(async move {
        run_periodic(sender, 60, "10s".to_string()).await.unwrap();
    });

    // Advance 11 seconds to be safe
    advance(Duration::from_secs(11)).await;
    
    handle.await.unwrap();

    let final_count = *sent_count.lock().await;
    // Depending on how interval works, it might be 10 or 11 ticks.
    // Specifically, interval(1s) ticks immediately, then every 1s.
    // So at 0s, 1s, ..., 9s -> total 10 ticks. 
    // 10s might or might not tick depending on timing.
    assert!(final_count >= 10);
}
