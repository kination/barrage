use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the server to receive triggers (Manual)
    Server,
    /// Manually trigger a message send via CLI
    Send {
        #[arg(short, long)]
        data: String,
        #[arg(short, long, default_value_t = 0)]
        task_index: usize,
    },
    /// Start as a periodic worker for a specific task
    Worker {
        #[arg(short, long, default_value_t = 0)]
        task_index: usize,
        #[arg(short, long, default_value = "config.yaml")]
        config: String,
    },
    /// Generate K8s manifests for all tasks in config
    Deploy {
        #[arg(short, long, default_value = "config.yaml")]
        config: String,
        #[arg(short, long, default_value = "k8s/generated")]
        output: String,
    },
}
