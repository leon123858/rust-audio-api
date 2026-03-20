.PHONY: help build release test test-doc clippy fmt clean example-karaoke example-reverb example-dynamic

help:
	@echo "Available commands:"
	@echo "  make build           - Build the project (debug)"
	@echo "  make release         - Build the project (release)"
	@echo "  make test            - Run all unit and doc tests"
	@echo "  make doc             - Build documentation
	@echo "  make clippy          - Run Rust clippy linter"
	@echo "  make fmt             - Format cargo code"
	@echo "  make clean           - Clean build artifacts"
	@echo "  make example-karaoke - Run the play_karaoke example (in release mode)"
	@echo "  make example-reverb  - Run the play_mic_reverb example (in release mode)"
	@echo "  make example-dynamic - Run the play_dynamic_properties example (in release mode)"
	@echo "  make publish         - Publish the crate to crates.io"

build:
	cargo build

publish-test:
	make release
	cargo test --doc
	cargo test
	make clippy
	make fmt
	cargo publish --dry-run --allow-dirty

publish:
	cargo publish

release:
	cargo build --release

test:
	cargo test

doc:
	cargo test --no-deps --doc
	cargo doc --no-deps --open

clippy:
	cargo clippy --all-targets --all-features

fmt:
	cargo fmt

clean:
	cargo clean

example-karaoke:
	cargo run --release --example play_karaoke

example-reverb:
	cargo run --release --example play_mic_reverb

example-dynamic:
	cargo run --release --example play_dynamic_properties
