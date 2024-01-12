PREFIX = $(HOME)/.local

build:
	mkdir -vp build
	cargo build --target-dir=build

build-release:
	mkdir -vp build
	cargo build --target-dir=build --release

install: build/release/pap
	mkdir -p $(PREFIX)/bin
	mv build/release/pap $(PREFIX)/bin
	chmod +x $(PREFIX)/bin/pap*

clean:
	rm -rf build

lint:
	cargo fmt
	cargo clippy