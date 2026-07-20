---
name: local-search-skill
description: How to work in this repository. Read before editing here.
---

# local-search — working here

## Purpose
Build and maintain the `local-search` CLI: free structured search, scraping,
artifacts, and browser interaction through a user's existing local browser
session.

## Mental model & key files
- `src/cli.rs` owns the public command surface. The primary binary is `lsearch`;
  compatibility binaries are `local-search` and `local-browser`.
- `src/commands/mod.rs` maps CLI commands to browser actions and JSON/text
  output contracts.
- `src/browser/` is the transport and browser-runtime layer. Keep CDP protocol
  details there.
- `README.md` is the user-facing contract; update examples when command behavior
  changes.

## Patterns to follow / invariants
- Search must run in the local browser. Do not add paid search APIs or hosted
  search services.
- Keep successful structured output under an `{ "ok": true, ... }` JSON envelope.
- Keep failures as stable JSON on stderr via `output::render_error`.
- Redact credential-bearing values by default. Generated artifacts are ignored.
- Default transport is an already-running Chromium-family browser CDP endpoint,
  not a launched clean profile.

## Common tasks → first action
- CLI shape change: edit `src/cli.rs`, then add the command handler in
  `src/commands/mod.rs`.
- Browser capability change: read `src/browser/SKILL.md`, then edit CDP helpers
  or scripts.
- Output contract change: update README examples and integration tests.
- Before commit: run `cargo fmt`, `cargo test`, and
  `cargo clippy --all-targets --all-features -- -D warnings`.

## Gotchas
- Chrome 136+ restricts default-profile remote debugging; prefer the
  `DevToolsActivePort` browser websocket when local debugging is enabled.
- Safari WebDriver does not satisfy the normal signed-in profile requirement.
- CDP endpoints grant broad local browser authority. Never print raw cookies
  unless a command explicitly opts in.
