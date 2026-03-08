MSRV=$(shell cargo metadata --no-deps --format-version 1 \
	| sed -n 's/.*"rust_version":"\([[:digit:]]\{1,\}\.[[:digit:].]\{1,\}\)".*/\1/p')
ifeq ($(MSRV),)
$(error Failed to determine MSRV)
endif


help:
	@printf "Usage: make TARGET [-- ARGS...]\n\n"
	@printf "Available targets:\n"
	@awk -F ' +##' 'NF>1 {printf "\033[36m  %-18s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@printf "\n"

setup:  ## Install all tools for local development
	rustup toolchain add stable
	rustup toolchain add nightly
	rustup toolchain add $(MSRV)
	rustup component add clippy
	cargo install cargo-audit cargo-outdated

  ##

lint:  ## Run linter/analysis tool
	cargo clippy --all-targets --all-features -- -D warnings \
		-A clippy::zero_prefixed_literal -A clippy::missing_safety_doc

test:  ## Run tests (or specify a test name/selector to run just that)
	cargo test --all-features -- $(filter-out $@, $(MAKECMDGOALS))

fulltest:  ## Run all tests for all supported versions
	$(MAKE) clean
	cargo test --all-features
	cargo +$(MSRV) test --all-features
	cargo +nightly test --all-features

doc:  ## Generate documentation
	$(MAKE) clean
	cargo doc --no-deps --all-features
	@printf "\nDocs available at: file:///$$(pwd)/target/doc/gexiv2_sys/index.html\n"

  ##

clean:  ## Remove all build artefacts
	cargo clean

release-check:  ## Run checks before releasing a new version
	rustup update
	cargo update
	$(MAKE) fulltest
	$(MAKE) lint
	cargo run --example open_buf > /dev/null
	cargo run --example raw_tag_access > /dev/null
	cargo run --features xmp-packet-access --example xmp_packet_access > /dev/null
	cargo outdated --root-deps-only
	cargo audit


.PHONY: help setup lint test fulltest doc clean release-check
