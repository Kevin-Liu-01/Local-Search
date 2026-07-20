---
name: local-search-scripts-skill
description: Maintenance scripts for local-search. Read before editing scripts here.
---

# local-search / scripts

## Purpose
Small agent-callable wrappers around the Rust CLI for cleanup and maintenance.

## Mental model & key files
- `local-search-cleanup.sh` delegates to `lsearch cleanup`.
- Scripts should stay thin; command behavior belongs in Rust.

## Patterns to follow / invariants
- Dry-run by default for cleanup.
- Do not delete browser profile data or cookies from scripts.
- Prefer exact managed local-search targets over broad browser/process cleanup.

## Common tasks → first action
- Need to stop the managed browser: run `scripts/local-search-cleanup.sh --kill`.
- Need to inspect cleanup state: run `scripts/local-search-cleanup.sh --pretty`.

## Gotchas
- The managed Chrome profile is persistent by design. Cleanup removes lock and
  DevTools marker files, not profile data.
