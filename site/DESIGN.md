# local-search website design contract

This document is the canonical visual, interaction, copy, and implementation
contract for `site/`. It translates the product principles in the root
`AGENTS.md` into decisions specific to the website. When a screenshot, a new
component, or a general design trend conflicts with this document, preserve the
local-search system unless the product direction is being changed intentionally.

## Product thesis

The browser on the developer's machine is the product asset. local-search is a
local browser API for agents: one CLI for searching the web, reading pages,
extracting records, interacting with sites, and using sessions in the local
Chrome profile the user explicitly chooses. It is the bridge between an agent command and that local browser,
not a search engine, hosted browser, or transparent network tunnel. The website
should make this category understandable in seconds and then prove it with the
real command surface, browser boundary, agent output, and reproducible benchmark
evidence.

The experience should feel like compact agent infrastructure: explicit, local,
inspectable, native, and fast. It should not look like a generic AI SaaS page.

## Design principles

1. **State the product before decorating it.** The primary claim is “A Local
   Browser API for Agents”; concrete examples are searching the web, reading
   pages, and using approved existing Chrome logins or a separate profile.
2. **Show the real mechanism.** Agent commands, the local browser boundary,
   search engines and sites, stable output, and measured proof are the visual
   subject matter.
3. **Stack the narrative.** The page progresses vertically through claim,
   installation, one paired agent/browser demo, benchmarks, FAQ, and final
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
8. **Readability wins over density.** Supporting text never drops below 16px.
   Remove a label or simplify a composition before shrinking its text. Let a
   diagram do the explaining; put qualifications in an accessible disclosure.

## Voice and copy

- Write short, direct sentences at the developer's level.
- No em dashes in visitor-facing copy, metadata, demo text, or copied prompts.
  Use a period, comma, or colon. Never alter CLI flags to match prose style.
- Give each section one clear point. Keep card descriptions to one or two short
  sentences and put setup steps in the adjacent disclosure. Cut repetition
  before reducing text size; keep approval, privacy, and benchmark caveats clear.
- Call local-search a “local browser API for agents.” Use “bridge” only to
  explain the relationship between an agent command and the browser. Avoid
  “tunnel,” which implies a transparent network proxy.
- Make clear that search is one capability. Mention reading pages, extraction,
  interaction, or using signed-in sites before narrowing to search engines.
  Use a range of recognizable examples, not one site as the whole product story.
- Use sentence case. Avoid jargon when the command or output can demonstrate the
  point more clearly.
- No marketing eyebrows, tiny uppercase labels, decorative step numbers, or
  repetitive subheadings. Technical labels must earn their space and remain
  at least 16px, including inside demos and charts.
- Keep headings to two lines where practical; a natural third line on narrow
  phones is preferable to shrinking the type or clipping it.
- Diagram nodes use one short sentence. Put qualifications, methodology, and
  implementation details in adjacent explanation cells.
- Use the product name as `local-search`, the executable as `lsearch`, and the
  package names exactly as published.
- Say “no hosted search API key” or “no metered search bill” when precision is
  needed. Do not imply that public search engines, network access, or a browser
  are unnecessary.
- Describe browser authority as an explicit choice: existing Chrome with Chrome
  approval, or a separate persistent profile with its own sign-in. Never imply
  silent access, cookie exporting, per-site consent, or approval that lasts forever.
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
| `--muted` | `#5e625c` | high-contrast supporting copy |
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
- Support light and dark page themes. Light keeps the warm paper palette; dark
  uses deep charcoal-green surfaces, off-white text, and brighter teal accents.
  Theme tokens and surface-specific overrides live in `app/theme.css`.
- Put the sun/moon toggle immediately before GitHub and crates.io. It must have
  an accessible action label and at least a 44px hit target. On the narrowest
  phones, retain the brand mark instead of squeezing the wordmark and controls.
- On first visit follow the device preference. Persist explicit choices locally,
  apply them before first paint, and synchronize other tabs. Storage failures
  must not break the page or toggle. Do not add a page-wide color animation.
- The toggle is a sun/moon glyph without an outer circle or orbital track.
  A shared masked disc morphs from sun to crescent; eight rays retract, and three
  stars appear in sequence. The glyph represents the current theme; the accessible
  label names the action. Pointer/touch changes use interruptible transform and
  opacity transitions (480ms eclipse). This intentionally longer,
  occasional delight never delays the actual theme change. Initial load, keyboard
  activation, and reduced motion settle immediately. No looping or extra library.
