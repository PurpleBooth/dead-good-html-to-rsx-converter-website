# This help screen
show-help:
  just --list

# Test it was built ok
test:
  RUST_BACKTRACE=1 cargo test

# Build release version
build:
  npx tailwindcss -i ./input.css -o ./public/tailwind.css  --minify
  dx build --profile release
  cargo build --profile release

# Run the dev server
dev: build
  cargo shuttle run

# Deploy the server
deploy: build
  cargo shuttle deploy

# Lint it
lint:
  cargo fmt --all -- --check
  cargo clippy -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
  cargo check
  cargo audit

# Format what can be formatted
fmt:
  cargo fix --allow-dirty --allow-staged
  cargo clippy --allow-dirty --allow-staged --fix -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery
  cargo fmt --all
  npx prettier --write **.yml

# Clean the build directory
clean:
  cargo clean
