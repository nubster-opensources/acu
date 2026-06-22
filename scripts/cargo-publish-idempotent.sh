#!/usr/bin/env bash
# Publish a crate to crates.io, skipping the call when the version is already
# live. Idempotency lets the release job be retried safely on partial failure.
#
# Usage: cargo-publish-idempotent.sh <crate-name> [cargo publish args...]
#
# The first argument is the crate name; every subsequent argument is forwarded
# to `cargo publish` verbatim (e.g. --dry-run, --token ...).

set -euo pipefail

CRATE="${1:?first argument must be the crate name}"
shift

# Resolve the version from the workspace manifest.
VERSION="$(cargo metadata --no-deps --format-version 1 \
  | grep -o "\"name\":\"${CRATE}\"[^}]*\"version\":\"[^\"]*\"" \
  | grep -o '"version":"[^"]*"' \
  | grep -o '[0-9][^"]*')"

if [[ -z "${VERSION}" ]]; then
  echo "error: could not determine version for crate '${CRATE}'" >&2
  exit 1
fi

# Check whether this exact version is already published.
HTTP_STATUS="$(curl -s -o /dev/null -w '%{http_code}' \
  "https://crates.io/api/v1/crates/${CRATE}/${VERSION}")"

if [[ "${HTTP_STATUS}" == "200" ]]; then
  echo "${CRATE} v${VERSION} is already published on crates.io, skipping."
  exit 0
fi

echo "Publishing ${CRATE} v${VERSION}..."
cargo publish --package "${CRATE}" "$@"
