#!/usr/bin/env bash
set -euo pipefail

if command -v rustup >/dev/null 2>&1; then
  rustup component add rustfmt clippy
fi

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

if cargo audit --version >/dev/null 2>&1; then
  cargo audit
elif command -v docker >/dev/null 2>&1; then
  docker run --rm -v "$PWD:/app" -w /app rustsec/rustsec:latest cargo audit
else
  echo "cargo-audit is not available and docker is not installed; cannot run security audit" >&2
  exit 1
fi

cargo build --release
