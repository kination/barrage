use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use std::sync::Arc;
use crate::sender::Sender;

struct AppState {
    sender: Box<dyn Sender>,
}

pub async fn run_server(sender: Box<dyn Sender>) -> anyhow::Result<()> {
    let state = Arc::new(AppState { sender });

    let app = Router::new()
        .route("/trigger", post(trigger_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn trigger_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<String, String> {
    match state.sender.send(payload).await {
        Ok(_) => Ok("Message sent successfully".to_string()),
        Err(e) => Err(format!("Failed to send message: {}", e)),
    }
}
