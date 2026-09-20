# local-search website design contract

This document is the canonical visual, interaction, copy, and implementation
contract for `site/`. It translates the product principles in the root
`AGENTS.md` into decisions specific to the website. When a screenshot, a new
component, or a general design trend conflicts with this document, preserve the
local-search system unless the product direction is being changed intentionally.

## Product thesis

The browser on the developer's machine is the product asset. local-search is a
local browser API for agents: one CLI for searching the web, reading pages,
extracting records, interacting with sites, and using sessions in a dedicated
Chrome profile. It is the bridge between an agent command and that local browser,
not a search engine, hosted browser, or transparent network tunnel. The website
should make this category understandable in seconds and then prove it with the
real command surface, browser boundary, agent output, and reproducible benchmark
evidence.

The experience should feel like compact agent infrastructure: explicit, local,
inspectable, native, and fast. It should not look like a generic AI SaaS page.

## Design principles

1. **State the product before decorating it.** The primary claim is “A Local
   Browser API for Agents”; the immediate outcome is “Search. Read. Use Signed-In
   Sites.”
2. **Show the real mechanism.** Agent commands, the local browser boundary,
   search engines and sites, stable output, and measured proof are the visual
   subject matter.
3. **Stack the narrative.** The page progresses vertically through claim,
   installation, live agent proof, workflow, output, benchmarks, FAQ, and final
   action. Do not turn it into a collection of unrelated side-by-side cards.
4. **Use one construction system.** Rails, one-pixel borders, reticles, hatched
   spacers, deterministic dithers, and isometric signal paths should make every
   section feel like part of one measured technical drawing.
5. **Reserve color for meaning.** Warm white, black, and gray carry the page.
   Cyan/teal indicates local execution, signals, successful states, selected
   controls, and local-search's benchmark advantage.
6. **Prefer proof over claims.** Real agent traces, real normalized output, and
   committed benchmark data replace vague superlatives.
7. **Keep machine and human presentations distinct.** The website may use
   color, motion, syntax highlighting, links, and agent chrome; it must still
   represent the compact structured data that agents actually receive.
8. **Make density legible.** Complex evidence belongs in aligned diagrams,
   bordered bento cells, tables, or charts—not in long marketing paragraphs.

## Voice and copy

- Write short, direct sentences at the developer's level.
- Call local-search a “local browser API for agents.” Use “bridge” only to
  explain the relationship between an agent command and the browser. Avoid
  “tunnel,” which implies a transparent network proxy.
- Make clear that search is one capability. Mention reading pages such as Reddit
  or docs, extraction, interaction, or browser-authenticated requests before
  narrowing the story to search engines.
- Use sentence case. Avoid jargon when the command or output can demonstrate the
  point more clearly.
- Marketing section headings do not use eyebrow copy. Small labels are allowed
  inside technical panels when they identify a benchmark, stream, command, or
  output state.
- A primary section title may occupy at most two lines at every supported
  breakpoint.
- Diagram nodes use one short sentence. Put qualifications, methodology, and
  implementation details in adjacent explanation cells.
- Use the product name as `local-search`, the executable as `lsearch`, and the
  package names exactly as published.
- Say “no hosted search API key” or “no metered search bill” when precision is
  needed. Do not imply that public search engines, network access, or a browser
  are unnecessary.
- Describe signed-in state as sessions in the dedicated local-search browser
  profile. Do not imply silent access to the user's ordinary Chrome profile.
- Do not describe temporary, simulated, or decorative UI as a live search.
- Performance, cost, package-size, reliability, and provider comparisons must
  match committed evidence and include methodology or a link to it.
- Avoid filler such as “revolutionary,” “effortless,” “AI-powered,” or “built for
  the future.”

## Visual foundation

### Color tokens

The semantic tokens in `app/globals.css` are the source of truth:

| Token | Value | Role |
| --- | --- | --- |
| `--page` | `#f5f5f2` | warm site background |
| `--ink` | `#151515` | primary text and controls |
| `--muted` | `#6f6f69` | supporting copy |
| `--line` | `#d7d7d1` | rails, cells, and dividers |
| `--surface` | `#ecece7` | quiet secondary surfaces |
| `--accent-soft` | `#c9f1ef` | selected and local fields |
| `--accent` | `#168b90` | active signals and proof |
| `--accent-deep` | `#0d6f74` | high-contrast teal text |
| `--terminal` | `#171717` | terminal surfaces |
| `--terminal-line` | `#343434` | terminal separators |

- Do not add another marketing accent without revisiting the whole system.
- Keep third-party logos in their correct brand colors. Their color is
  identification, not an additional site palette.
