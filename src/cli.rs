use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server,
    // Send {
    //     #[arg(short, long)]
    //     data: String,
    //     #[arg(short, long, default_value_t = 0)]
    //     task_index: usize,
    // },
    Worker {
        #[arg(short, long, default_value_t = 0)]
        task_index: usize,
        #[arg(short, long, default_value = "config/traffic.yaml")]
        config: String,
    },
    /// Generate K8s manifests for all tasks in config
    Init {
        #[arg(short, long, default_value = "config/dep.yaml")]
        config: String,
        #[arg(short, long, default_value = "k8s/generated")]
        output: String,
    },
    /// Trigger application to k8s environment with defined config
    Serve {
        #[arg(short, long, default_value = "k8s/generated")]
        input: String,
    },
    /// Stop k8s environment (Delete resources)
    Stop {
        #[arg(short, long, default_value = "k8s/generated")]
        input: String,
    },
}