- Keep logos recognizable and adapt only monochrome marks to their backgrounds.
  The simulated browser follows the page theme; the agent shells keep their
  native colors, including Cursor's scoped light palette. Social images remain
  deterministic and independent of the visitor's preference.
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
- Body copy is 18–20px. Navigation, controls, chart values, footnotes, and demo
  details have a 16px floor. Section titles scale from 30–54px and the desktop
  hero reaches 62px. Never shrink text to make a layout fit.
- Native SVG logos are exempt from the text floor; raster graphics containing
  small labels are not a substitute for readable HTML evidence.

### Geometry and spacing

- The centered frame is `min(calc(100% - 48px), 1360px)` on desktop and
  `calc(100% - 24px)` on phones. Shared section gutters scale from 22–56px.
- Major sections use 48–88px vertical padding. Separate a heading, its visual,
  and its evidence with intentional space rather than nested padded cards.
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
  centered anchor navigation, and GitHub followed by crates.io at the right.
  GitHub is the high-contrast action; crates.io uses its collected package icon
  and a quiet outlined treatment. Keep both destinations on mobile.
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
- The headline is `A local browser API` followed by the teal `for your agents.`
- Supporting copy names the actions: search, read pages, and use signed-in sites
  through the user's browser. Keep approval explicit. The paired demo shows
  the mechanism and the compact data returned to the agent.
- Hero actions appear in this order: View on GitHub, Copy Agent Prompt. The
  crates.io link lives beside GitHub in the top-right header instead.
- Copy Agent Prompt copies a useful instruction and confirms “Copied!” in place;
  a screen-reader status adds the instruction to paste it into an agent.
- The install control sits with the actions and proof, not in a detached card.
  Cargo is the default selection. Cargo and npm are two presentations of the
  same product, with the selector embedded at the left of the command row.
- The install selector uses a pale blue track and a white sliding selected tab.
  Use equal cell widths, 5px track insets, and symmetric 12px button padding.
  Keep the full Cargo/npm names and logos readable; stack above the command
  when space runs out rather than squeezing the selected label.
- The proof row uses real Rust, JSON, and Chromium/browser-state iconography for
  “Built with Rust,” “Compact JSON,” and “Your browser, your choice.”
- Keep the short supporting paragraph visible on mobile: it explains the
  premise. Stack controls and the art instead of hiding the explanation.
- Give desktop text and artwork separate grid columns. They must not overlap.

### Isometric hero artwork

- The scene shows four aligned octagonal engine slabs in this order from top to
  bottom: Google, Bing, Brave, DuckDuckGo.
- Logos are large, centered, brand-correct, and placed on lightly differentiated
  slabs. The plates should read as a coherent stack, not four unrelated cards.
- A dark local-search browser box with a flat `/ls` label sits below and to the
  left. The label has enough side padding to remain legible at small sizes.
- The box's top-face mark starts with Claude Code. Clicking or tapping the box
  cycles through Claude Code, Codex, Cursor, and OpenClaw. Keep each mark on the
  same isometric plane, inside the perimeter groove and clear of the lights.
  Use a native button with an accessible current/next-agent label, a box-shaped
  focus ring, and Enter/Space support. Crossfade marks on pointer input; keyboard
  and reduced-motion changes are immediate. The rest of the diagram stays still.
- Four parallel isometric wires exit the box, travel below it, turn as one
  configuration, and connect to the right side of their corresponding slabs.
  Wires must be evenly spaced, avoid accidental crossings, and remain inside the
  artwork bounds.
- From top to bottom, box ports connect to DuckDuckGo, Brave, Bing, then Google.
  Route the lowest engine along the innermost return path and Google along the
  outermost path. Validate intersections; do not hide crossed wires with masks.
- Base strokes and animated signals use one shared set of path coordinates.
- Ports, device lights, and hexagon clusters share the same isometric axes. The
  bottom hexagons must clear the wire bundle.
- A faint dither field integrates the artwork with the hero. There is no generic
  “agent” node and no decorative diagram label competing with the headline.
- At reduced widths the art may scale and crop slightly, but the
  local box, slab stack, and connection idea must remain recognizable.

## Paired agent and browser demo

One section explains the mechanism and shows it inside the agent. Keep the
heading “Your agent asks. Your browser answers.” Do not restore a separate
“See it in your agent” section or a second generic terminal demonstration.

