---
name: tests-skill
description: How to work in `tests/`. Read before editing here.
---

# Local-Search / tests

## Purpose
<!-- agent-docs:fill:purpose -->
Public CLI regression tests. Exercise real binaries against isolated local state,
fake browser endpoints, and fake registries rather than personal browser data.

## Mental model & key files
<!-- agent-docs:fill:model -->
- `cli.rs` checks help, welcome text, output modes, and structured errors.
- `connection.rs` checks explicit browser selection, background work tabs,
  disconnection, and cleanup boundaries with local fake CDP endpoints.
- `updates.rs` checks update JSON, aliases, package-manager detection, opt-out,
  timeout, and macOS interactive startup via `/usr/bin/script`.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
- Set `LOCAL_SEARCH_CONFIG_DIR` to a test-owned temporary directory.
- Never connect to a real signed-in browser or fetch a registry in unit/integration tests.
- Keep stdout contracts parseable, and assert stderr and exit status separately.
- Use subprocess-specific environment overrides, not global environment mutations.
- Bound waits and shut down fake listeners and child processes at test completion.
- Serialize update-check subprocess/PTY fixtures because their real two-second
  deadlines must not compete with sibling cold process launches on loaded hosts.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
- Run `cargo test --test updates` for release-check changes.
- Run `cargo test --test connection` for browser authority and lifecycle changes.
- Finish with `cargo fmt --check`, `cargo test`, and
  `cargo clippy --all-targets --all-features -- -D warnings`.

## Gotchas
<!-- agent-docs:fill:gotchas -->
- The npm wrapper has its own version; fixtures must not equate it with the native crate.
- Interactive update checks require both output streams to be terminals. Pipes
  intentionally do not exercise that path.
- Unix fake executables need execute permission. PTY assertions are macOS-only;
  pure update/cache tests also run on other platforms.
