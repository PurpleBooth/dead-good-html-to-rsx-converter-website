# This help screen
show-help:
        just --list

# Test it was built ok
test:
  RUST_BACKTRACE=1 cargo test --features ssr

# Build release version
build:
  npx tailwindcss -i ./input.css -o ./public/tailwind.css --minify
  dx build --features web --release
  cargo build --features ssr --release

dev:
  npx tailwindcss -i ./input.css -o ./public/tailwind.css
  dx build --features web
  cargo run --features ssr


# Check performance
bench:
  cargo bench

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
