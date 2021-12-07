CARGO_TARGET_DIR ?= target
PKG_BASE_NAME ?= timer-x86_64-unkown-linux-gnu

.DEFAULT: help
.PHONY: help
help:	## Show this help menu.
	@echo "Usage: make [TARGET ...]"
	@echo ""
	@@egrep -h "#[#]" $(MAKEFILE_LIST) | sed -e 's/\\$$//' | awk 'BEGIN {FS = "[:=].*?#[#] "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
	@echo ""

.PHONY: build
build:	## compile timer
build:
	cargo build --release

.PHONY: lint
lint:	## lint code
lint:
	cargo clippy -- -D warnings
	cargo fmt -- --check

.PHONY: test
test:	## run tests
test: lint
	cargo test

.PHONY: tag
tag:	## create a tag using version from Cargo.toml
	PROJECT_VERSION=$$(sed -n 's/^version = "\(.*\)"/\1/p' timer_core/Cargo.toml | head -n1); \
	# add changelog to tag message \
	git tag -s v$${PROJECT_VERSION} && \
	git push origin v$${PROJECT_VERSION}

.PHONY: release
release:	## generate vendor.tar.gz and timer-v${VERSION}-x86_64-unkown-linux-gnu.tar.gz
	cargo vendor
	tar -czf vendor.tar.gz vendor
	cargo build --frozen --release --all-features
	tar -czf $(PKG_BASE_NAME).tar.gz -C $(CARGO_TARGET_DIR)/release timer
	@echo Released in $(CARGO_TARGET_DIR)/release/timer

.PHONY: publish
publish:	## publish crates
	@for package in $(shell find . -mindepth 2 -not -path './vendor/*' -name Cargo.toml -exec dirname {} \; | sort -r);do \
		cd $$package; \
		cargo publish; \
		cd -; \
	done;
