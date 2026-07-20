#!/usr/bin/env bash
set -euo pipefail

# Agent-friendly cleanup wrapper. Dry-run by default; pass --kill to terminate
# only the managed local-search browser listener and clear stale profile markers.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [ -x "$ROOT/target/debug/lsearch" ]; then
  exec "$ROOT/target/debug/lsearch" cleanup "$@"
fi

if [ -x "$ROOT/target/release/lsearch" ]; then
  exec "$ROOT/target/release/lsearch" cleanup "$@"
fi

exec lsearch cleanup "$@"
