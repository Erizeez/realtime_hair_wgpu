.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown

.PHONY: run
run:
	cargo run

.PHONY: run-wasm
run-wasm:
	cargo run --target wasm32-unknown-unknown

.PHONY: run-trace
run-trace:
	WASM_SERVER_RUNNER_ADDRESS=0.0.0.0 cargo run --target wasm32-unknown-unknown --features bevy/trace_chrome