- Use white and neutral surfaces without generic white-to-transparent fades.
- Do not use gradient orbs. Shader or paper textures may be used only as a
  restrained material field behind product proof, never as the product itself.

### Typography

- Manrope is the only site typeface, including code, commands, terminal chrome,
  charts, and agent recreations.
- Load the Latin variable font through `next/font`; do not add a second webfont.
- Headlines use tight tracking, controlled line height, and a deliberate weight
  contrast. The hero's first line is medium/strong; the cyan payoff is lighter.
- Use tabular numerals for benchmarks, timing, package size, cost, ranks, and
  step counts.
- Do not fake a monospace identity for commands. Hierarchy comes from syntax
  color, spacing, borders, and weight.

### Geometry and spacing

- The shared page frame is `min(calc(100% - 24px), 1360px)` and is centered.
- One-pixel borders form the default geometry. Adjacent elements must share a
  border rather than drawing two lines on top of one another.
- Corner reticles use the same apparent stroke weight as regular borders, remain
  visible but not black, and sit above neighboring surfaces in z-order.
- Hatched spacer bands separate major chapters. They are structural breathing
  room, not banners.
- Deterministic dot fields use the established 5, 8, and 13 pixel rhythms with
  elliptical masks and low contrast. Do not replace them with random noise or
  arbitrary background line art.
- Restrict rounded corners to interactive controls, terminal/product chrome,
  logo tiles, and the few surfaces whose physical metaphor needs them.
- Avoid a padded card floating inside another padded card. Dense proof should
  meet its cell edges.

## Page frame and navigation

- The side rails begin at the header and continue through every section and the
  footer. Sections align to the same frame.
- The sticky desktop header contains the `/ls` mark and local-search wordmark,
  centered anchor navigation, and a high-contrast GitHub action with the GitHub
  logo.
- Keep the header compact, evenly padded, and aligned to the rails. The header
  border and first spacer border must never double up.
- Hide the center navigation on narrow screens; retain the brand and primary
  repository action.
- All anchors require visible keyboard focus and a target offset that clears the
  sticky header.

## Hero

The hero is one integrated composition inside the rails:

- Copy occupies the left field on desktop. The isometric mechanism occupies the
  right field without creating a separate card.
- The headline is exactly two conceptual lines: `A Local Browser API for Agents`
  followed by the cyan `Search. Read. Use Signed-In Sites.` outcome.
- Supporting copy gives concrete examples: search Google, read Reddit or docs,
  and work with a site signed into inside the dedicated local-search browser.
  It ends with the simple mechanism: the browser does the work and the agent
  receives compact data.
- Primary actions appear in this order: View on GitHub, Agent Prompt, crates.io.
- Agent Prompt copies a useful instruction and confirms success in place; its
  changed width must not reflow surrounding content unexpectedly.
- The install control sits with the actions and proof, not in a detached card.
  Cargo is the default selection. Cargo and npm are two presentations of the
  same product, with the selector embedded at the left of the command row.
- The proof row uses real Rust, JSON, and Chromium/browser-state iconography for
  “Built with Rust,” “Agent-ready output,” and “Local browser sessions.”
- On small screens, hide the supporting paragraph before compromising headline,
  actions, install control, or diagram readability.

### Isometric hero artwork

- The scene shows four aligned octagonal engine slabs in this order from top to
  bottom: Google, Bing, Brave, DuckDuckGo.
- Logos are large, centered, brand-correct, and placed on lightly differentiated
  slabs. The plates should read as a coherent stack, not four unrelated cards.
- A dark local-search browser box with a flat `/ls` label sits below and to the
  left. The label has enough side padding to remain legible at small sizes.
- Four parallel isometric wires exit the box, travel below it, turn as one
  configuration, and connect to the right side of their corresponding slabs.
  Wires must be evenly spaced, avoid accidental crossings, and remain inside the
  artwork bounds.
- Ports, device lights, and hexagon clusters share the same isometric axes. The
  bottom hexagons must clear the wire bundle.
- A faint dither field integrates the artwork with the hero. There is no generic
  “agent” node and no decorative diagram label competing with the headline.
- At reduced widths the art may scale, crop softly, and reduce opacity, but the
  local box, slab stack, and connection idea must remain recognizable.

## Agent playground

The playground is the first full product proof and should feel like a real,
fixed-size terminal—not a slideshow inside a marketing card.

- The section background uses the established pale-cyan grain/paper field.
- Claude Code, Codex, and Cursor controls live inside the terminal header. Do not
  wrap the terminal in a second white shell or add redundant run/replay bars.
