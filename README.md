# local-search

Free structured web search for agents, powered by your local browser.

`local-search` gives agents Exa/Firecrawl/Brave/Tavily-style search, read, and
extract outputs without an API key or metered search bill. The primary CLI is
`lsearch`. It searches through a local Chrome/Chromium profile, so it can use the
same public web, logged-in sessions, cookies, and regional results you can see in
the browser.

Browser automation is implementation support, not the product category. The
wedge is local, free, structured search for agents.

## Install

```sh
cargo install local-search
```

Until the crates.io release has propagated, install directly from GitHub:

```sh
cargo install --git https://github.com/Kevin-Liu-01/local-search
```

From a checkout:

```sh
cargo install --path . --force
lsearch doctor --pretty
```

Compatibility binaries are also installed:

```sh
local-search --help
local-browser --help
```

## Quick Start

Start the managed browser profile once:

```sh
lsearch launch
```

This opens a persistent Chrome profile owned by `local-search` at:

```txt
~/Library/Application Support/local-search/chrome-profile
```

Sign in to accounts there once if you want authenticated search/read/extract.
After that:

```sh
lsearch "latest rust cdp browser automation"
lsearch search "site:docs.rs tokio Runtime" --limit 5 --pretty
lsearch search "best browser search APIs for agents" --with-content --limit 3 --pretty
lsearch read https://example.com
lsearch extract "a[href]" --field title=text --field url=href --pretty
lsearch map https://example.com --depth 1 --limit 25 --pretty
lsearch cleanup --pretty
```

## Why This Exists

Agent search is dominated by paid or hosted APIs:

- [Exa](https://exa.ai/docs/reference/search) offers search plus extracted
  contents/highlights and deeper research modes.
- [Firecrawl](https://docs.firecrawl.dev/api-reference/v2-introduction) offers
  search, scrape, crawl, map, extract, and agentic web data features.
- [Brave Search API](https://api-dashboard.search.brave.com/documentation)
  exposes Brave's independent index, including LLM-oriented context endpoints.
- [Tavily](https://docs.tavily.com/documentation/api-reference/introduction)
  offers search, extract, crawl, and research APIs for agents.

Those are useful production services. `local-search` is for the cases where an
agent should use the browser already on the machine and spend zero API credits.

The browser-tooling neighbors are different:

- [browse.sh](https://browse.sh/) / Browserbase Browse CLI is a broader browser
  skills and browser automation surface.
- [agent-browser](https://github.com/vercel-labs/agent-browser) is an
  agent-first browser automation CLI with snapshots, refs, tabs, forms, and
  network tools.
- [browser-use CLI](https://docs.browser-use.com/open-source/browser-use-cli)
  gives coding agents direct browser control using local or cloud browsers.
- [AgentWebSearch](https://mcpmarket.com/server/agentwebsearch) is the closest
  local-search neighbor: local LLM web search through real Chrome/CDP.

`local-search` keeps the browser controls, but frames them as search
infrastructure.

## Comparison

| Tool | Primary job | Hosted/API key | What local-search optimizes for |
|---|---|---:|---|
| Exa | AI-native web search, contents, highlights, deep search | Yes | Zero-cost local search and browser-session auth |
| Firecrawl | Search, scrape, crawl, map, extract at scale | Hosted or self-hosted | Single-machine agent search without service setup |
| Brave Search API | Independent search index and LLM context | Yes | Consumer search surfaces through your own browser |
| Tavily | Search/extract/crawl/research APIs | Yes | No account, no metered usage, local browser state |
| browse.sh / Browse CLI | Browser skills and browser/cloud automation | Optional cloud | Search-first CLI with paid-search replacement framing |
| agent-browser | General browser automation for agents | No | Structured search/read/extract as the main product |
| browser-use CLI | Agent browser control via Python workflows | Optional cloud | Native Rust, JSON-first search API replacement |
| AgentWebSearch | Local Chrome search for LLMs | No | CLI-first structured outputs plus extraction/artifacts |

## Commands

Search:

```sh
lsearch "hi"
lsearch search "open source browser automation rust" --limit 10
lsearch search "firecrawl alternatives" --engine duckduckgo --with-content --limit 5
```

DuckDuckGo is the default engine because it is usually less hostile to local CLI
search sessions. Google and Bing remain available with `--engine google` and
`--engine bing`.

Read and extract:

```sh
lsearch read https://example.com --format markdown
lsearch read https://example.com --format json --pretty
lsearch extract "article" --field title="h1=>text" --field url="a=>href"
```

Map a site locally:

```sh
lsearch map https://docs.rs --depth 1 --limit 50 --pretty
```

Authenticated browser fetch:

```sh
lsearch request https://example.com/api/me --header "Accept: application/json" --pretty
```

Artifacts:

```sh
lsearch screenshot artifacts/page.png --full-page
lsearch mhtml artifacts/page.mhtml
lsearch record https://browse.sh/ --har artifacts/browse.har --mhtml artifacts/browse.mhtml
```

Browser primitives for debugging workflows:

```sh
lsearch snapshot --pretty
lsearch click @e3
lsearch fill "input[name=q]" "local search cli"
lsearch press Enter
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

## Browser Setup

Recommended:

```sh
lsearch launch
lsearch cleanup --pretty       # dry-run managed browser cleanup
lsearch cleanup --kill         # stop managed browser and clear stale markers
```

This avoids Chrome's default-profile remote debugging prompts by using a
separate persistent `local-search` profile. You can still attach to an existing
endpoint when needed:

```sh
lsearch --cdp 9222 doctor
lsearch --cdp ws://127.0.0.1:9222/devtools/browser/... tabs list
```

Agents can call the cleanup wrapper directly:

```sh
scripts/local-search-cleanup.sh --pretty
scripts/local-search-cleanup.sh --kill --pretty
```

Safari is intentionally limited. Its official WebDriver automation uses isolated
automation sessions, not the normal signed-in browsing profile this project
targets.