- Place the selected agent on the left and its simulated local Chrome window
  on the right. Claude Code, Codex, and Cursor tabs live in the agent header,
  with no Pause/Play control. Do not add scenario pills or another playback bar.
- One controller owns the query and timeline for both panes: prompt, command,
  browser navigation, search results, then an agent answer. Switching agents
  changes the complete example; neither pane may show stale results.
- Use the three recorded DuckDuckGo traces from `lib/traces.ts`. Show current
  explicit syntax: `lsearch "<query>" --engine duckduckgo --limit 3 --json`.
  Do not relabel the recorded results as Google, Bing, or Brave.
- Preserve each agent's recognizable visual language. Claude Code and Codex use
  the MIT-licensed `theswerd/brainless` message and tool primitives; Cursor uses
  the maintained light agent-pane recreation. Omit sidebars and decorative
  composers that compete with the paired browser for space.
- Human-readable result links are visible by default; native tool disclosures
  expose the full JSON. Both panes must use identical result titles and URLs.
  The browser adds the recorded snippets, while the agent presents a compact list.
- Desktop panes align at 660px high and scroll internally. At 800px and below,
  join the agent directly above the browser: a 540px agent pane (560px on phones)
  and a 560px browser pane. Never grow the page as output arrives.
- Supporting text, commands, browser chrome, and provenance stay at least 16px.
  Keep tabs comfortably tappable. Use the correct collected
  agent logos, including high-contrast Codex and Cursor marks on dark chrome.
- The simulated search completes in 1.1 seconds: command at 180ms, navigation at
  380ms, browser results at 750ms, and agent results at 1100ms. The cycle advances
  at 6 seconds, keeping most of the time for reading. These are presentation
  timings, not performance claims. A short dashed connection joins the windows;
  do not restore the circular arrow button between them.
- Autoplay holds the finished result before advancing to the next agent and keeps
  looping. Scrolling, touch, focus, result links, and tool disclosures must never
  stop playback. Suspend timers only offscreen or in a hidden tab; resume
  automatically when visible again.
- Tabs support arrow keys, Home, End, Enter, and Space. Keyboard selection shows
  a completed example, then rejoins the loop. Reduced motion shows static results without
  autoplay. Immediate selections must not animate result changes.
- Initial HTML contains a complete example, with meaningful prompt, command, and
  results. Hydration must not introduce a blank preview or collapse the windows.
- Show “Simulation” in the agent footer and the recorded-results date below the
  pair. Keep attribution to brainless. This is not a live search or a connection
  to the visitor's browser.
- Four supported-engine logos beneath the demo are informational, not buttons.
  Preserve `#demo` and `#output` links in the combined section.

## Supporting measurements

- Do not restore a second strip of tiny repeated statistics. The benchmark
  chart owns search measurements; recorded binary size and warm startup live
  in the FAQ with the audit date and platform.
- Keep values and units together. Measurements are recorded evidence, not
  universal guarantees.

## Browser choice and setup

The demo shows search; the hero, browser-choice comparison, and FAQ explain reading,
extraction, interaction, and signed-in browser access beyond that example.

- Use “Choose your browser.” below the demo, followed by the choice in one short
  sentence: current logins or separate agent work. The existing-Chrome card
  explains login reuse; its illustration shows that Chrome asks for approval.
- Two illustrated options compare existing Chrome (`lsearch connect --existing`)
  with a separate persistent profile (`lsearch connect --managed`). The existing
  browser shows signed-in sites; the separate profile shows two distinct browser
  identities. Use collected Chrome and local-search marks, not generic cloud art.
- Show a range of example sites in the existing-browser illustration: Communities
  (Reddit, LinkedIn, Discord), Docs (GitHub, Notion, Google Docs), and Work apps
  (Slack, Linear, Figma). Use locally stored, brand-correct SVGs with accessible
  names. Category tiles become horizontal rows in narrow cards; never squeeze
  labels or imply a verified integration with a per-site checkmark.
- Keep both commands visible. Put detailed setup instructions in a native
  disclosure within each option, including Chrome 144+, remote-debugging settings,
  and explicit approval. Stack the options on phones without shrinking text.
- State the guarantee once: the choice is remembered and a disconnected browser
  is reported, never replaced silently. On macOS/Linux one approved existing-Chrome
  connection is reused between commands; `lsearch disconnect` ends access without
  closing Chrome. Reconnection is explicit and requires Chrome approval again.
