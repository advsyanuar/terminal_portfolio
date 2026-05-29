---
title: "Cyber Pulse v1.0"
description: "High-performance data ingestion pipeline in Rust. 1M+ req/s with low latency. Real-time telemetry included."
date: 2024-10-01
techStack: ["Rust", "Tokio", "Kafka", "Prometheus"]
links:
  source: "https://github.com/manonthemoon/cyber-pulse"
  demo: "https://example.com"
featured: true
---

## Architecture

Cyber Pulse is an async data ingestion pipeline built with Tokio. It reads from Kafka topics, transforms streams in-flight, and writes to configurable sinks.

### Performance

- **Throughput**: 1M+ requests/second
- **p99 Latency**: <5ms
- **Memory**: ~120MB steady-state

### Design

```
Kafka → Consumer → Transform Chain → Sink
```

Each stage is a trait. The pipeline is composed at build time via configuration.
