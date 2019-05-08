test:
	cargo test --all

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy

check:
	cargo check --all

ci: fmt clippy check test

ci-install:
	cargo fmt --version || rustup component add rustfmt
	cargo clippy --version || rustup component add clippy

build-debug:
	cargo build --all

build-release:
	cargo build --release --all

.PHONY: test fmt clippy check
.PHONY: ci ci-install
.PHONY: build-debug build-release
