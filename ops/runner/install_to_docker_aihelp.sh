#!/usr/bin/env bash
set -euo pipefail

TARGET_ROOT=/docker/aihelp
TARGET_RUNNER=${TARGET_ROOT}/runner

mkdir -p "${TARGET_RUNNER}/state" "${TARGET_RUNNER}/work"
install -m 0644 ops/runner/docker-compose.yml "${TARGET_RUNNER}/docker-compose.yml"
install -m 0644 ops/runner/.env.example "${TARGET_RUNNER}/.env.example"
install -m 0644 ops/runner/README.md "${TARGET_RUNNER}/README.md"

if [[ ! -f "${TARGET_RUNNER}/.env" ]]; then
  cp "${TARGET_RUNNER}/.env.example" "${TARGET_RUNNER}/.env"
fi

echo "Runner assets installed at ${TARGET_RUNNER}"
echo "Edit ${TARGET_RUNNER}/.env, then run:"
echo "  cd ${TARGET_RUNNER} && docker compose up -d"