- The terminal keeps fixed desktop dimensions while content scrolls internally.
  New output auto-scrolls; it must never grow the page as a terminal trace runs.
- Prompts appear in the authentic prompt area of each agent, not in a separate
  row of scenario cards.
- The demo autoplays, completes one captured search, transitions smoothly to the
  next agent, and loops. Manual tab selection remains usable.
- Preserve each agent's recognizable visual language. Claude Code and Codex use
  the relevant MIT-licensed `theswerd/brainless` primitives; Cursor remains a
  locally maintained recreation because brainless does not ship one.
- Use actual agent logos. Codex and Cursor surfaces use their appropriate light
  presentation where required for contrast.
- Output represents real `lsearch` calls and current result shapes. Do not invent
  fields or display results that contradict the CLI.
- Links in human result views are clickable and keyboard reachable.
- Keep transition durations short enough to feel responsive. Use opacity and
  transforms with deliberate cubic-bezier curves; avoid layout animation and
  typewriter stutter.
- Reduced-motion mode stops decorative looping and presents a complete, usable
  state without hiding content.

## Metrics strip

- The summary strip is a single aligned row of compact facts on desktop and a
  responsive grid on smaller screens.
- Values stay on one line (`50.3 KiB`, not a broken number/unit stack).
- Treat package-size and startup figures as recorded release measurements, not
  universal guarantees.
- Keep the dither subtle enough that numbers remain the dominant signal.

## Workflow and JSON output

This chapter makes one argument: an agent calls one command, the local browser
acts on the real site, and compact data returns. Search is the concrete example;
the surrounding copy also names reading, extraction, interaction, and signed-in
browser sessions.

- The introduction, command, three-step circuit, field explanation, and JSON
  output form one bordered bento composition. Do not restore separate “how it
  works” and “output” sections.
- The circuit is one wide connected cyan dither field, not three independent
  cards. Agent, Browser, and Agent context nodes share three continuous signal
  paths.
- Step labels do not include `01`, `02`, or `03` prefixes.
- Keep the diagram phrases simple: “Call one command,” “Use the local browser,”
  and “Return compact data.”
- Desktop uses horizontal circuit geometry. Mobile uses a purpose-built vertical
  circuit; do not rotate or squeeze the desktop artwork.
- The command example must use supported current syntax.
- JSON is properly indented and syntax colored. Keys, strings, numbers,
  literals, and punctuation must be distinguishable without relying on color
  alone for meaning.
- Returned field chips explain the stable contract; longer schema and security
  details belong in prose or documentation links.

## Benchmark evidence

Benchmarks are evidence, not decoration.

- Use only committed results from `benchmarks/results/` and wording supported by
  `benchmarks/README.md`.
- Put methodology beside the chart it qualifies and link to the runner and
  machine-readable data. Record query count, result depth, token accounting,
  cache state, runtime/hardware context, and pricing date where relevant.
- The context comparison explicitly measures visible local-search command/output
  against rendered search-page snapshots. Do not shorten this to an ambiguous
  “browser” comparison.
- The hosted-provider comparison uses actual provider logos and exact provider
  names. Highlight local-search with the cyan field and `Best overall` treatment
  only while the committed evidence supports it.
- Compare aligned rows for normalized tokens per result, median latency, fulfilled
  depth, and 24-request usage. Cost or credits must stay aligned with the usage
  column and must not overflow at mobile widths.
- Horizontal bars for a metric share the same origin and scale. Bars within a
  comparison touch vertically where that makes the difference easier to read.
- Use light-to-dark neutral differentiation for comparison providers; reserve
  teal for local-search. Avoid arbitrary rainbow chart palettes.
- Vertical metric separators are dashed and restrained. Do not add decorative
  chart grid lines.
- Put “lower is better” in the relevant column title rather than as detached
  axis clutter.
- Footnote markers and their explanations remain visually paired under the
  comparison.
- Metrics use tabular numerals and include units. Pricing conversions must be
  sourced from current provider pricing rather than memory.

## FAQ

- FAQ content is a strict two-column bordered grid on desktop and one column on
  mobile. It should read like structured product documentation, not a set of
  floating accordion cards.
- Questions are direct and answers are short. Put deeper material in linked
  documentation.
- Visuals are low-opacity, three-dimensional logo plates positioned absolutely
  at the bottom right and allowed to crop at the card edge.
- A single-concept answer gets one large graphic.
- Only coding agents, search engines, and hosted API comparisons use multiple
  graphics. Four-item groups use a tight 2×2 arrangement with equal tile sizes
  and gaps.
