# ------------------------------------------------------------------------------
# Cargo Updater Stage
# ------------------------------------------------------------------------------
FROM rust:latest AS updater
# FROM ekidd/rust-musl-builder AS updater
# Work dir
WORKDIR /app
# Update libraries
RUN apt-get update -y && apt-get upgrade -y

# ------------------------------------------------------------------------------
# Cargo Musl Stage
# ------------------------------------------------------------------------------
FROM updater AS musl
# Install musl-tools and openssl
RUN apt-get install musl-tools libssl-dev -y
# RUN apt-get install pkg-config libssl-dev -y
# Add target musl
RUN rustup target add x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Cargo Chef Stage
# ------------------------------------------------------------------------------
FROM musl AS chef
# Install cargo-chef
RUN cargo install cargo-chef 

# ------------------------------------------------------------------------------
# Cargo Planner Stage: Generate receipt file
# ------------------------------------------------------------------------------
FROM chef AS planner
# Copy files
COPY . .
# Build the app
RUN cargo chef prepare --recipe-path recipe.json

# ------------------------------------------------------------------------------
# Cargo Cacher Stage: Build dependencies
# ------------------------------------------------------------------------------
FROM chef AS cacher
# Copy receipt file
COPY --from=planner /app/recipe.json recipe.json
# Build the app
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# ------------------------------------------------------------------------------
# Cargo Build Stage: Build app
# ------------------------------------------------------------------------------
FROM cacher AS builder
# Copy files
COPY . .
# Set the env args
ENV RUSTFLAGS=-Clinker=musl-gcc
# Build the app
RUN cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
# FROM gcr.io/distroless/cc-debian12:latest AS runtime
FROM alpine:latest AS runtime
# Work dir
WORKDIR /app
# Copy files
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ddns_client /app/ddns_client
# Run the app
CMD ["./ddns_client /data/config.yaml --demon"]

# docker login --username mouradost --password ${{ secrets.GH_TOKEN }} ghcr.io
# docker build -t ghcr.io/mouradost/ddns_client:latest .
# docker push ghcr.io/mouradost/ddns_client:latest
