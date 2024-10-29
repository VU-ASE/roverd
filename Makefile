# Makefile in accordance with the docs on git management (to use in combination with meta)
.PHONY: build start clean test

BUILD_DIR=target/release
BINARY_NAME=roverd

lint:
	@cargo clippy

# build-open-api:
# 	@openapi-generator-cli generate -i spec/apispec.yaml -g rust -o openapi/
# 	@cat misc/openapi_lints.rs | cat - openapi/src/lib.rs > openapi/src/lib-temp.rs && mv openapi/src/lib-temp.rs openapi/src/lib.rs
# 	@cargo clippy --fix --lib -p openapi --allow-dirty

build-open-api:
	@openapi-generator-cli generate -i spec/apispec.yaml -g rust-axum  -o openapi/
	@cd openapi ; cargo fmt
	@cargo clippy

build-prod: build-open-api lint
	@cargo build --release

build-dev: build-open-api lint
	@cargo build

build: lint
	@cargo build

run: build
	@sudo ./target/debug/roverd

test: lint
	@cargo test

clean:
	rm -rf openapi/
	cargo clean





