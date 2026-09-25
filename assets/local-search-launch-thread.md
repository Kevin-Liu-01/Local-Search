# local-search: agent-first posts

These drafts describe the locally tested implementation accurately today.
Persistent sessions and the new guide are not yet pushed or published. Do not
claim a fresh npm/crates.io install has this behavior until the release checklist
in [verification.md](../docs/verification.md) passes. Once shipped, replace the
last X post's release-pending sentence with the exact release and guide link.

## X thread

### 1/7

Your browser is already logged in. Your agent hits a login page.

I've been building local-search to close that gap: a small Rust CLI that lets agents search, read, and work with sites through a local browser you approve.

Media: [agent and browser overview](../docs/images/01-agent-browser.png).

### 2/7

Just tested LinkedIn, GitHub, and Reddit with my existing Chrome login.

LinkedIn feed: readable.
GitHub settings: open.
Reddit settings: readable.

Same approved connection. No cookie export. No extra login in a second browser.

Media: [recorded signed-in checks](../docs/images/03-signed-in-sites.png).

### 3/7

The command is the same across sites:

lsearch read https://www.linkedin.com/feed/ --format json

Give your agent a page and a task. It gets readable content or JSON through your signed-in browser.

The page data goes to your agent, so choose what you share.

Media: reuse the signed-in checks image only if posting this independently.

### 4/7

Search is one command too:

lsearch "rust browser automation libraries" --engine duckduckgo --limit 3 --json

Google, Bing, Brave, or DuckDuckGo. The same title, URL, domain, rank, and snippet fields.

Small results the agent can use, without a hosted search API key.

Media: [search workflow](../docs/images/04-search.png), with the recorded-results
date kept visible.

### 5/7

Then keep going.

Read the linked page. Extract a list of records. Inspect a form. Make an authorized browser request. Save a screenshot when you need visual proof.

One CLI for a shell-capable agent. Claude Code, Codex, Cursor, OpenClaw, or your own.

Media: [extraction](../docs/images/05-extract.png) and
[interaction, requests, captures](../docs/images/06-work-with-pages.png).

### 6/7

You choose your existing Chrome or a separate persistent profile.

Existing Chrome asks for approval once per connection. Commands reuse it.

lsearch disconnect ends access and leaves Chrome open.

If the connection drops, the agent gets an error. No silent reconnect.

Media: [browser choice](../docs/images/02-browser-choice.png) and
[disconnect](../docs/images/07-control.png).

### 7/7

The signed-in workflow is tested locally. The persistent-session package update is next.

Open source, with the agent command reference in the repo:
https://github.com/Kevin-Liu-01/Local-Search

Where does your agent most often get stuck at a login screen?

Media: none. Link back to the first post as needed.

## LinkedIn

The browser was already logged in. I wanted my agent to use that session.

I've been building local-search, a small Rust CLI that gives agents one interface to a local browser.

Today I tested it against LinkedIn, GitHub, and Reddit using my existing Chrome login.

It read my LinkedIn feed, opened GitHub profile settings, and read Reddit account settings. The same GitHub URL redirected an unauthenticated request to login.

All through one approved Chrome connection. No cookie export or extra login in a second browser.

The command is straightforward:

lsearch read https://www.linkedin.com/feed/ --format json

That gives a shell-capable agent page content it can use for the task you've authorized. Claude Code, Codex, Cursor, OpenClaw, or your own agent can call the same CLI.

It also searches Google, Bing, Brave, and DuckDuckGo, extracts records, interacts with pages, makes browser requests, and saves screenshots or documents when needed.

You choose your everyday Chrome or a separate persistent profile. With existing Chrome, you approve one connection and subsequent commands reuse it. lsearch disconnect ends access without closing Chrome. If the connection drops, it reports an error instead of silently reconnecting.

That approval grants broad browser access, so the agent still needs to stay within your task. Sites keep their access checks. Returned page content goes to the calling agent.

The persistent-session update is tested locally and awaiting release. I put together an illustrated guide showing the commands, the browser choice, and what comes back.

Source and agent reference:
https://github.com/Kevin-Liu-01/Local-Search

Where does your agent most often get stuck at a login screen?

## Media order and alt text

For LinkedIn, attach these four images in order. All are public-safe screenshots
of the authored guide, not screenshots of private accounts.

1. [Signed-in checks](../docs/images/03-signed-in-sites.png).
   Alt: “Recorded local-search checks reached a signed-in LinkedIn feed, GitHub
   profile settings, and Reddit account settings through one approved Chrome
   connection. Private content omitted. Local build, September 25, 2026.”
2. [Agent/browser overview](../docs/images/01-agent-browser.png).
   Alt: “An agent calls lsearch, the chosen local browser loads sites, and useful
   data returns. Search, read, extract, interact, request, and capture are shown.”
3. [Extract records](../docs/images/05-extract.png).
   Alt: “Two synthetic task cards become title and URL records using lsearch
   extract. The illustration contains no real work or account data.”
4. [End access](../docs/images/07-control.png).
   Alt: “lsearch disconnect returns browserClosed false. Separate errors identify
   approval timeout, disconnection, and a busy browser connection.”

For X, the media instructions appear beside each post. Keep the provenance text
visible when uploading. Do not crop away the local-build or illustration labels.
