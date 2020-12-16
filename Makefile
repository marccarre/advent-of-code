IMAGE_REGISTRY := docker.io
IMAGE_ORG := marccarre
IMAGE_NAME := linter
IMAGE_TAG := $(shell grep "^version\s*=" Cargo.toml | head -n 1 | cut '-d"' -f2)
IMAGE := $(IMAGE_REGISTRY)/$(IMAGE_ORG)/$(IMAGE_NAME):$(IMAGE_TAG)

CURRENT_DIR := $(dir $(realpath $(firstword $(MAKEFILE_LIST))))

.DEFAULT_GOAL := all
.PHONY: all
all: clean build lint test coverage

.PHONY: build
build:
	docker build --target build -t $(IMAGE) $(CURRENT_DIR)

.PHONY: lint
lint:
	docker build --target lint $(CURRENT_DIR)

.PHONY: test
test:
	docker build --target test $(CURRENT_DIR)

.PHONY: coverage
coverage:
	# tarpaulin leverages ptrace, which requires special security permissions to
	# work properly, hence --security-opt seccomp=unconfined below:
	docker run --security-opt seccomp=unconfined \
		-v "$(CURRENT_DIR):/volume" \
		xd009642/tarpaulin

.PHONY: clean
clean:
	docker rmi -f $(IMAGE)
	rm -fr $(CURRENT_DIR)/target
