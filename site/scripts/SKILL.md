---
name: local-search-site-scripts
description: Deterministic public documentation asset sync and static-export checks.
---

# Site scripts

## Purpose
Keep the deployed documentation tied to canonical repo sources, without a new
runtime dependency or a second copy of screenshots in Git.

## Files
- sync-docs.mjs runs before dev/build. It copies an explicit list of seven
  authored screenshots and rewrites relative Markdown links for the web.
- check-docs.mjs checks the finished static export. Run pnpm test:docs after build.

## Boundaries
Never glob-copy artifacts, profiles, account captures, or private output into
public/. Generated public/docs-assets/ is ignored. Add a public-safe image only
after checking its provenance and inspecting it. Keep /docs-assets separate from
/docs so the static directory cannot shadow the page route.

## Verification
Run pnpm build and pnpm test:docs. Check Markdown/image URLs in the browser and
preserve homepage metadata separately from the docs TechArticle.
