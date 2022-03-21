build:
	cargo fmt
	cargo build

build-mac:
	cargo fmt
	RUSTFLAGS='-L/opt/homebrew/opt/mysql-client/lib' cargo build

build-release:
	cargo fmt
	cargo build --release

test:
	cargo fmt
	cargo test --all

run:
	cargo fmt
	RUST_LOG=info cargo run

run-mac:
	cargo fmt
	RUST_LOG=info RUSTFLAGS='-L/opt/homebrew/opt/mysql-client/lib' cargo run

run-release:
	RUST_LOG=info ./target/release/cota-registry-aggregator

install:
	cargo fmt
	cargo install --path .

.PHONY: build test run install