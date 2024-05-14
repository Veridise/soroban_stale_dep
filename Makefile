all:
	soroban contract build

test:
	cargo test --release

clean:
	rm -rf ./target/

## This target is for
second:
	cargo build --package second --target wasm32-unknown-unknown --release