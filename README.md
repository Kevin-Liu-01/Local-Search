# local-search

**Give your agent the browser you already use.**

Search the web, read pages, and use signed-in sites through one small Rust CLI.
Choose your existing Chrome or a separate profile. Your agent gets compact JSON
or readable text, without a hosted browser service or a search API key.

[Documentation](https://lsearch.dev/docs) · [Agent reference](https://github.com/Kevin-Liu-01/Local-Search/blob/main/SKILL.md) · [crates.io](https://crates.io/crates/local-search) · [npm](https://www.npmjs.com/package/@kevinliu01/localsearch)

<a href="https://lsearch.dev"><img src="https://raw.githubusercontent.com/Kevin-Liu-01/Local-Search/main/site/public/social/2026-09/chrome-demo.gif?v=2026-09-30-clean" alt="Demo: an agent runs lsearch, Chrome loads search results, and structured results return to the terminal" width="100%"></a>

*Illustrated demo, not a timing measurement. [Download video](https://raw.githubusercontent.com/Kevin-Liu-01/Local-Search/main/site/public/social/2026-09/chrome-demo.mp4).*

## Install

```sh
cargo install local-search
```

Or use npm (requires Node.js 18+ and Rust/Cargo):

```sh
npm install -g @kevinliu01/localsearch
```

Both install the same native Rust CLI. Chrome or Chromium is required.
`local-search` and `local-browser` remain aliases for `lsearch`.

## Connect your browser

Choose one:

**Use your existing logins.** In Chrome 144+, enable remote debugging at
`chrome://inspect/#remote-debugging`. Then run this command and approve Chrome's
connection prompt:

```sh
lsearch connect --existing
```

One approved connection is reused across commands on macOS/Linux. Your logins
stay in Chrome; you do not need to export cookies.

**Keep agent work separate.** Create a persistent local profile and sign into
the sites your agent needs there:

```sh
lsearch connect --managed
```

Your choice is remembered. If the browser disconnects, lsearch reports an error
instead of silently switching profiles. To end existing-browser access without
closing Chrome:

```sh
lsearch disconnect
```

## Use it

```sh
# Search Google, Bing, Brave, or DuckDuckGo
lsearch "rust async cancellation" --engine google --limit 3 --json

# Read a page as text or structured data
lsearch read https://docs.rs/tokio/latest/tokio/ --format markdown

# Read a signed-in page through the profile you chose
lsearch read https://www.linkedin.com/feed/ --format json

# Inspect the current page before interacting
lsearch snapshot --limit 40
```

| Task | Command |
| --- | --- |
| Search with ranked titles, URLs, and snippets | `lsearch "query" --engine bing --json` |
| Read a page | `lsearch read URL --format json` |
| Extract repeated fields | `lsearch extract SELECTOR --field name=SOURCE` |
| Map links on a site | `lsearch map URL --depth 1 --limit 20` |
| Interact with a page | `lsearch snapshot`, then `click` or `fill` |
| Make a browser-authenticated request | `lsearch request URL` |
| Capture a page | `lsearch screenshot`, `html`, or `pdf` |

Run `lsearch COMMAND --help` for options. The
[command reference](https://github.com/Kevin-Liu-01/Local-Search/blob/main/SKILL.md)
also covers tabs, bounded reads, exports, and error handling.

## Give it to your agent

Works with Claude Code, Codex, Cursor, OpenClaw, and any agent that can run shell
commands. Give it the [agent reference](https://github.com/Kevin-Liu-01/Local-Search/blob/main/SKILL.md), then a task:

> Use lsearch with my connected browser. Research this topic and return a short
> summary with source links. Do not post, send messages, or change settings.
> Ask before reconnecting or switching profiles.

Piped search output and `--json` use stable JSON. Progress stays on stderr.
Failures have a nonzero exit status and a structured error code.

## Benchmarks

![Search context comparison: local-search 54.7, Exa 68.5, Brave Search 71.0, Tavily 73.5, and Firecrawl 70.3 median tokens per result](https://raw.githubusercontent.com/Kevin-Liu-01/Local-Search/main/site/public/social/2026-09/benchmark-clean.png?v=2026-09-30-dark)

Ten matched queries, three results each, the same JSON fields, and a 120-character
snippet cap for every provider. local-search 0.2.0 used **54.7 median tokens per
result** through Bing. This measures context size, not search quality.

The full Bing run returned all requested results in **24/24 searches**. The chart
uses the shared successful subset because some hosted requests failed. Latency,
cache hits, and failed requests are reported separately.

See the [search benchmark and methodology](https://github.com/Kevin-Liu-01/Local-Search/blob/main/benchmarks/search-2026-09-29.md)
for the latest measurements, request settings, and reproducible runner.

## Access and privacy

Browser work happens on your machine. Existing-browser approval gives broad
access to that Chrome session, so connect only trusted agents and define their
task clearly. A separate profile keeps agent logins apart from everyday browsing.

Sites still control access. lsearch does not bypass logins, CAPTCHAs, or site
restrictions. Cookie values are redacted by default. See the
[security model](https://github.com/Kevin-Liu-01/Local-Search/blob/main/SECURITY.md).

[MIT licensed](https://github.com/Kevin-Liu-01/Local-Search/blob/main/LICENSE).
