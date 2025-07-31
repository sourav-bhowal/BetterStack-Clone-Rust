# .devcontainer/Dockerfile
FROM rust:latest

# Install system packages
RUN apt-get update && apt-get install -y \
    pkg-config \
    libpq-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
