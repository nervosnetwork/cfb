FLATC := flatc

FBS_FILES := $(wildcard tests/common/*.fbs)
BFBS_FILES := $(patsubst %.fbs,%.bfbs,${FBS_FILES})
JSON_FILES := $(patsubst %.fbs,%.json,${FBS_FILES})
FLATC_RUST_FILES := $(patsubst %.fbs,%_generated.rs,${FBS_FILES})
BUILDER_FILES := $(patsubst %.fbs,%_builder.rs,${FBS_FILES})
FLATBUFFERS_VERIFIER_FILES := $(patsubst %.fbs,%_generated_verifier.rs,${FBS_FILES})

GEN_FILES := ${BFBS_FILES} ${JSON_FILES} ${FLATC_RUST_FILES} ${BUILDER_FILES} ${FLATBUFFERS_VERIFIER_FILES}

TEMPLATES := $(wildcard cfb/templates/*.jinja)

ifeq (${VIRTUAL_ENV},)
  PIPENV_RUN := pipenv run
endif
VERBOSE := $(if ${CI},--verbose,)

test: test-python test-rust
test-python:
	${PIPENV_RUN} python -m unittest discover
test-rust:
	cargo test ${VERBOSE}

gen: ${GEN_FILES}
gen-clean:
	rm -f ${GEN_FILES}
gen-force: gen-clean gen

publish-python:
	rm -rf dist
	${PIPENV_RUN} python setup.py sdist bdist_wheel
	twine --version || ${PIPENV_RUN} pip install twine
	twine upload dist/*

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

%_builder.rs: %.bfbs ${TEMPLATES}
	pipenv run bin/cfbc -o $(shell dirname $@) $<

%_generated.rs: %.fbs
	$(FLATC) -r -o $(shell dirname $@) $<

%.bfbs: %.fbs
	$(FLATC) -b --schema -o $(shell dirname $@) $<

%.json: %.bfbs
	$(FLATC) -t --strict-json -o $(shell dirname $@) reflection.fbs -- $<

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --all --all-targets --all-features -- -D warnings -D clippy::clone_on_ref_ptr -D clippy::enum_glob_use

ci: ci-rust ci-python

ci-rust: fmt clippy test-rust
	git diff --exit-code Cargo.lock

ci-gen-prepare:
	rm -f ${BUILDER_FILES} ${FLATBUFFERS_VERIFIER_FILES}
	touch ${BFBS_FILES}
ci-gen: ${BUILDER_FILES}
	git diff --exit-code tests/common

ci-python: test-python ci-gen-prepare ci-gen

.PHONY: test test-python test-rust
.PHONY: gen gen-clean gen-force
.PHONY: doc doc-clean doc-publish
.PHONY: fmt clippy
.PHONY: ci ci-rust ci-python ci-gen ci-gen-clean
.PHONY: publish-python
