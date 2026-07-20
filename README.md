# local-browser

Free structured search and browser automation through a local signed-in browser.

`local-browser` is a Rust CLI for agents that need web search, page scraping,
authenticated fetches, screenshots, MHTML, and light browser control without
paid search APIs. It talks directly to a local Chromium-family browser over the
Chrome DevTools Protocol, so the browser can reuse accounts you are already
signed in to.

## Why

The reference points were:

- [`browse.sh`](https://browse.sh/): a browser CLI for agents with primitives for
  page interaction, screenshots, console/network inspection, and structured
  fetch/search.
- [`agent-browser`](https://github.com/vercel-labs/agent-browser): local
  Chromium profile control, accessibility refs, cookies, network capture, and
  scriptable browser operations.
- [`_ontologic` on X](https://x.com/_ontologic/status/2078849784123977799):
  MHTML is useful when scraping DOM-heavy sites.
- [`thdxr` on X](https://x.com/thdxr/status/2078727284865827140): record browser
  network traffic and derive direct clients instead of replaying the UI every
  time.
- [`N0V4Dev` on X](https://x.com/N0V4Dev/status/2078064761766363183): native Rust
  browser tooling with accessibility-tree-like interaction is a good agent
  primitive.

## Install

```sh
cargo install --git https://github.com/Kevin-Liu-01/local-browser
```

Or from a checkout:

```sh
cargo build --release
./target/release/local-browser doctor
```

## Browser Setup

Chrome and Chromium-family browsers are supported through CDP. Enable local
remote debugging in Chrome, then run:

```sh
local-browser doctor
local-browser connect
```

You can also pass an endpoint explicitly:

```sh
local-browser --cdp 9222 doctor
local-browser --cdp ws://127.0.0.1:9222/devtools/browser/... tabs list
```

Safari is detected as an intentional limitation: its official WebDriver
automation uses isolated automation sessions, not the normal signed-in browsing
profile this project targets.

## Examples

Structured search:

```sh
local-browser search "site:docs.rs tokio Runtime" --engine google --limit 5 --pretty
```

Scrape a page:

```sh
local-browser open https://browse.sh/
local-browser read --format json --pretty
local-browser extract "a[href]" --field title=text --field url=href --limit 20 --pretty
```

Agent interaction loop:

```sh
local-browser snapshot --pretty
local-browser click @e3
local-browser fill "input[name=q]" "local browser search cli"
local-browser press Enter
```

Authenticated browser request:

```sh
local-browser request https://example.com/api/me --header "Accept: application/json" --pretty
```

Artifacts:

```sh
local-browser screenshot artifacts/page.png --full-page
local-browser mhtml artifacts/page.mhtml
local-browser record https://browse.sh/ --har artifacts/browse.har --mhtml artifacts/browse.mhtml
```

## Output Contract

Successful structured commands return:

```json
{ "ok": true, "...": "..." }
```

Failures are JSON on stderr:

```json
{ "ok": false, "error": { "code": "browser_not_found", "message": "..." } }
```

Human-readable `read --format markdown`, `read --format text`, and `html`
without a path write raw content to stdout.
