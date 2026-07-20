---
name: src-bin-skill
description: CLI binary shims for `lsearch`, `local-search`, and legacy `local-browser`.
---

# local-search / src/bin

## Purpose
Expose the public binaries for the Rust crate. `lsearch` is the primary CLI;
`local-search` and `local-browser` are compatibility aliases that execute the
same entrypoint.

## Mental model & key files
- `lsearch.rs` owns the binary entrypoint and calls `local_search::run`.
- `local-search.rs` and `local-browser.rs` intentionally include `lsearch.rs`.
  Keep them boring so all command behavior stays centralized in `src/cli.rs`
  and `src/commands/mod.rs`.

## Patterns to follow / invariants
- Do not add command logic in this directory.
- Keep `lsearch` as the recommended command in docs and examples.
- Preserve `local-browser` until there is an explicit breaking-release decision.

## Common tasks → first action
- Adding a CLI feature: edit `src/cli.rs` and `src/commands/mod.rs`, not these
  shims.
- Renaming public commands: update `Cargo.toml` `[[bin]]` entries and README
  install/usage examples together.

## Gotchas
- `include!("lsearch.rs")` means warnings/errors in the shared entrypoint may be
  reported for multiple binaries.
