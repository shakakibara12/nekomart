# NekoMart: Polyglot Microservices E-commerce

<div align="center">

**Highly Scalable • Performant • Cloud-Native**

[![Rust](https://img.shields.io/badge/Rust-Actix--web-orange?logo=rust)](https://actix.rs/)
[![Python](https://img.shields.io/badge/Python-FastAPI-green?logo=python)](https://fastapi.tiangolo.com/)
[![Kubernetes](https://img.shields.io/badge/Kubernetes-EKS-blue?logo=kubernetes)](https://kubernetes.io/)
[![IaC](https://img.shields.io/badge/IaC-Terraform-7B42BC?logo=terraform)](https://www.terraform.io/)

</div>

A simple E-commerce microservice application. This is basically me experimenting with how actual
mainstream large sized websites are made and ran on a large scale, handling huge amounts of requests daily!.

---

## Architecture

The application is split into two distinct core services to demonstrate polyglot capabilities:

1.  **Catalog Service (Python/FastAPI):**
    *   Handles product listings and retrieval (Read-heavy).
    *   Leverages FastAPI for rapid development and async I/O.
    *   **Port:** 8000
2.  **Checkout Service (Rust/Actix-web):**
    *   Handles order processing and state management (Write-heavy).
    *   Leverages Rust for memory safety, zero-cost abstractions, and high concurrency.
    *   **Port:** 8080

## Tech Stack

| Category | Technology |
| :--- | :--- |
| **Languages** | Rust, Python |
| **Frameworks** | Actix-web, FastAPI |
| **Containerization** | Docker, Docker Compose |
| **Orchestration** | Kubernetes (EKS) |
| **Infrastructure** | Terraform (AWS) |
| **GitOps** | ArgoCD |
| **Observability** | Prometheus, Grafana, Loki |

---

## Local Development

### 1. Catalog Service (Python)

Navigate to the catalog directory and install dependencies:

```bash
cd catalog-service
pip install -e
uvicorn main:app --reload
```
Access API Documentation: [http://localhost:8000/docs](http://localhost:8000/docs)

### 2. Checkout Service (Rust)

Navigate to the checkout directory:

```bash
cd checkout-service
cargo run
```
Test the health endpoint:
```bash
curl http://localhost:8080/
```

### 3. Docker (Multi-container)

Build and run both services using Docker:

from the root of the project
```bash
podman compose -f docker-compose.yml up 
```

---

## Project Structure

`eza --tree --git-ignore `
```
.
├── catalog-service          # Python FastAPI Service
│   ├── main.py              # Application entry point
│   ├── pyproject.yaml       # Python dependencies
│   └── Dockerfile           # Container definition
├── checkout-service         # Rust Actix-web Service
│   ├── Cargo.toml           # Rust dependencies
│   ├── src/
│   │   └── main.rs          # Application entry point
│   └── Dockerfile           # Multi-stage container definition
├── docker-compose.yml       # Create a unified docker image
├── k8s                      # Kubernetes Manifests
│   ├── catalog-deployment.yaml
│   └── checkout-deployment.yaml
└── README.md
```

## TODO

- [ ] Deploy using kubernetes
- [ ] setup up CI/CD
- [ ] Create first release
- [ ] Deploy Prometheus, Loki and Grafana


## License

This project is open source and available under the [GNU-2.0 License](LICENSE).

