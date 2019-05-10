FLATC := flatc
VERBOSE := $(if ${CI},--verbose,)

FBS_FILES := $(filter-out schema/reflection.fbs,$(wildcard schema/*.fbs))
BFBS_FILES := $(patsubst %.fbs,%.bfbs,${FBS_FILES})
JSON_FILES := $(patsubst %.fbs,%.json,${FBS_FILES})
FLATC_RUST_FILES := $(patsubst schema/%.fbs,cfbc/tests/common/%_generated.rs,${FBS_FILES})
BUILDER_FILES := $(patsubst schema/%.fbs,cfbc/tests/common/%_builder.rs,${FBS_FILES})

GEN_FILES := ${BFBS_FILES} ${JSON_FILES} ${FLATC_RUST_FILES} ${BUILDER_FILES}
GEN_DEPENDENCIES := $(wildcard cfbc/src/helpers/*.rs) $(wildcard cfbc/templates/*.hbs)

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

%.bfbs: %.fbs
	$(FLATC) -b --schema -o $(shell dirname $@) $<

%.json: %.bfbs
	$(FLATC) -t --strict-json -o $(shell dirname $@) schema/reflection.fbs -- $<

cfbc/tests/common/%_generated.rs: schema/%.bfbs
	$(FLATC) -r -o $(shell dirname $@) $<
	rustfmt $@

cfbc/tests/common/%_builder.rs: schema/%.bfbs $(GEN_DEPENDENCIES)
	cargo run -- -o $(shell dirname $@) $<
	rustfmt $@

gen: $(GEN_FILES)

gen-clean:
	rm -f $(GEN_FILES)

clean: gen-clean

.PHONY: test fmt clippy check
.PHONY: ci ci-install
.PHONY: build-debug build-release
.PHONY: gen gen-clean
.PHONY: clean
