AMBERSKY_LIB_PATH=./src/amberskynet
MANIFEST_PATH=./src/Cargo.toml

build: rust-build web-build

rust-clean:
	cargo clean --manifest-path $(MANIFEST_PATH)/Cargo.toml

rust-build:
	#cargo build --manifest-path $(MANIFEST_PATH) --target wasm32-unknown-unknown --release
	wasm-pack build $(AMBERSKY_LIB_PATH) --release --out-dir ../../amberskynet --out-name amberskynet --target web
	#wasm-bindgen ./src/target/wasm32-unknown-unknown/release/amberskynet.wasm --out-dir amberskynet --out-name amberskynet --target web
	#wasm-bindgen ./src/target/wasm32-unknown-unknown/release/amberskynet.wasm --out-dir amberskynet_pkg --out-name amberskynet --target web --no-typescript

web-build:
	yarn install
	parcel build web/index.html --no-source-maps
	#parcel build web_build/index.html
	#parcel build web_build/index.html --no-source-maps
