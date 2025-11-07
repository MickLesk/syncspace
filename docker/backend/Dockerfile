# Simple Rust Backend Build
FROM rust:1.82-slim as builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

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
    && rm -rf /var/lib/apt/lists/*

# Create app directory
RUN mkdir -p /app/data

# Copy binary and migrations
COPY --from=builder /app/target/release/syncbackend /app/syncbackend
COPY --from=builder /app/migrations /app/migrations

WORKDIR /app

EXPOSE 8080

CMD ["./syncbackend"]