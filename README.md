# Barrage (WIP)

**Barrage** is lightweight, high-performance testing tool designed to push various types of data to target API server, Kafka broker, or more.

It is built with Rust(on top of k8s), allowing you to simulate high-frequency data traffic or schedule periodic data synchronization.

## Key Features (TODO)

- Multi-Protocol Support: Push data via HTTP GET/POST or Kafka topics.
- Structured Configuration: Separate deployment specs (`dep.yaml`) from traffic logic (`traffic.yaml`).
- K8s Native: Automatically generates professional Kubernetes manifests with resource limits and independent scaling.
---

## Configuration

Configurations are stored in the `config/` directory.

### 1. `config/dep.yaml` (Deployment Spec)
Defines how the workers should behave in Kubernetes.
```yaml
instance: 2    # Number of replicas per task
cpu: 100m      # CPU limit/request
mem: 128Mi     # Memory limit/request
```

### 2. `config/traffic.yaml` (Traffic Spec)
Defines the actual data tasks to be performed.
```yaml
# For HTTP API traffic
- type: http
  host: https://jsonplaceholder.typicode.com/
  path: posts/1
  frequency: 5    # 5 requests per minute
  duration: 1m    # Stop after 1 minute

# For Kafka producing traffic
- type: kafka
  host: http://localhost:9092
  topic: sample_topic
  frequency: 10
  duration: 1h    # Stop after 1 hour
```

---

## Getting Started

### 1. Manual Local Build
To build the project locally, you need `cmake`, `pkg-config`, and `openssl` installed on your system (for `rdkafka` support).
```bash
# On macOS
brew install cmake pkg-config openssl

cargo build --release
```

### 2. Docker Image Preparation
For Kubernetes deployment, build the container image:
```bash
docker build -t barrage:latest -f docker/Dockerfile .
```

---

## Command Guide

Barrage provides a simple CLI to manage the entire lifecycle:

### Kubernetes Lifecycle

| Command | Description |
| :--- | :--- |
| `cargo run -- init` | Generates K8s manifests in `k8s/generated/` based on configs. |
| `cargo run -- serve` | Applies the generated manifests to your active K8s context. |
| `cargo run -- stop` | Stops the running pods and cleans up K8s resources. |

### Local Testing

| Command | Description |
| :--- | :--- |
| `cargo run -- worker` | Runs a specific task directly on your local machine. |
| `cargo run -- server` | Starts a manual trigger server (Axum) on port 3000. |

---

## Kubernetes Workflow Example

```bash
# 1. Generate manifests (timestamped to force pod restarts)
$ cargo run -- init

# 2. Deploy to cluster (Docker Desktop / MiniKube)
$ cargo run -- serve

# 3. Check logs
$ kubectl logs -l app=barrage -f

# 4. Clean up
$ cargo run -- stop
```

---

## Advanced: Image Updates
If you change the source code and want to see it reflected in K8s:
1. Rebuild the image: `docker build -t barrage:latest -f docker/Dockerfile .`
2. Run `cargo run -- init` (this updates the `restartedAt` timestamp to force a rollout).
3. Run `cargo run -- serve`.
