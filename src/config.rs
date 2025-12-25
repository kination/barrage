use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub tasks: Vec<TaskConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskConfig {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub task_type: TaskType,
    pub host: String,
    pub path: Option<String>,
    pub topic: Option<String>,
    pub frequency: u64, // times per minute
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Http,
    Kafka,
}

impl AppConfig {
    pub fn from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        let tasks: Vec<TaskConfig> = serde_yaml::from_str(content)?;
        Ok(Self { tasks })
    }
}
