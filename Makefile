.PHONY: build test test-all clean fmt clippy

build:
	@echo "Building Soroban contracts..."
	@cargo build --target wasm32-unknown-unknown --release

test:
	@echo "Running tests..."
	@cargo test --lib

test-all:
	@echo "Running all tests..."
	@cargo test

fmt:
	@echo "Formatting code..."
	@cargo fmt

clippy:
	@echo "Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings

clean:
	@echo "Cleaning build artifacts..."
	@cargo clean

build-wasm:
	@mkdir -p target/wasm32-unknown-unknown/release
	@cargo build --target wasm32-unknown-unknown --release

verify-wasm: build-wasm
	@echo "Verifying WASM artifacts..."
	@ls -lh target/wasm32-unknown-unknown/release/*.wasm