- Existing logins require Chrome approval; separate profiles require signing in
  there. Sites still control access. Never imply visiting this website reads
  cookies, connects a browser, or executes a command.
- Command strings may wrap at narrow widths but must remain fully copyable.
  Schema details belong in the expandable demo output and documentation.

## Benchmark evidence

Benchmarks are evidence, not decoration.

- Use only committed results from `benchmarks/` and wording supported by
  `benchmarks/README.md`.
- Put methodology beside the chart it qualifies and link to the runner and
  machine-readable data. Record query count, result depth, token accounting,
  cache state, runtime/hardware context, and pricing date where relevant.
- The context comparison explicitly measures visible local-search command/output
  against interactive page snapshots, not screenshot image tokens. Keep this
  distinction even if an older launch graphic calls them screenshots.
- The hosted-provider comparison uses actual provider logos and exact provider
  names. Highlight local-search with the cyan field, not superlative badges.
- Use three large metric selectors for Context, Latency, and API cost. Each shows
  local-search's recorded value and unit. Below, show all five providers in one
  shared-scale horizontal chart. Keep the white field, teal local-search row,
  and neutral provider bars; do not squeeze three metric columns into a phone.
- A provider's value and its ratio to local-search sit beside the bar on desktop.
  On phones, put provider and value above a full-width bar. Preserve readable
  units in the chart heading and the zero-to-maximum scale. A zero cost uses a
  baseline marker, not a fake minimum-width bar. All values remain in a native
  methodology disclosure with a full data table.
- Keep the 96.5% context comparison in its own quiet pale field: a large readable
  figure beside two shared-scale bars for 309 and 8,760.5 visible tokens. Stack
  the figure and bars on mobile. This is not the hosted-provider metric.
- Horizontal bars for a metric share the same origin and scale. Give each
  provider enough separation to keep its label, value, and bar unambiguous.
- Use light-to-dark neutral differentiation for comparison providers; reserve
  teal for local-search. Avoid arbitrary rainbow chart palettes.
- Separate provider rows with quiet rules. Do not add decorative chart grid lines.
- Put “lower is better” in the selected metric description rather than as detached
  axis clutter.
- Footnote markers and their explanations remain visually paired under the
  comparison.
- Metrics use tabular numerals and include units. Label historical pricing with
  its recorded date; new pricing claims need freshly verified sources.
- Keep the cache caveat visible: 148.7ms combines cold and cached searches.
  Provide 384.5ms cold / 6.5ms cached, token accounting, workload, and links to
  committed evidence in the adjacent native details disclosure.
- On phones, use labeled provider rows instead of a squeezed desktop table.

## FAQ

- FAQ content is a single full-width native details list with shared horizontal
  rules. Answers remain in server-rendered HTML and open with keyboard input.
- Questions are direct and answers are short. Put deeper material in linked
  documentation.
- Reuse the collected SVG assets from `public/brand/` beside each question.
  Product, browser, credentials, Rust, and open-source marks get a single slot;
  agent and search-engine logos keep their two-by-two groups. Map artwork by
  question rather than array index, so content changes cannot mislabel a visual.
- Keep the artwork in its own fixed-width grid column, separate from both the
  text and chevron. Do not crop, fade, or position it behind the answer. Scale
  the illustration column on mobile, never the question text below 18px.
- Artwork remains decorative and visible whether the answer is open or closed.
  Only the disclosure chevron rotates; inline brand SVGs must stay still.

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
  - use a single 68px header row: wordmark and two accessible, 44px icon links;
  - retain the short hero explanation, a stronger headline, and side-by-side
    GitHub / Copy Agent Prompt actions; shorten labels, never reduce type to fit;
  - put the full-width Cargo/npm selector above its copyable command, keeping
    command words together where possible and wrapping long URLs safely;
  - keep the complete isometric hero in normal flow with bottom breathing room;
    fixed artwork heights must not crop the lowest wires or hexagons;
  - integrate visible agent logos and labels into the terminal header without
    a pause button; keep the viewport internally scrollable
    and bounded by the phone's available height;
  - keep agent selection in the window header; connect the readable agent
    to the browser beneath it, with fixed heights and no
    page growth when results arrive;
  - keep all five benchmark providers visible in every metric view; use
    full-width bars below provider/value pairs, never tiny bars inside narrow
    table cells; preserve units and the full recorded values disclosure;
  - put compact FAQ illustrations to the left of 18px questions and keep
    expanded answers full width;
  - keep the primary action above the first major scroll break where practical.
