.PHONY: build test

build:
	@cd contracts/settlement && cargo build --target wasm32-unknown-unknown --release

test:
	@cd contracts/settlement && cargo test

clean:
	@cd contracts/settlement && cargo clean