- Use canonical local brand assets, sourced from official marks or thesvg.org as
  appropriate. Do not substitute generic line icons when the real Rust, API,
  agent, engine, or open-source mark communicates the idea.
- Visuals must never cover the question or answer. Reserve enough text space at
  every breakpoint.

## Closing action and footer

- The closing section returns to the dark foundation and repeats one clear
  action: install local-search or open the repository.
- Reuse the shared Cargo/npm install control; Cargo remains selected by default.
- The footer is minimal and uses real links for GitHub, crates.io, Rust/Cargo
  marks, licensing, and Kevin Liu. Keep link labels and external-link indicators
  aligned and prevent awkward wrapping.

## Motion and interaction

- Motion explains state changes, signal travel, agent transitions, or successful
  copy actions. It does not exist merely to keep the page moving.
- Prefer the established `cubic-bezier(.22, 1, .36, 1)` family for physical UI
  movement and a short ease for color/opacity feedback.
- Use `transform` and `opacity` for animated elements. Avoid properties that
  trigger page reflow during the agent demo.
- Isometric wire signals, workflow ports, and subtle grain may loop at a low
  amplitude. No ring spinners, bouncing ornaments, or parallax.
- Hover feedback is one pixel of lift at most. Focus must be at least as visible
  as hover.
- Controls should acknowledge input within 100 ms.
- `prefers-reduced-motion: reduce` must disable nonessential animation and leave
  every control, diagram, and output understandable.

## Responsive behavior

- The supported layout must remain readable from narrow mobile widths through
  the 1360 pixel content cap without horizontal page overflow.
- At tablet widths, simplify the header, reduce hero art prominence, and stack
  benchmark copy above its visual.
- At mobile widths:
  - hide the hero supporting paragraph;
  - stack the three hero actions and preserve a usable install selector;
  - keep the isometric hero visible at a smaller scale;
  - integrate the three agent tabs into a full-width terminal header;
  - keep the agent viewport fixed and internally scrollable;
  - switch the workflow to vertical circuit geometry;
  - turn benchmark tables into legible labeled rows without truncating units;
  - collapse FAQ to one column and reserve the lower card region for art;
  - keep the primary action above the first major scroll break where practical.
- Test long titles, command strings, cost values, and copied-state labels. Do not
  solve overflow by reducing body text below a readable size.

## Accessibility

- Preserve the skip link, semantic landmarks, heading order, table semantics,
  and descriptive section labels.
- Interactive elements use native links, buttons, inputs, and details whenever
  possible.
- Do not encode status or benchmark advantage with color alone; pair it with
  text, shape, position, or an icon.
- Decorative artwork is hidden from assistive technology. Informative images and
  social assets have concise alt text.
- Maintain readable contrast on warm white, cyan, dark terminal, and selected
  states.
- All agent tabs, copied links, install toggles, result links, and navigation
  anchors must work from a keyboard.
- Respect browser zoom, reduced motion, and `prefers-contrast` improvements where
  supported.

## Icons, logos, and assets

- `components/icons.tsx` and `components/brand-logo.tsx` are the canonical icon
  and `/ls` primitives. Extend them before introducing an icon dependency.
- Store third-party marks in `public/brand/`; do not hotlink production assets.
- Preserve brand proportions, colors, and clear space. Center marks optically,
  not only by their SVG view box.
- The local-search logo is the jagged `/ls` cut from a filled block. Keep enough
  horizontal padding inside the block and use the cyan version for the favicon
  and primary brand moments.
- Inline small deterministic SVG when it improves rendering and bundle size.
  Avoid shipping a general-purpose icon library.

## Social cards and README animation

- Open Graph and Twitter images are 1200×630, generated by
  `app/opengraph-image.tsx` and `app/twitter-image.tsx`.
- Their background matches `--page` and uses the site's dither field. The left
  side contains the claim and installation paths; the right side contains the
  isometric browser/engine artwork.
- Social copy is left aligned: `A Local Browser API` followed by the cyan `for
  Agents`, then `No API Key, No Billing`. Show four search-engine logos and four
  compatible-agent logos in aligned groups. Cargo appears above npm when both
  install commands are shown.
- Keep text and logos inside social-image safe areas. Never depend on a white
  gradient to make the composition readable.
- The README GIF is the 960×540 asset at
  `public/social/local-search-demo.gif`. It begins with real `lsearch` commands,
  uses explicit `--engine` values, shows current results with high-contrast
  syntax color, and ends on the branded `$0 Browser Search API` frame long enough
  to read.
- The GIF's terminal window keeps the macOS header and `local-search` title.
  Search-engine logos persist during a search and transition only between
  searches. The outro places four engine and four agent logos in balanced
  side-by-side groups.
