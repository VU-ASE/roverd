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

clean:
	@cargo clean