- Verify 320, 360, 390, and 430px phones, 768px tablet, and 1024 / 1440px desktop.
  Exercise both installers, all simulated searches and agent tabs, all benchmark
  metrics, expanded FAQs, keyboard navigation, and reduced motion.
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
  the paired demo, and benchmark metric selection.
- The paired demo is a bounded client island with a completed server-rendered
  initial state. Preserve its dimensions before hydration and gate its timers
  on viewport visibility, document visibility, and motion preferences.
- Keep the shared demo controller separate from agent-shell presentation; do not
  ship another hidden or independently animated copy of the same demo.
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
- `app/theme.css`: light/dark control, dark palette, and surface overrides.
- `components/theme-toggle.tsx` and `lib/theme.ts`: persisted theme selection,
  device preference, cross-tab sync, and pre-paint initialization. Run `pnpm test`
  for initialization regression checks as well as browser checks of both themes.
- `components/agent-playground-interactive.tsx`: controlled agent-shell presentation.
- `components/benchmark-comparison.tsx`: responsive provider and context charts.
- `components/browser-workflow.tsx`: shared agent/browser timeline, selection, and Chrome simulation.
- `components/browser-connection.tsx` and its CSS: signed-in browser comparison and setup instructions.
- `components/hero-agent-switch.tsx`: the hero box's accessible agent-logo cycle.
- `components/product-proof.css`: simulation and benchmark visual/responsive rules.
- `components/install-command.tsx`: shared Cargo/npm selector and command copy.
- `components/icons.tsx`: canonical inline icons.
- `components/brand-logo.tsx`: canonical local-search logo.
- `app/opengraph-image.tsx` and `app/twitter-image.tsx`: social compositions.
- `lib/seo.ts` and JSON-LD components: product facts and structured metadata.
- `public/brand/`: locally stored third-party brand assets.
- `public/social/local-search-demo.gif`: README and launch animation.

Keep changes in the owning layer. Do not solve a component-specific problem with
global CSS unless it is a reusable system rule.

## Documentation at /docs

Keep the same header, theme, type, and structural rails as the product page.
Use a sticky section index on desktop and a native "On this page" disclosure on
mobile. Setup and tasks come before the exhaustive agent reference. Body copy
is 18px or larger; controls, code, captions, and navigation are at least 16px.
Commands must be selectable, copyable, and able to wrap on a 320px screen.

Use short, single-line titles without clipping text or shrinking mobile type.
Section headings and navigation share the same labels. Lead each section with
a task name, one purpose sentence, a diagram where useful, and the main command.
Use simple technical English: "read a page", "choose a browser", "extract data".
Keep extra flags, setup steps, and errors in one named native disclosure per
section. Avoid repeated callout boxes or extra rows that interrupt this flow.
The complete Markdown guide and command reference remain available for agents.

docs/docs-diagrams.tsx uses existing brand icons and live HTML text for the
agent/browser flow, signed-in reading, shared search fields, extraction, page
actions, and disconnection. Connections must stack cleanly on small screens.
Do not replace these with fixed-size screenshots or hide the primary flow.

The canonical walkthrough and seven screenshots live in ../docs/. Group them
in one "Screenshots" disclosure at the end, with full-size links. Show dates, synthetic-data
labels, permission boundaries, and the release status. Never use raw private
account screenshots. Do not claim that diagrams are live browser sessions.

The build syncs approved screenshots and Markdown into public/docs-assets/.
Keep those generated copies out of Git. Run pnpm test:docs after pnpm build,
then verify mobile/desktop in both themes and check copy/download behavior.

## Prohibited patterns

- Generic gradient orbs, glassmorphism, testimonial carousels, pricing-card
  theater, or decorative AI sparkles.
- Arbitrary coding-agent boxes, fake search APIs, invented result data, or
  browser artwork unrelated to the local execution boundary.
- Multiple nested card shells around one product surface.
- Unaligned or crossing wires, mismatched isometric angles, and decorative line
  networks that do not communicate flow.
- Sub-16px supporting text, marketing eyebrows, more than one accent,
  low-contrast gray-on-gray copy, or brand logos
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
