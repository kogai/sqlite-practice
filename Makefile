NAME := sqlite
DEBUG := target/debug/$(NAME)
RELEASE := target/release/$(NAME)
WASM_TRPL := wasm32-unknown-unknown
WASM_DEBUG := target/$(WASM_TRPL)/debug/$(NAME)
WASM_RELEASE := target/$(WASM_TRPL)/release/$(NAME)
SRC := $(shell find ./src -type f -name '*.rs')


all: $(DEBUG) $(RELEASE) $(WASM_DEBUG) $(WASM_RELEASE)

$(DEBUG): $(SRC)
	cargo build

$(RELEASE): $(SRC)
	cargo build --release

$(WASM_DEBUG): $(SRC)
	cargo build --target $(WASM_TRPL)

$(WASM_RELEASE): $(SRC)
	cargo build --target $(WASM_TRPL) --release

.PHONY: install
install:
	rustup target add wasm32-unknown-unknown
