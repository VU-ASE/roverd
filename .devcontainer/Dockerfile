FROM ubuntu:22.04

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && \
    apt-get install -y curl \
        sudo \
        nano \
        software-properties-common \
        build-essential \
        gcc-aarch64-linux-gnu \
        gcc-arm-linux-gnueabihf \
        pkg-config

# Java install
RUN add-apt-repository ppa:openjdk-r/ppa && \
apt-get update && \
apt-get install -y openjdk-17-jdk-headless 

# Python 3 install
RUN apt-get install -y python3 python3-pip

RUN echo 'root:dev' | chpasswd
RUN useradd -ms /bin/bash dev && echo 'dev:dev' | chpasswd && adduser dev sudo
RUN echo 'dev ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

COPY --chown=dev:dev .devcontainer/scripts/gu /usr/local/bin/gu
COPY --chown=dev:dev .devcontainer/scripts/.bashrc /home/dev/.bashrc

USER dev

# Rust Install
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN /home/dev/.cargo/bin/rustup target add aarch64-unknown-linux-gnu

# OpenAPI-cli install
RUN pip install openapi-generator-cli==4.3.1

# Cross Compilation Dependencies
RUN sudo apt install -y libssl-dev file git \
    gcc-aarch64-linux-gnu \
    binutils-aarch64-linux-gnu


# Clean up
# RUN apt-get clean && rm -rf /var/lib/apt/lists/*
