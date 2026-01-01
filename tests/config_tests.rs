use barrage::config::{DeploymentConfig, TrafficConfig, TaskType};

#[test]
fn test_deployment_config_deserialization() {
    let yaml = r#"
instance: 3
cpu: 500m
mem: 1Gi
"#;
    let config = DeploymentConfig::from_yaml(yaml).unwrap();
    assert_eq!(config.instance, 3);
    assert_eq!(config.cpu, "500m");
    assert_eq!(config.mem, "1Gi");
}

#[test]
fn test_traffic_config_deserialization() {
    let yaml = r#"
- name: test-http
  type: http
  host: http://localhost:8080
  path: /api/v1
  frequency: 10
  duration: 1m
- name: test-kafka
  type: kafka
  host: localhost:9092
  topic: test-topic
  frequency: 5
  duration: 30s
"#;
    let config = TrafficConfig::from_yaml(yaml).unwrap();
    assert_eq!(config.tasks.len(), 2);
    assert_eq!(config.tasks[0].task_type, TaskType::Http);
    assert_eq!(config.tasks[1].task_type, TaskType::Kafka);
    assert_eq!(config.tasks[1].topic, Some("test-topic".to_string()));
}
