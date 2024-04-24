.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown

.PHONY: run
run:
	cargo run --target wasm32-unknown-unknown