# Makefile in accordance with the docs on git management (to use in combination with meta)
.PHONY: build start clean test

BUILD_DIR=target/release
BINARY_NAME=roverd

lint:
	@cargo fmt
	@cargo clippy

build-open-api:
	@openapi-generator-cli generate -i spec/apispec.yaml -g rust-axum -o openapi/
	@cd openapi ; cargo fmt
	@cargo clippy

build-prod: build-open-api lint
	@cargo build --release

build-dev: build-open-api lint
	@cargo build

build:
	@cargo build

run: build
	@sudo ./target/debug/roverd

test: lint
	@cargo test

clean:
	rm -rf openapi/
	cargo clean
