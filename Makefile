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

dev: build
	sudo ./target/release/roverd

build-arm:
	cargo build --target=aarch64-unknown-linux-gnu --release

dev-arm: build-arm
	sudo ./target

clean:
	@cargo clean

loc:
	@echo roverd:
	@cd roverd/src && find . -name '*.rs' | xargs wc -l
	@echo 

	@echo openapi:
	@cd roverd/openapi/src && find . -name '*.rs' | xargs wc -l
	@echo 

	@echo rovervalidate
	@cd rovervalidate/src && find . -name '*.rs' | xargs wc -l
	@echo 


