mod config;
mod sender;
mod server;
mod cli;
mod k8s_gen;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::config::AppConfig;
use crate::sender::create_sender;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    match cli.command {
        Commands::Server => {
            // Server mode might need a different config or just the first one
            let config_content = std::fs::read_to_string("config.yaml").unwrap_or_default();
            let config = AppConfig::from_yaml(&config_content)?;
            let sender = create_sender(&config.tasks[0]);
            server::run_server(sender).await?;
        }
        Commands::Send { data, task_index } => {
            let config_content = std::fs::read_to_string("config.yaml").unwrap_or_default();
            let config = AppConfig::from_yaml(&config_content)?;
            let sender = create_sender(&config.tasks[task_index]);
            let json_data: serde_json::Value = serde_json::from_str(&data)
                .unwrap_or(serde_json::json!({ "message": data }));
            sender.send(json_data).await?;
        }
        Commands::Worker { task_index, config: config_path } => {
            let config_content = std::fs::read_to_string(config_path)?;
            let config = AppConfig::from_yaml(&config_content)?;
            let task = &config.tasks[task_index];
            let sender = create_sender(task);
            sender::run_periodic(sender, task.frequency).await?;
    }

    Ok(())
}
