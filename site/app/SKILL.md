---
name: site-app-skill
description: Landing-page composition, global CSS, metadata, and social-card routes.
---

# Local-Search / site/app

## Purpose
<!-- agent-docs:fill:purpose -->
Compose server-rendered product content and crawlable routes.

## Mental model & key files
<!-- agent-docs:fill:model -->
page.tsx assembles the narrative; globals.css defines the design tokens and
responsive layout. layout.tsx and metadata routes own search/social presentation.
theme.css owns dark surfaces and the theme control; layout runs the saved-theme
initializer before paint without making the page client-only.
docs/page.tsx and docs/docs.css own the task-first /docs guide. Keep its commands
and release boundary aligned with ../../docs/agent-guide.md. Homepage JSON-LD
lives in page.tsx; docs emits its own TechArticle, not the homepage FAQ schema.
docs/docs-diagrams.tsx owns the responsive, server-rendered workflow visuals.
Keep the default reading path short; secondary commands and troubleshooting
belong in named native disclosures. Full agent docs must remain available.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
Follow ../DESIGN.md. Reuse existing logos and copy controls. Browser setup must
show two explicit choices with permission, persistence, and disconnection behavior.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
Inspect the owning component before changing global CSS. Run the website check
matrix and visually inspect at desktop, tablet, and narrow mobile widths.

## Gotchas
<!-- agent-docs:fill:gotchas -->
Do not put client-only wrappers around important prose or shrink supporting text
to fit. Generated out/ and .next/ are not source files.
