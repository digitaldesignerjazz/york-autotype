# York Autotype — Nexus automation prototype with nxmesh AgentHeartbeat
# Multi-stage build, Docker + Podman compatible

FROM rust:1.75-bookworm AS builder

WORKDIR /build

# Placeholder for dependency caching
COPY Cargo.toml ./
RUN mkdir -p src/bin && \
    echo "fn main() {}" > src/lib.rs && \
    echo "fn main() {}" > src/bin/york-heartbeat.rs && \
    cargo build --release 2>/dev/null || true

# Real sources
COPY src ./src
COPY Cargo.toml ./

RUN cargo build --release --bin york-heartbeat

# ── Runtime ──────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /build/target/release/york-heartbeat /app/york-heartbeat

VOLUME ["/data", "/status"]

ENV RUST_LOG=york_autotype=info
ENV YORK_STATUS_DIR=/status
ENV YORK_NODE_ID=york-container-01

ENTRYPOINT ["/app/york-heartbeat"]
CMD ["--node-id", "york-container-01", "--interval", "30", "--status-dir", "/status"]
