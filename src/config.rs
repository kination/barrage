use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeploymentConfig {
    pub instance: i32,
    pub cpu: String,
    pub mem: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TrafficConfig {
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
    pub frequency: u64,
    pub duration: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskType {
    Http,
    Kafka,
}

impl DeploymentConfig {
    pub fn from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }
}

impl TrafficConfig {
    pub fn from_yaml(content: &str) -> Result<Self, serde_yaml::Error> {
        let tasks: Vec<TaskConfig> = serde_yaml::from_str(content)?;
        Ok(Self { tasks })
    }
}
