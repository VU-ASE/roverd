# Makefile in accordance with the docs on git management (to use in combination with meta)
.PHONY: build start clean test

BUILD_DIR=target/release
BINARY_NAME=roverd

lint:
	cargo clippy

build: lint
	cargo build --release

clean:
	cargo clean

test: lint
	cargo test

generate-types:
	openapi-generator-cli generate -i spec/apispec.yaml -g rust -o types/


