#!/usr/bin/env bash
set -euo pipefail

if cargo audit --version >/dev/null 2>&1; then
  exit 0
fi

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64)
    AUDIT_VERSION="0.22.1"
    AUDIT_ASSET="cargo-audit-x86_64-unknown-linux-musl-v${AUDIT_VERSION}.tgz"
    AUDIT_DIR="cargo-audit-x86_64-unknown-linux-musl-v${AUDIT_VERSION}"
    ;;
  *)
    echo "cargo-audit is missing and no bootstrap binary is configured for this platform" >&2
    exit 1
    ;;
esac

tmpdir_audit="$(mktemp -d)"
cleanup() {
  rm -rf "${tmpdir_audit}"
}
trap cleanup EXIT

curl -fsSL \
  "https://github.com/rustsec/rustsec/releases/download/cargo-audit/v${AUDIT_VERSION}/${AUDIT_ASSET}" \
  -o "${tmpdir_audit}/${AUDIT_ASSET}"
tar -xzf "${tmpdir_audit}/${AUDIT_ASSET}" -C "${tmpdir_audit}"
install -m 0755 "${tmpdir_audit}/${AUDIT_DIR}/cargo-audit" "${HOME}/.cargo/bin/cargo-audit"
