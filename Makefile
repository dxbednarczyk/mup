lint:
	cargo fmt
	cargo clippy --all --fix --allow-dirty -- -D warnings