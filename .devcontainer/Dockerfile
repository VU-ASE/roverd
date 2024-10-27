# Use Ubuntu 22.04 as the base image
FROM ubuntu:22.04

# Set environment variables to prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

# Update package list and install necessary tools and Node.js
RUN apt-get update && apt-get install -y \
	build-essential \
	sudo \
	make \
	git \
	curl \
	gnupg


# Add custom user "dev" with sudo permissions
RUN useradd dev -u 1000 -m -s /bin/bash && \
	echo "dev ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers
USER dev

# Install Rust using rustup for the latest stable version
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

# Set up Rust environment variables
ENV PATH="/home/dev/.cargo/bin:${PATH}"

# Install common Rust tools
RUN rustup component add clippy rustfmt

# Setup stable toolchain
RUN rustup default stable

COPY --chown=dev:dev custom/gu /usr/local/bin/gu
COPY --chown=dev:dev custom/.bashrc /home/dev/.bashrc
