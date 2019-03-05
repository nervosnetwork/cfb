FLATC := flatc

FBS := $(wildcard tests/common/*.fbs)
BFBS := $(patsubst %.fbs,%.bfbs,${FBS})
JSON := $(patsubst %.fbs,%.json,${FBS})
FLATC_RS := $(patsubst %.fbs,%_generated.rs,${FBS})
BUILDER := $(patsubst %.fbs,%_builder.rs,${FBS})

ifeq (${VIRTUAL_ENV},)
  PIPENV_RUN := pipenv run
endif

test: test-python test-rust
test-python:
	${PIPENV_RUN} python -m unittest discover
test-rust:
	cargo test

gen: ${BFBS} ${JSON} ${FLATC_RS} ${BUILDER}
gen-clean:
	rm -f ${BFBS} ${JSON} ${FLATC_RS}

doc:
	cargo doc

doc-clean:
	rm -rf target/doc/

doc-publish: doc-clean doc
	git checkout gh-pages
	rsync -avu --delete -h target/doc/ ./doc/
	git add doc
	git commit --amend -n -m 'doc: publish doc' --date="$$(date -R)"
	git push --force origin gh-pages
	git checkout master

%_builder.rs: %.bfbs cfb/templates/builder.rs.jinja
	pipenv run bin/cfbc -o $(shell dirname $@) $<

%_generated.rs: %.fbs
	$(FLATC) -r -o $(shell dirname $@) $<

%.bfbs: %.fbs
	$(FLATC) -b --schema -o $(shell dirname $@) $<

%.json: %.bfbs
	$(FLATC) -t --strict-json -o $(shell dirname $@) reflection.fbs -- $<

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings -D clippy::clone_on_ref_ptr -D unused_extern_crates -D clippy::enum_glob_use

ci: ci-rust ci-python

ci-rust: fmt clippy test-rust
	git diff --exit-code Cargo.lock

ci-python: test-python

.PHONY: test test-python test-rust
.PHONY: gen gen-clean
.PHONY: doc doc-clean doc-publish
.PHONY: fmt clippy
.PHONY: ci ci-rust ci-python
