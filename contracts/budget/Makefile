.PHONY: build test

build:
	@cd contracts/budget && cargo build --target wasm32-unknown-unknown --release

test:
	@cd contracts/budget && cargo test

clean:
	@cd contracts/budget && cargo clean
