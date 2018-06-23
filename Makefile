NAME := sqlite
DEBUG := target/debug/$(NAME)
RELEASE := target/release/$(NAME)
WASM_TRPL := wasm32-unknown-unknown
WASM_DEBUG := target/$(WASM_TRPL)/debug/$(NAME).wasm
WASM_RELEASE := target/$(WASM_TRPL)/release/$(NAME).wasm
WASM_NPM_BIN := $(shell npm bin)/wa
SRC := $(shell find ./src -type f -name '*.rs')

all: $(DEBUG) $(RELEASE) # $(WASM_DEBUG) $(WASM_RELEASE)
#  $(NAME).debug.wast $(NAME).release.wast

$(DEBUG): $(SRC)
	cargo build

$(RELEASE): $(SRC)
	cargo build --release

$(WASM_DEBUG): $(SRC)
	cargo build --target $(WASM_TRPL)

$(WASM_RELEASE): $(SRC)
	cargo build --target $(WASM_TRPL) --release

$(NAME).debug.wast: $(WASM_DEBUG)
	$(WASM_NPM_BIN) disassemble -o $(NAME).debug.wast $(WASM_DEBUG)

$(NAME).release.wast: $(WASM_RELEASE)
	$(WASM_NPM_BIN) disassemble -o $(NAME).release.wast $(WASM_RELEASE)

.PHONY: test
test: $(DEBUG) clean
	cargo test 

.PHONY: install
install:
	rustup target add wasm32-unknown-unknown
	npm install

.PHONY: clean
clean:
	rm -f tmp/*.db
