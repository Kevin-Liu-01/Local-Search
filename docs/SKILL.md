---
name: local-search-docs
description: Illustrated agent workflows, screenshot sources, and dated feature evidence.
---

# local-search / docs

## Purpose
<!-- agent-docs:fill:purpose -->
Explain what the user approves and what the agent can do through one local CLI.
Keep the guide shorter than the complete root command reference.

## Mental model & key files
<!-- agent-docs:fill:model -->
- agent-guide.md owns the quick walkthrough and example commands.
- visual-guide.html owns the seven screenshot compositions.
- images/ contains only authored, public-safe captures.
- verification.md records evidence, source provenance, and release boundaries.
- fixtures/tasks.html is synthetic, contains no credentials, and never submits externally.
- capture-browser.json forces a fresh capture configuration; the capture script
  additionally isolates its session and permits only localhost.
- site/app/docs/page.tsx presents the curated guide at /docs. Keep commands,
  release notices, and permission boundaries aligned with agent-guide.md.
- site/scripts/sync-docs.mjs copies the seven public-safe images and Markdown
  references into ignored site/public/docs-assets/ before dev/build. Edit the
  canonical sources here or the root SKILL.md, never those generated copies.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
Use compact JSON and bounded reads for agents. Keep approval, private page
content, and task-specific action authority separate. Never imply the CLI
enforces the agent's per-action permission policy. No account screenshots,
cookies, tokens, personal feed posts, or fabricated results. Label illustrations,
synthetic fixtures, dated records, shortened output, and unreleased features.
Preserve the site's Manrope, teal, large text, and local brand assets.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
Run node scripts/capture-agent-guide.mjs to regenerate visuals and check layouts.
It requires agent-browser and a prior site build for the bundled font. Use
--serve to inspect locally, then stop the server. Check all Markdown links,
CLI syntax, image legibility, and the release notice before publishing.

## Gotchas
<!-- agent-docs:fill:gotchas -->
docs/ is excluded from the native crate. Links in crates.io documentation must
resolve to GitHub, not a nonexistent packaged screenshot. Capture generation
must not require access to the user's signed-in browser. New raw account data
must not enter fixtures or generated images.
