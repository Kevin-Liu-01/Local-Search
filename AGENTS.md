# local-browser Agent Notes

This is a standalone Rust CLI repo.

- Use `cargo fmt`, `cargo test`, and `cargo clippy --all-targets --all-features
  -- -D warnings` before committing behavior changes.
- Keep browser credentials out of fixtures and commits. Generated `artifacts/`,
  HAR, MHTML, screenshots, PDFs, and local target output are ignored.
- Prefer CDP browser-level WebSocket support over launching a separate browser:
  the project goal is normal-profile, signed-in local browser access.
- Do not add paid search/API dependencies for search. Search is performed in the
  user's local browser.

## Agent-docs (auto-maintained)

> Machine-derived facts maintained by `agent-docs`; do not hand-edit inside the markers.

<!-- agent-docs:auto:stack start -->
- **Name:** local-browser
- **Package manager:** unknown
- **Languages:** n/a
- **Framework:** n/a
<!-- agent-docs:auto:stack end -->

<!-- agent-docs:auto:commands start -->
- (no package.json scripts detected)
<!-- agent-docs:auto:commands end -->

<!-- agent-docs:auto:dirmap start -->
| Directory | Skill | Purpose |
|---|---|---|
| `src/` | [`src/SKILL.md`](src/SKILL.md) | How to work in `src/`. Read before editing here. |
| `src/browser/` | [`src/browser/SKILL.md`](src/browser/SKILL.md) | How to work in `src/browser/`. Read before editing here. |
<!-- agent-docs:auto:dirmap end -->

<!-- agent-docs:auto:env start -->
- (none detected)
<!-- agent-docs:auto:env end -->

<!-- agent-docs:auto:repo-graph start -->
- Use Graphify for repo topology, path/explain/affected questions, PR risk, and unfamiliar codebase orientation.
- Use `rg` for exact strings; use Kevin-Wiki `qmd` for people, tools, decisions, and compiled wiki knowledge.
- Use `agent-browser` for browser/UI work; use Playwright only for committed regression tests.
- Runtime memories (Hermes/Hindsight/Honcho) are not project truth until written back to AGENTS.md, SKILL.md, or the wiki.
- Status: `cd ~/Documents/GitHub/kevin-wiki && npm run graphify:sidecar -- status --run outputs/graphify/local-browser`
- Build from this repo: `PROJECT_ROOT="$(pwd)" && cd ~/Documents/GitHub/kevin-wiki && npm run graphify:sidecar -- build "$PROJECT_ROOT" --run outputs/graphify/local-browser --no-viz`
- Query after build: `cd ~/Documents/GitHub/kevin-wiki && npm run graphify:sidecar -- query "what should I inspect first?" --run outputs/graphify/local-browser`
- Never run Graphify installers/hooks or commit generated `graphify-out/` artifacts.
<!-- agent-docs:auto:repo-graph end -->
