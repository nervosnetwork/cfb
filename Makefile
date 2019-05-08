VERBOSE := $(if ${CI},--verbose,)

test:
	cargo test ${VERBOSE} --all

fmt:
	cargo fmt ${VERBOSE} -- --check

clippy:
	cargo clippy ${VERBOSE} --all --all-targets --all-features -- -D warnings -D clippy::clone_on_ref_ptr -D clippy::enum_glob_use -D clippy::fallible_impl_from

check:
	cargo check ${VERBOSE} --all

ci: fmt clippy check test

ci-install:
	cargo fmt --version || rustup component add rustfmt
	cargo clippy --version || rustup component add clippy

build-debug:
	cargo build ${VERBOSE}

build-release:
	cargo build ${VERBOSE} --release

.PHONY: test fmt clippy check
.PHONY: ci ci-install
.PHONY: build-debug build-release