- Update the README cache-busting query whenever the GIF changes so GitHub does
  not retain an older animation.

## SEO, GEO, and AEO

- `app/layout.tsx`, `lib/seo.ts`, JSON-LD, `manifest.ts`, `robots.ts`, and
  `sitemap.ts` own canonical metadata and machine-readable product facts.
- The page must render meaningful headings, explanations, FAQ answers, links,
  and demo preview content in initial HTML. Do not hide discoverable content
  behind client-only interactions.
- Keep `/llms.txt`, `/llms-full.txt`, and `/benchmarks.json` aligned with the
  human-facing page.
- Twitterbot is explicitly allowed to fetch `/api/og`, `/api/og-home`,
  `/opengraph-image`, and `/twitter-image`. Preserve that rule when changing
  crawler policy.
- Metadata and structured data must use the production `lsearch.dev` identity
  and the same package names, claims, measurements, and links as the page.

## Performance contract

- Use current stable Next.js and React releases; do not move production to a
  canary release for a visual feature.
- Keep the route, navigation, hero, diagrams, proof, FAQ, and marketing copy as
  Server Components.
- Hydrate only genuine interactions such as copy controls, install selection,
  and the agent playground.
- Keep a server-rendered playground preview in initial HTML so deferred
  JavaScript never creates an empty section or layout shift.
- Load the full agent playground and its supporting code only when appropriate;
  preserve its dimensions before hydration.
- Keep Manrope to one Latin variable font and let `next/font` self-host it.
- Prefer CSS, inline SVG, and optimized local assets over runtime drawing
  libraries. Shader use must be bounded and must degrade to a static field.
- Apply `content-visibility: auto` only to substantial below-the-fold sections
  with an accurate intrinsic size.
- The headline is the LCP candidate and must not wait for an entrance animation.
- Treat route build output, shipped client JavaScript, asset weight, hydration,
  layout shift, and horizontal overflow as release checks.

## Ownership map

- `app/page.tsx`: page narrative, semantic structure, and static proof.
- `app/globals.css`: tokens, construction system, responsive behavior, and
  motion.
- `components/agent-playground*`: interactive agent demo and faithful shells.
- `components/install-command.tsx`: shared Cargo/npm selector and command copy.
- `components/icons.tsx`: canonical inline icons.
- `components/brand-logo.tsx`: canonical local-search logo.
- `app/opengraph-image.tsx` and `app/twitter-image.tsx`: social compositions.
- `lib/seo.ts` and JSON-LD components: product facts and structured metadata.
- `public/brand/`: locally stored third-party brand assets.
- `public/social/local-search-demo.gif`: README and launch animation.

Keep changes in the owning layer. Do not solve a component-specific problem with
global CSS unless it is a reusable system rule.

## Prohibited patterns

- Generic gradient orbs, glassmorphism, testimonial carousels, pricing-card
  theater, or decorative AI sparkles.
- Arbitrary coding-agent boxes, fake search APIs, invented result data, or
  browser artwork unrelated to the local execution boundary.
- Multiple nested card shells around one product surface.
- Unaligned or crossing wires, mismatched isometric angles, and decorative line
  networks that do not communicate flow.
- More than one marketing accent, low-contrast gray-on-gray copy, or brand logos
  recolored for stylistic consistency.
- Ring spinners, stuttering typewriter effects, auto-growing terminals, or
  autoplay that steals focus.
- Claims without committed evidence, pricing without a dated source, and social
  cards whose facts differ from the page.

## Verification checklist

Before merging a meaningful site change:

1. Run `pnpm lint`, `pnpm typecheck`, and `pnpm build` from `site/`.
2. Inspect the desktop page at the content cap and at an intermediate laptop
   width.
3. Inspect a narrow mobile viewport for overflow, art/text collisions, terminal
   growth, FAQ cropping, and benchmark value wrapping.
4. Exercise navigation, all agent tabs, autoplay, result links, copied states,
   Cargo/npm selection, and reduced-motion mode.
5. Verify hover, focus, active, and disabled states with keyboard navigation.
6. Open `/opengraph-image` and `/twitter-image` and inspect the 1200×630 output.
7. If the GIF changed, decode it, inspect its first/search/outro frames, and bump
   the README cache key.
8. Confirm all benchmark, package-size, startup, cost, and engine claims against
   committed sources.
9. Check that reticles, rails, and adjacent borders render at one apparent pixel
   without double lines.
10. Run `git diff --check` and ensure no generated browser profiles, captures,
    secrets, or unrelated artifacts are included.
