PREFIX = $(HOME)/.local

lint:
	cargo fmt
	cargo clippy --all -- -D warnings