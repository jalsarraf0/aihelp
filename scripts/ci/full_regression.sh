#!/usr/bin/env bash
set -euo pipefail

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

if ! cargo audit --version >/dev/null 2>&1; then
  cargo install cargo-audit
fi

cargo audit
cargo build --release
