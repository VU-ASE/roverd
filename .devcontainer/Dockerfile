FROM jetpackio/devbox:latest

# Set environment variables to prevent interactive prompts during package installation
ARG DEBIAN_FRONTEND=noninteractive

USER root:root

# Update packages and install common apt tools
RUN apt-get update && apt-get install -y \
	build-essential \
	sudo \
	make \
	git \
	curl \
	gnupg \
	binutils \
	xz-utils \
	wget


USER ${DEVBOX_USER}:${DEVBOX_USER}

COPY --chown=${DEVBOX_USER}:${DEVBOX_USER} .devcontainer/scripts/gu /usr/local/bin/gu
COPY --chown=${DEVBOX_USER}:${DEVBOX_USER} .devcontainer/scripts/.bashrc /home/${DEVBOX_USER}/.bashrc

# Install all devbox packages globally
COPY --chown=${DEVBOX_USER}:${DEVBOX_USER} devbox.json /home/${DEVBOX_USER}/.local/share/devbox/global/default/devbox.json
RUN devbox global install ; echo "Installed devbox packages."


# --- DEPENDENCIES ---

RUN devbox global run -- rustup default stable
RUN devbox global run -- rustup component add clippy rustfmt

RUN sudo apt install -y libssl-dev pkg-config

# -----------------------------






