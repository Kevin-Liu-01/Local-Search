---
name: site-components-skill
description: Interactive agent demos, browser workflow, setup/copy controls, and brand primitives.
---

# Local-Search / site/components

## Purpose
<!-- agent-docs:fill:purpose -->
Show the CLI-to-browser-to-output relationship using recognizable, readable UI.

## Mental model & key files
<!-- agent-docs:fill:model -->
browser-workflow owns one synchronized agent/browser demo and its shared timeline.
agent-playground-interactive presents the selected agent without owning a timer.
Both panes use the same recorded trace. install-command and copy-command share controls.
browser-connection explains existing Chrome approval versus a persistent separate
profile, with visible commands and native setup disclosures.
icons and brand-logo own iconography; faq-visual maps collected assets by question.
theme-toggle owns persisted light/dark selection and follows system changes only
until the visitor chooses a theme. lib/theme.ts initializes it before first paint.

## Patterns to follow / invariants
<!-- agent-docs:fill:patterns -->
Preserve keyboard semantics, reduced motion, readable states, fixed demo heights,
and completed server previews. The demo loops continuously without pause controls
or interaction-triggered stops. Stop timers offscreen and resume automatically.
Illustrations never access a visitor's browser credentials.

## Common tasks → first action
<!-- agent-docs:fill:tasks -->
When browser setup changes, update agent-copy-button and browser-connection together.
Test copied command text, state feedback, and mobile wrapping.
For theme changes, check both palettes, keyboard activation, reload persistence,
cross-tab sync, blocked storage, and the header at 320px. Run pnpm test.

## Gotchas
<!-- agent-docs:fill:gotchas -->
Do not replace collected SVGs with approximations. Never instruct agents to run
both browser-choice commands sequentially or silently recover in another profile.
