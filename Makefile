all:
	soroban contract build
	cargo test --release

clean:
	rm -rf ./target/

second:
	cargo build --target wasm32-unknown-unknown --release