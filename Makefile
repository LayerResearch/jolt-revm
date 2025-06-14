.PHONY: bootstrap help

help:
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

bootstrap: ## Install required dependencies
	git config --global --add safe.directory $(shell pwd)/
	cargo install cargo-nextest	

	apt-get update && apt-get install -y --no-install-recommends gh device-tree-compiler

	@if ! gh auth status >/dev/null 2>&1; then \
		echo "GitHub authentication required. Please login:"; \
		gh auth login; \
	fi
	mkdir -p /opt/riscv/
	gh release download --clobber spike-1.1.1 --repo LayerResearch/jolt-revm --pattern "spike-1.1.1-$(shell uname -s)-$(shell uname -m).tar.gz" -O /tmp/spike.tar.gz && tar -xzf /tmp/spike.tar.gz -C /opt/riscv/
	gh release download --clobber sail-riscv-0.7 --repo LayerResearch/jolt-revm --pattern "sail-riscv-0.7-$(shell uname -s)-$(shell uname -m).tar.gz" -O /tmp/sail.tar.gz && tar -xzf /tmp/sail.tar.gz -C /opt/riscv/

build-spike: ## Build the guest binary to run in Spike
	CARGO_PROFILE_RELEASE_LTO=false \
	CARGO_ENCODED_RUSTFLAGS=$(shell printf -- '-Clink-arg=-T$(shell pwd)/guest/riscv32im-unknown-none-elf.ld') \
	cargo build -p revm-guest --release --target riscv64im-unknown-none-elf --features no-jolt

clean-spike: ## Clean the build artifacts
	cargo clean -p revm-guest --target riscv32im-unknown-none-elf

inspect-spike: ## Inspect the built binary (size, sections, symbols)
	ls -lah ./target/riscv32im-unknown-none-elf/release/revm-guest
	readelf -hl ./target/riscv32im-unknown-none-elf/release/revm-guest
	nm ./target/riscv32im-unknown-none-elf/release/revm-guest | grep -E '(tohost|fromhost)'

run-spike: build-spike ## Run the binary in Spike emulator
	spike --isa=rv32im ./target/riscv32im-unknown-none-elf/release/revm-guest

lint: ## Fix linting errors
	cargo clippy --fix --allow-dirty --allow-staged -- -D warnings
	cargo fmt --all --

build-host: ## Build the host binary
	cargo build -p revm-host --release

run-jolt: build-host ## Run the guest binary with Jolt
	RUST_BACKTRACE=full ./target/release/revm-host

test-guest: ## Test the guest binary on the building host
	cargo nextest run -p revm-guest

build-jolt: ## Build the guest binary with Jolt
	CARGO_ENCODED_RUSTFLAGS=$(shell printf -- '-Clink-arg=-T/tmp/jolt-guest-linkers/revm-guest.ld\x1f-Cpasses=lower-atomic\x1f-Cpanic=abort\x1f-Cstrip=symbols\x1f-Copt-level=z') \
	JOLT_FUNC_NAME=exec \
	cargo build --release --features guest -p revm-guest --target-dir /tmp/jolt-guest-targets/revm-guest/ --target riscv32im-unknown-none-elf

test-host: ## Test the host binary
	RUST_BACKTRACE=full cargo nextest run -p revm-host --no-capture

ci: ## Run the CI pipeline
	make bootstrap
	make test-guest
	make run-spike
	make test-host
