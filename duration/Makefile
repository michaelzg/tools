# Specify the name of the binary
BINARY_NAME=dur

# Default target executed when no arguments are given to make.
default: build

# Build the project in release mode.
build:
	cargo build --release

# Run tests.
test:
	cargo test

# Move the binary to a convenient location (e.g., /usr/local/bin).
install: build
	sudo cp ./target/release/duration /usr/local/bin/$(BINARY_NAME)

# Clean up the project.
clean:
	cargo clean
	sudo rm /usr/local/bin/$(BINARY_NAME)

.PHONY: default build test install clean