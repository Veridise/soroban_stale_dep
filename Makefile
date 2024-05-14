all:
	soroban contract build

test:
	cargo test --release

clean:
	rm -rf ./target/

## This target is for
second:
	cargo build --package second --target wasm32-unknown-unknown --release

keys:
	soroban network add \
		--global testnet \
		--rpc-url https://soroban-testnet.stellar.org:443 \
		--network-passphrase "Test SDF Network ; September 2015"
	soroban keys generate --global alice --network testnet

## Deploy into the test network using the `alice` user (shall be created beforehand)
deploy:
	soroban contract deploy \
		--wasm target/wasm32-unknown-unknown/release/first.wasm \
		--source alice \
		--salt $(shell od -An -N4 -l /dev/urandom | tr -d ' ') \
		--network testnet > ./first.txt    ## Save contract address into a file

## Call `consumer` contract function
invoke_consumer:
	soroban contract invoke \
	  --id $(shell cat ./first.txt) \
	  --source alice \
	  --network testnet \
	  -- \
	  consumer