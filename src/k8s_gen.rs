use crate::config::{DeploymentConfig, TrafficConfig};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{
    ConfigMap, Container, PodSpec, PodTemplateSpec, Volume, VolumeMount, ConfigMapVolumeSource,
    ResourceRequirements,
};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};

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
        
        // Define labels
        let mut labels = BTreeMap::new();
        labels.insert("app".to_string(), "barrage".to_string());
        labels.insert("task".to_string(), name.clone());

        // Define annotations
        let mut annotations = BTreeMap::new();
        annotations.insert("barrage.io/restartedAt".to_string(), chrono::Utc::now().to_rfc3339());

        // Define resource requirements
        let mut resource_limits = BTreeMap::new();
        resource_limits.insert("cpu".to_string(), Quantity(dep_config.cpu.clone()));
        resource_limits.insert("memory".to_string(), Quantity(dep_config.mem.clone()));

        let deployment = Deployment {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                labels: Some(labels.clone()),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(dep_config.instance),
                selector: LabelSelector {
                    match_labels: Some(labels.clone()),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(labels.clone()),
                        annotations: Some(annotations),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![Container {
                            name: "barrage".to_string(),
                            image: Some("barrage:latest".to_string()),
                            image_pull_policy: Some("Always".to_string()),
                            command: Some(vec!["barrage".to_string()]),
                            args: Some(vec![
                                "worker".to_string(),
                                "--task-index".to_string(),
                                i.to_string(),
                                "--config".to_string(),
                                "/etc/barrage/traffic.yaml".to_string(),
                            ]),
                            resources: Some(ResourceRequirements {
                                limits: Some(resource_limits.clone()),
                                requests: Some(resource_limits),
                                ..Default::default()
                            }),
                            volume_mounts: Some(vec![VolumeMount {
                                name: "config-volume".to_string(),
                                mount_path: "/etc/barrage".to_string(),
                                ..Default::default()
                            }]),
                            ..Default::default()
                        }],
                        volumes: Some(vec![Volume {
                            name: "config-volume".to_string(),
                            config_map: Some(ConfigMapVolumeSource {
                                name: "barrage-config".to_string(),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&deployment)?;
        let file_path = output_path.join(format!("{}.yaml", name));
        fs::write(file_path, yaml)?;
        tracing::info!("Generated manifest for task {}: {}", i, name);
    }

    // Generate ConfigMap for traffic.yaml
    let mut config_data = BTreeMap::new();
    config_data.insert("traffic.yaml".to_string(), serde_yaml::to_string(&traffic_config.tasks)?);

    let config_map = ConfigMap {
        metadata: ObjectMeta {
            name: Some("barrage-config".to_string()),
            ..Default::default()
        },
        data: Some(config_data),
        ..Default::default()
    };

    let cm_yaml = serde_yaml::to_string(&config_map)?;
    fs::write(output_path.join("configmap.yaml"), cm_yaml)?;

    Ok(())
}
