.PHONY: build start clean test
SHELL := /bin/bash

# This Makefile entrypoint is used by CI

lint:
	@cargo fmt
	@cargo clippy

test:
	@cargo test

build:
	@cargo build --release

setup:
	sudo ./scripts/setup_rover_files.sh

run: build setup
	sudo ./target/release/roverd

clean:
	@cargo clean




