prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd contract && cargo build --release --target wasm32-unknown-unknown
	# cd receive_inventory && cargo build --release --target wasm32-unknown-unknown
	#
# cd send_inventory && cargo build --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/inventory-count.wasm 2>/dev/null | true
	# wasm-strip receive_inventory/target/wasm32-unknown-unknown/release/inventory-receive.wasm 2>/dev/null | true
	# wasm-strip send_inventory/target/wasm32-unknown-unknown/release/inventory-send.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp contract/target/wasm32-unknown-unknown/release/inventory-count.wasm tests/wasm
	# cp receive_inventory/target/wasm32-unknown-unknown/release/inventory-receive.wasm tests/wasm
	# cp send_inventory/target/wasm32-unknown-unknown/release/inventory-send.wasm tests/wasm
	cd tests && cargo test

clippy:
	cd contract && cargo clippy --all-targets -- -D warnings
	# cd receive_inventory && cargo clippy --all-targets -- -D warnings
	# cd send_inventory && cargo clippy --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cd contract && cargo fmt -- --check
	# cd receive_inventory && cargo fmt -- --check
	# cd send_inventory && cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cd contract && cargo fmt
	# cd tests && cargo fmt
	# cd receive_inventory && cargo fmt
	cd send_inventory && cargo fmt

clean:
	cd contract && cargo clean
	# cd receive_inventory && cargo clean
	# cd send_inventory && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
