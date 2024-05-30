# Set the name of your Axum server binary
BINARY_NAME=basic_web_service

# Set the path to the source files you want to watch for changes
WATCH_PATHS=src

# Set the cargo command to run your server
RUN_CMD=cargo run --bin $(BINARY_NAME)

.PHONY: default
default: build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	$(RUN_CMD)

.PHONY: watch
watch:
	while true; do \
		$(MAKE) build; \
		$(RUN_CMD) || break; \
		inotifywait -qrre close_write,moved_to,create,delete $(WATCH_PATHS) || break; \
	done

.PHONY: check
check:
	cargo check

.PHONY: clean
clean:
	cargo clean

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all -- --check

.PHONY: clippy
clippy:
	cargo clippy --all-targets --all-features --tests -- -D warnings

.PHONY: test
test:
	cargo test --all --workspace --bins --tests --benches
