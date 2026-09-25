---
name: site-lib-skill
description: Shared metadata, FAQ facts, structured data, and benchmark interpretation.
---

# Local-Search / site/lib

## Purpose
<!-- agent-docs:fill:purpose -->
Keep machine-readable and human-readable product facts consistent.

## Mental model & key files
<!-- agent-docs:fill:model -->
seo.ts supplies metadata, FAQ content, release measurements, and JSON-LD data.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
Match the actual CLI contract, README, SKILL.md, and public/llms*.txt. Keep
historical benchmark dates and qualifications; never recast them as current guarantees.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
When changing a FAQ question, update components/faq-visual's keyed mapping.
Run typecheck and build to verify server-rendered metadata.

## Gotchas
<!-- agent-docs:fill:gotchas -->
Existing Chrome authority requires Chrome approval for the retained connection;
macOS/Linux commands reuse it until disconnect or transport loss. Explicit
reconnection asks again. Cookie locality does not mean signed-in page output stays outside
the calling agent's context.
