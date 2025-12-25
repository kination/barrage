use crate::config::{DeploymentConfig, TrafficConfig};
use std::fs;
use std::path::Path;


pub fn clear_namifest(output_dir: &str) -> anyhow::Result<()> {
    let output_path = Path::new(output_dir);
    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
    }
    Ok(())
}

pub fn generate_manifests(
    dep_config: &DeploymentConfig, 
    traffic_config: &TrafficConfig, 
    output_dir: &str
) -> anyhow::Result<()> {
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }

    for (i, task) in traffic_config.tasks.iter().enumerate() {
        let name = task.name.clone().unwrap_or_else(|| format!("barrage-task-{}", i));
        let manifest = format!(
r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: {name}
  labels:
    app: barrage
    task: {name}
spec:
  replicas: {instances}
  selector:
    matchLabels:
      app: barrage
      task: {name}
  template:
    metadata:
      annotations:
        barrage.io/restartedAt: "{now}"
      labels:
        app: barrage
        task: {name}
    spec:
      containers:
      - name: barrage
        image: barrage:latest
        imagePullPolicy: Always
        command: ["barrage"]
        args: ["worker", "--task-index", "{i}", "--config", "/etc/barrage/traffic.yaml"]
        resources:
          limits:
            cpu: "{cpu}"
            memory: "{mem}"
          requests:
            cpu: "{cpu}"
            memory: "{mem}"
        volumeMounts:
        - name: config-volume
          mountPath: /etc/barrage
      volumes:
      - name: config-volume
        configMap:
          name: barrage-config
---
"#, 
        name = name, 
        instances = dep_config.instance,
        cpu = dep_config.cpu,
        mem = dep_config.mem,
        i = i,
        now = chrono::Utc::now().to_rfc3339()
        );
        let file_path = output_path.join(format!("{}.yaml", name));
        fs::write(file_path, manifest)?;
        tracing::info!("Generated manifest for task {}: {}", i, name);
    }

    // Generate a ConfigMap for traffic.yaml
    let traffic_content = serde_yaml::to_string(&traffic_config.tasks)?;
    let config_map = format!(
r#"apiVersion: v1
kind: ConfigMap
metadata:
  name: barrage-config
data:
  traffic.yaml: |
{}"#, indent(&traffic_content, 4));
    
    fs::write(output_path.join("configmap.yaml"), config_map)?;

    Ok(())
}

fn indent(s: &str, n: usize) -> String {
    let prefix = " ".repeat(n);
    s.lines()
        .map(|line| format!("{}{}", prefix, line))
        .collect::<Vec<_>>()
        .join("\n")
}
