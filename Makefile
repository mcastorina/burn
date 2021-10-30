BIN := burn
RM := rm -f

RS_FILES := $(shell find src -type f)
.DEFAULT_GOAL := build

.PHONY: check
check:
	cargo check

build: target/debug/$(BIN)
release: target/release/$(BIN)

target/debug/$(BIN): Cargo.toml $(RS_FILES)
	cargo build

target/release/$(BIN): Cargo.toml $(RS_FILES)
	cargo build --release

fmt: $(RS_FILES)
	cargo fmt

.PHONY: test
test:
	cargo test

.PHONY: test-all
test-all:
	cargo test -- --include-ignored

.PHONY: clean
clean:
	$(RM) target/debug/$(BIN) target/release/$(BIN)

.PHONY: clean-all
clean-all:
	cargo clean
