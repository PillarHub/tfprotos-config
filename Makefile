MAKEFLAGS		+= --silent

all: | inspect-protos fmt generated-line-count

inspect-protos:
	echo "\nregenerated protos:" && tree src/protos

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

generated-line-count:
	echo "\nGenerated lines:" && wc -l src/tensorflow.rs

msrv: | debug
	cargo msrv
