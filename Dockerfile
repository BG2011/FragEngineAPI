FROM rust:1.85-bookworm as builder

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake g++

WORKDIR /usr/src/app
COPY . .

# --- MEMORY OPTIMIZATIONS FOR RENDER FREE TIER ---
ENV CARGO_BUILD_JOBS=1
ENV CARGO_INCREMENTAL=0
ENV SQLX_OFFLINE=true

# Build the release binary
RUN cargo build --release

# --- PRODUCTION IMAGE ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /usr/src/app/target/release/fragengine /usr/local/bin/fragengine

# Standard Axum/Render configuration
EXPOSE 3000
ENV PORT=3000
ENV RUST_LOG=info

CMD ["fragengine"]