MAKEFLAGS		+= --silent

all: | release

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy --all -- -D warnings

fmt:
	cargo fmt

clean:
	cargo clean

debug:
	cargo build

release:
	cargo build --release

regenerate:
	CARGO_REGENERATE=1 cargo build
	echo "\nregenerated protos:" && tree tensorflow
