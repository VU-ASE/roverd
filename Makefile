# Makefile in accordance with the docs on git management (to use in combination with meta)
.PHONY: build start clean test

BUILD_DIR=bin/
BINARY_NAME=rovervalidate

lint:
	# todo: lint checks here

build: lint
	# todo: rust build here

clean:
	# todo: clean here

test: lint
	# todo: rust tests here
