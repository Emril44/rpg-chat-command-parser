# Makefile for rpg-chat-command-parser

.PHONY: test format lint build run publish

test:
	cargo test

format:
	cargo fmt

lint:
	cargo clippy -- -D warnings

build:
	cargo build --release

run:
	cargo run -- parse "/cast fireball --power=high"

publish: format lint test
	cargo publish

info:
	cargo run -- info