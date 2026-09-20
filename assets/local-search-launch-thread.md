# local-search launch thread

## Standalone post A · Product

Your browser can already do more than a search API.

local-search gives agents one local browser API: search Google, read Reddit or
docs, extract pages, and use sites you sign into in its Chrome profile.

No API key. No metered search bill. 50.3 KiB crate.

`cargo install local-search`

https://github.com/Kevin-Liu-01/Local-Search

## Standalone post B · Context

Search pages are built for people. Agents need results.

In our 3-result benchmark, local-search returned 309 tokens vs. 8,760.5 from an interactive browser snapshot—96.5% less context.

Your browser searches. Stable JSON returns.

https://github.com/Kevin-Liu-01/Local-Search

## Standalone post C · Numbers

local-search is a 50.3 KiB Rust crate that turns local Chrome into a browser API
for agents.

Matched provider run:
• 53.4 tokens/result
• 148.7 ms median latency
• 24/24 result depths fulfilled
• $0 hosted API usage

No key. No search bill.

https://github.com/Kevin-Liu-01/Local-Search

# Thread

## 1 · Launch

Introducing local-search: a local browser API for agents, packaged as a tiny Rust CLI.

Search Google. Read Reddit and docs. Extract or interact with a page. Use sites you sign into in its dedicated Chrome profile.

No API key. No metered search bill.

In our benchmark, 3 results used 96.5% less visible agent context than an interactive browser snapshot.

## 2 · Links

Install it with:

`cargo install local-search`

Repo + benchmark methodology:
https://github.com/Kevin-Liu-01/Local-Search

crates.io:
https://crates.io/crates/local-search

## 3 · Principle

local-search's performance comes from one simple principle: return result data, not search-page chrome.

Your local browser does the web work. The agent gets stable JSON: rank, title, URL, domain, snippet, and optional page content.

## 4 · Benchmark

For 3-result searches, local-search returned 309 visible tokens vs. 8,760.5 for an interactive browser snapshot: 96.5% less context.

In the matched provider run: 53.4 tokens/result, 148.7 ms median latency, and $0 hosted API usage.

## 5 · Reliability

That smaller output still held up across engines:

• 72/72 requested result depths fulfilled
• 60/60 schema-valid stability runs
• 72/72 content pages returned the full 1,200-character cap

Google, Bing, DuckDuckGo, and Brave Search all run through your local browser.

## 6 · Native Rust footprint

The CLI is small, too:

• 50.3 KiB compressed crates.io package
• 1.06 MB arm64 macOS release binary
• 4.61 ms median warm process start

The binary and startup figures are machine-specific; search latency is measured separately.

## 7 · Close

local-search is built around a simple idea: agents should be able to use a local browser through one clean interface—without wiring up a separate API for every site.

MIT licensed. Written in Rust. Benchmarks + methodology are public.

https://github.com/Kevin-Liu-01/Local-Search

## Image alt text

Benchmark graphic comparing local-search with interactive browser snapshots and hosted search APIs. For three results, local-search returns 309 visible tokens versus 8,760.5 for a browser snapshot, or 96.5% less context. In matched provider runs, local-search records 53.4 normalized tokens per result, 148.7 milliseconds median latency, 24 of 24 requested result depths fulfilled, and zero dollars in hosted API usage.
