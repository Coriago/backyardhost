# Default recipe - show help
default:
    @just --list

# Build the project (debug)
build:
    cargo build

# Build the project (release)
build-release:
    cargo build --release

# Run the project
run *ARGS:
    cargo run -- {{ARGS}}

# Run tests
test:
    cargo test

# Check the project compiles without building
check:
    cargo check

# Run the linter
lint:
    cargo clippy -- -D warnings

# Format source code
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Remove build artifacts
clean:
    cargo clean
