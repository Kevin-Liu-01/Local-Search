---
name: src-skill
description: How to work in `src/`. Read before editing here.
---

# local-search / src

## Purpose
Application code for the CLI, including argument parsing, command dispatch,
typed errors, browser control, and output rendering.

## Mental model & key files
- `src/bin/lsearch.rs` parses Clap arguments and turns typed errors into stable
  stderr JSON.
- `lib.rs` exposes the async `run` entry point.
- `cli.rs` is declarative command/argument structure.
- `commands/mod.rs` is orchestration: attach browser, execute command, print
  result.
- `error.rs` keeps operator-facing messages and machine-readable error codes in
  one place.

## Patterns to follow / invariants
- Keep command handlers thin enough to read from top to bottom.
- Prefer typed `Error` variants over ad hoc strings.
- Any command that may expose browser credentials should redact by default or
  require an explicit opt-in flag.
- Do not put CDP JSON protocol plumbing in command handlers when it belongs in
  `src/browser/`.

## Common tasks → first action
- New command: add Clap args, add command branch, update README, add CLI test.
- Managed browser cleanup: keep `cleanup` dry-run by default, kill exact managed
  listener PIDs only, and never delete profile cookies/data.
- New output mode: update `output.rs` only if the envelope contract changes.
- New error: add variant in `error.rs` and stable `code()` mapping.

## Gotchas
- `run` owns the parsed `Cli`; avoid async closures borrowing `&mut CdpClient`
  across helper boundaries unless lifetimes are explicit.
- `read --format text|markdown` and `html` without a path intentionally write raw
  body content instead of JSON.
