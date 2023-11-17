SHELL:=/bin/sh
.PHONY: all

VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")
RUST_VERSION = $(word 2, $(shell rustc -V))
LONG_VERSION = "$(VERSION) ( rev: $(GIT_REVISION), rustc: $(RUST_VERSION), build at: $(BUILD_TIME) )"
BINARY_NAME = aws-sso-auth

export LONG_VERSION

help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

doctoc: ## Create table of contents with doctoc
	doctoc .

pre-commit: ## Run pre-commit
	pre-commit run -a

release-linux: ## Create release for linux arm and amd64
	# First make release for amd64
	cargo build --release --target=x86_64-unknown-linux-musl
	zip -j ${BINARY_NAME}-v${VERSION}-x86_64-linux.zip target/x86_64-unknown-linux-musl/release/${BINARY_NAME}
	# Release for arm64
	cargo build --release --target=aarch64-unknown-linux-gnu
	zip -j ${BINARY_NAME}-v${VERSION}-arm64-linux.zip target/aarch64-unknown-linux-gnu/release/${BINARY_NAME}


release-mac: ## Create release for mac arm and amd64
	cargo build --release --target=x86_64-apple-darwin
	zip -j ${BINARY_NAME}-v${VERSION}-x86_64-mac.zip target/x86_64-apple-darwin/release/${BINARY_NAME}
	# Release for arm64
	cargo build --release --target=aarch64-apple-darwin
	zip -j ${BINARY_NAME}-v${VERSION}-arm64-mac.zip target/aarch64-apple-darwin/release/${BINARY_NAME}
