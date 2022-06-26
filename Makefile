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

web-build:
	parcel build web_build/index.html
	#parcel build web_build/index.html --no-source-maps
