# 1. Build Stage
FROM rust:1.92.0-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 2. Runtime Stage
FROM debian:trixie-slim
WORKDIR /app
COPY --from=builder /app/target/release/checkout-service /app/checkout-service

# Expose port 8080
EXPOSE 8080

CMD ["/app/checkout-service"]
