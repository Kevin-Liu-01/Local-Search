---
name: src-browser-skill
description: How to work in `src/browser/`. Read before editing here.
---

# local-search / src/browser

## Purpose
Browser transport and runtime scripts for local signed-in browser control.

## Mental model & key files
- `discovery.rs` finds a browser-level CDP websocket from `--cdp`, saved config,
  `DevToolsActivePort`, or common localhost ports.
- `cdp.rs` is a minimal browser-level CDP client using flattened target sessions.
- `scripts.rs` stores JavaScript snippets evaluated in the page for snapshots,
  interactions, extraction, readable content, and in-browser fetch.

## Patterns to follow / invariants
- Connect to the browser websocket, then attach to a page target with
  `Target.attachToTarget` and `flatten: true`.
- Keep JavaScript snippets deterministic and return JSON-serializable values.
- Resolve `@eN` refs through `data-local-browser-ref`; `snapshot` is what assigns
  those refs.
- Discovery should prefer existing browser endpoints over launching browsers.

## Common tasks → first action
- Add a CDP primitive: implement it on `CdpClient`, then call it from commands.
- Add a DOM operation: add a snippet function in `scripts.rs` and JSON-test the
  output through a real browser page.
- Add browser discovery: extend `devtools_files()` or explicit endpoint parsing.

## Gotchas
- `/json/version` can return 404 on newer Chrome normal-profile debugging; the
  `DevToolsActivePort` browser websocket is the reliable path.
- `Runtime.evaluate` needs `returnByValue` for stable CLI JSON.
- HAR recording currently stores raw Network events in a HAR-shaped envelope; do
  not claim full browser-export HAR parity without adding response body capture.
