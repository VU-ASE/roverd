.PHONY: lint build-open-api build run test clean
SHELL := /bin/bash

BUILD_DIR=target/release
BINARY_NAME=roverd

lint:
	@cargo fmt
	@cargo clippy

build-open-api:
	@openapi-generator-cli generate -i spec/apispec.yaml -g rust-axum -o openapi/
	@cd openapi ; cargo fmt ; cargo clippy

build: build-open-api lint
	@cargo build --release

run: build
	@sudo ./target/release/roverd

test:
	@cargo test -- --show-output --test-threads 1

clean:
	rm -rf openapi/
	cargo clean
