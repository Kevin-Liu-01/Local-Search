---
name: site-skill
description: Next.js landing page, browser workflow, benchmarks, and social assets.
---

# Local-Search / site

## Purpose
<!-- agent-docs:fill:purpose -->
Present the local browser API clearly, with readable product proof and explicit
browser authority. Read DESIGN.md before visual or marketing changes.

## Mental model & key files
<!-- agent-docs:fill:model -->
app/ owns routes, metadata and global CSS; components/ owns interactive displays;
lib/ owns shared product facts; public/ owns collected logos and launch media.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
Keep existing Chrome approval distinct from a separate persistent profile.
Do not imply automatic login access or credentials remaining outside agent
context when returned pages contain private data. Keep supporting text ≥16px.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
Run pnpm lint, pnpm typecheck, pnpm build. Verify desktop/mobile through
agent-browser. Use the committed benchmark evidence for numeric claims.

## Gotchas
<!-- agent-docs:fill:gotchas -->
The site is a static export: serving out/ requires rebuilding after edits.
Do not publish the site or packages without explicit user authorization.
