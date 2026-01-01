use barrage::config::{DeploymentConfig, TrafficConfig, TaskType};
use barrage::k8s_gen;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_manifest_generation_workflow() {
    let dir = tempdir().unwrap();
    let output_dir = dir.path().to_str().unwrap();

    let dep_config = DeploymentConfig {
        instance: 2,
        cpu: "100m".to_string(),
        mem: "128Mi".to_string(),
    };

    let traffic_yaml = r#"
- name: my-http-task
  type: http
  host: http://example.com
  frequency: 5
  duration: 10s
"#;
    let traffic_config = TrafficConfig::from_yaml(traffic_yaml).expect("Failed to parse traffic config");

    k8s_gen::generate_manifests(&dep_config, &traffic_config, output_dir).expect("Failed to generate manifests");

    // Check if files exist
    assert!(dir.path().join("my-http-task.yaml").exists());
    assert!(dir.path().join("configmap.yaml").exists());

    // Check content of generated deployment
    let deployment_content = fs::read_to_string(dir.path().join("my-http-task.yaml")).unwrap();
    assert!(deployment_content.contains("name: my-http-task"));
    assert!(deployment_content.contains("replicas: 2"));
    assert!(deployment_content.contains("cpu: 100m"));
    assert!(deployment_content.contains("memory: 128Mi"));
}

#[tokio::test]
async fn test_create_sender_logic() {
    use barrage::config::{TaskConfig, TaskType};
    use barrage::sender::create_sender;

    let config = TaskConfig {
        name: Some("test".to_string()),
        task_type: TaskType::Http,
        host: "http://localhost".to_string(),
        path: Some("/test".to_string()),
        topic: None,
        frequency: 1,
        duration: "1s".to_string(),
    };

    let sender = create_sender(&config).await.expect("Failed to create sender");
    drop(sender);
}
