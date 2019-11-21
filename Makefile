@PHONY: build-deps test doc docs

default_toolchain ?= $(rustup toolchain list | grep 'default' | cut -d ' ' -f 1)

build-deps:
	rustup update
	rustup override set stable
	rustup component add rustfmt --toolchain $(default_toolchain)
	rustup component add clippy --toolchain $(default_toolchain)

test:
	cargo test
	cargo fmt --all
	cargo clippy

docs:
	make doc

doc:
	cargo doc --no-deps
	rm -rf docs/doc
	cp -r target/doc docs/
