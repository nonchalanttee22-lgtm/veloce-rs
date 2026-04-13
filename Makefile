# Veloce-rs automation
all: build

build:
	cargo build --target wasm32-unknown-unknown --release
	@echo "✅ WASM build complete in target/wasm32-unknown-unknown/release/"

test:
	cargo test

clean:
	cargo clean

# Quick deploy to testnet (Update your secret key and network)
deploy:
	soroban contract deploy \
	  --wasm target/wasm32-unknown-unknown/release/veloce_rs.wasm \
	  --source-account default \
	  --network testnet
