#!/usr/bin/env bash
set -euo pipefail

if command -v rustup >/dev/null 2>&1; then
  rustup component add rustfmt clippy
fi

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features

bash scripts/ci/bootstrap_cargo_audit.sh
export PATH="${HOME}/.cargo/bin:${PATH}"

cargo audit

cargo build --release
