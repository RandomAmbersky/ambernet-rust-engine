AMBERSKYNET_LIB_PATH=./src/amberskynet/
MANIFEST_PATH=./src/amberskynet/Cargo.toml

build: rust-build

clean: rust-clean

rust-watch:
	cargo watch

rust-clean:
	cargo clean --manifest-path $(MANIFEST_PATH)

rust-build:
	wasm-pack build $(AMBERSKYNET_LIB_PATH)
#	cargo build --manifest-path $(MANIFEST_PATH) --target wasm32-unknown-unknown --release
#	wasm-bindgen ./src/target/wasm32-unknown-unknown/release/amberskynet.wasm --out-dir web_build --out-name amberskynet --target web --no-typescript

web-build:
	parcel build web_build/index.html
	#parcel build web_build/index.html --no-source-maps
