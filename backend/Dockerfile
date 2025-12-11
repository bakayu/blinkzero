# build stage
FROM rust:1.91-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy only the dependency manifests to cache dependencies
COPY Cargo.toml Cargo.lock ./
# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Copy the actual source code
COPY . .
RUN touch src/main.rs

# Build the binary
ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL is often needed for HTTP clients)
RUN apt-get update && apt-get install -y ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/blinkzero ./server
COPY --from=builder /app/configuration ./configuration

# Create a non-root user for security
RUN useradd -ms /bin/bash appuser
USER appuser

# Expose the port defined in production.yaml (8000)
EXPOSE 8000

CMD ["./server"]