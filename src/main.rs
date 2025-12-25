mod config;
mod sender;
mod server;
mod cli;
mod k8s_gen;

use clap::Parser;
use crate::cli::{Cli, Commands};
use crate::config::{DeploymentConfig, TrafficConfig};
use crate::sender::create_sender;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();

    let cli = Cli::parse();
    
    match cli.command {
        Commands::Server => {
            // Server mode might need a different config or just the first one
            tracing::info!("Trigger 'server' command");
            let traffic_content = std::fs::read_to_string("config/traffic.yaml").unwrap_or_default();
            let traffic = TrafficConfig::from_yaml(&traffic_content)?;
            let sender = create_sender(&traffic.tasks[0]);
            server::run_server(sender).await?;
        }
        // Commands::Send { data, task_index } => {
        //     let traffic_content = std::fs::read_to_string("traffic.yaml").unwrap_or_default();
        //     let traffic = TrafficConfig::from_yaml(&traffic_content)?;
        //     let sender = create_sender(&traffic.tasks[task_index]);
        //     let json_data: serde_json::Value = serde_json::from_str(&data)
        //         .unwrap_or(serde_json::json!({ "message": data }));
        //     sender.send(json_data).await?;
        // }
        Commands::Worker { task_index, config: config_path } => {
            let traffic_content = std::fs::read_to_string(config_path)?;
            let traffic = TrafficConfig::from_yaml(&traffic_content)?;
            let task = &traffic.tasks[task_index];
            let sender = create_sender(task);
            sender::run_periodic(sender, task.frequency, task.duration.clone()).await?;
        }
        Commands::Init { config: config_path, output } => {
            let dep_content = std::fs::read_to_string(&config_path)?;
            let dep_config = DeploymentConfig::from_yaml(&dep_content)?;
            
            // Assume traffic.yaml exists in same dir or default
            let traffic_content = std::fs::read_to_string("config/traffic.yaml").unwrap_or_default();
            let traffic_config = TrafficConfig::from_yaml(&traffic_content)?;

            k8s_gen::clear_namifest(&output)?;
            k8s_gen::generate_manifests(&dep_config, &traffic_config, &output)?;
            tracing::info!("K8s manifests generated in {}", output);
        }
        Commands::Serve { input } => {
            tracing::info!("Serve application {}...", input);
            let status = std::process::Command::new("kubectl")
                .args(["apply", "-f", &format!("{}/configmap.yaml", input)])
                .status()?;
            
            if !status.success() {
                anyhow::bail!("Failed to apply configmap.yaml");
            }

            // Then apply all other manifests in the directory
            let status = std::process::Command::new("kubectl")
                .args(["apply", "-f", &input])
                .status()?;

            if status.success() {
                tracing::info!("Successfully triggered application to k8s.");
            } else {
                anyhow::bail!("Failed to apply manifests from {}", input);
            }
        }
        Commands::Stop { input } => {
            tracing::info!("Stopping application in k8s {}...", input);
            
            let status = std::process::Command::new("kubectl")
                .args(["delete", "-f", &input])
                .status()?;

            if status.success() {
                tracing::info!("Successfully stopped application in k8s.");
            } else {
                anyhow::bail!("Failed to delete resources from {}", input);
            }
        }
    }

    Ok(())
}
