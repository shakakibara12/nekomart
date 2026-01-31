## AWS & Kubernetes Deployment

>>>>>>>>>>>THIS IS AS OF NOW NOT TESTED.<<<<<<<<<<<
This project uses Terraform to provision infrastructure and ArgoCD for deployment.

### 1. Provision Infrastructure (Terraform)

*Note: Ensure your AWS credentials are configured.*

```bash
cd terraform
terraform init
terraform plan
terraform apply
```

This will create:
- VPC & Subnets
- EKS Cluster
- RDS / Elasticache (If defined)
- ECR Repositories

### 2. Push Images

Tag and push your local Docker images to the ECR repositories created by Terraform.

```bash
aws ecr get-login-password --region <region> | docker login --username AWS --password-stdin <account-id>.dkr.ecr.<region>.amazonaws.com

docker push <account-id>.dkr.ecr.<region>.amazonaws.com/nekomart-catalog:latest
docker push <account-id>.dkr.ecr.<region>.amazonaws.com/nekomart-checkout:latest
```

### 3. Deploy via ArgoCD (GitOps)

1. Install ArgoCD on your EKS cluster.
2. Create a new Application pointing to this repository.
3. ArgoCD will sync the manifests in `./k8s` automatically.

---

## 📊 Observability & Monitoring

Once deployed, the services expose metrics on `/metrics` (via Prometheus Exporters).

- **Grafana Dashboards** are configured to visualize:
  - **Memory Efficiency:** Comparison of Resident Set Size (RSS) between Rust and Python pods.
  - **Latency:** P95/P99 response times for API endpoints.
  - **Throughput:** Requests per second handled by each service.

### Comparing Performance

To verify the scalability difference:
1. Use a load testing tool (e.g., [K6](https://k6.io/)) to hit the `/products` (Python) and `/order` (Rust) endpoints.
2. Monitor the **Horizontal Pod Autoscaler (HPA)** events.
3. Observe Grafana to see how many Python pods spin up compared to Rust pods under identical load.
