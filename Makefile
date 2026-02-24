.PHONY: generate update build test clean

generate:
	./scripts/generate.sh

update:
	./scripts/update.sh

build:
	cargo build

test:
	cargo test

clean:
	cargo clean
