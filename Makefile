CARGO_TARGET_DIR ?= target
PKG_BASE_NAME ?= timer-x86_64-unkown-linux-gnu

.DEFAULT: help
.PHONY: help
help:	## Show this help menu.
	@echo "Usage: make [TARGET ...]"
	@echo ""
	@@egrep -h "#[#]" $(MAKEFILE_LIST) | sed -e 's/\\$$//' | awk 'BEGIN {FS = "[:=].*?#[#] "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""

.PHONY: update-version
update-version: ## update version from VERSION file in all Cargo.toml manifests
update-version: */Cargo.toml
	@VERSION=`cat VERSION`; sed -i "0,/^version\ \= .*$$/{s//version = \"$$VERSION\"/}" */Cargo.toml
	@echo updated to version "`cat VERSION`" cargo files

.PHONY: build
build:	## compile timer
build:
	cargo build --release

lint:	## lint code
lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

test:	## run tests
test: lint
	cargo test

release:	## generate vendor.tar.gz and timer-v${VERSION}-x86_64-unkown-linux-gnu.tar.gz
	cargo vendor
	tar -czf vendor.tar.gz vendor
	cargo build --release
	tar -czf $(PKG_BASE_NAME).tar.gz -C $(CARGO_TARGET_DIR)/release timer

publish:	## publish crates
	@for package in $(shell find . -mindepth 2 -not -path './vendor/*' -name Cargo.toml -exec dirname {} \; | sort -r);do \
		cd $$package; \
		cargo publish; \
		cd -; \
	done;
