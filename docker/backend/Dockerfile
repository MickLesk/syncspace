# SyncSpace Backend - Multi-Stage Build
FROM rust:1.82-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    curl &&
    rm -rf /var/lib/apt/lists/*

# Copy backend source
COPY backend/ .

# Build application
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    libsqlite3-0 \
    curl &&
    rm -rf /var/lib/apt/lists/* &&
    useradd -r -s /bin/false syncspace

# Create app directory
RUN mkdir -p /app/data && chown -R syncspace:syncspace /app

# Copy binary and migrations
COPY --from=builder /app/target/release/syncbackend /app/syncbackend
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app
USER syncspace

EXPOSE 8080

CMD ["./syncbackend"]
