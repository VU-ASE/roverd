# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Set environment variables to prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# Update package list and install necessary tools
RUN apt-get update && \
    apt-get install -y \
    git \
    curl \
    gnupg \
    make \
    && rm -rf /var/lib/apt/lists/*

# Install Rust using rustup for the latest stable version
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set up Rust environment variables
ENV PATH="/root/.cargo/bin:${PATH}"

# Install common Rust tools
RUN rustup component add clippy rustfmt
