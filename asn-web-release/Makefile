run: rust-run

build: rust-build

build-wasm:
	npm i
	npm run build

serve:
	npm i
	npm run serve
	#cargo build --release --target wasm32-unknown-unknown

rust-build:
	cargo build --release

rust-run:
	cargo run --release

show:
	rustup show

# example delete:
# rustup toolchain remove 1.68-aarch64-apple-darwin
