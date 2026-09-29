# X and LinkedIn drafts

Prepared September 25, 2026. Product workflow: released 0.2.0. Core redesign: unreleased.
See [the media map and publishing checks](README.md) before posting.

## X thread

Media: post 1 agent-browser.gif; post 4 browser-choice.gif; post 6 benchmark.png.

### 1/9

Your browser is already signed in. Your coding agent should be able to use it.

I built local-search: one Rust CLI that lets agents search, read pages, and use an approved Chrome session on your machine.

Give it a URL and a task. Here's how it works.

### 2/9

Say you're already signed into LinkedIn.

lsearch read https://www.linkedin.com/feed/ --format json

After you connect and approve Chrome, the agent can read through that session. The same interface works for public docs and other pages your chosen profile can access.

### 3/9

Search works the same way.

lsearch "rust async" --engine bing --limit 3 --json

The agent gets ranked titles, URLs, and snippets. Choose Google, Bing, Brave, or DuckDuckGo with the same command.

Claude Code, Codex, Cursor, OpenClaw: anything that can run a shell command.

### 4/9

You choose the browser.

Use your open Chrome with approval, or keep agent work in a separate persistent profile.

On macOS/Linux, commands reuse the approved connection. lsearch disconnect ends that access and leaves Chrome open. Reconnecting requires approval again.

### 5/9

That approval grants broad browser control. Use agents you trust and give them a clear task.

Sites still decide what you can access. Verification pages are reported as blocked. If your chosen browser disconnects, lsearch returns an error instead of switching profiles.

### 6/9

I've also been rebuilding the core. The development build is still unreleased.

Bing, one query, 20 samples/build: median uncached search fell 219.6 to 175.7 ms. Cached: 23.9 to 16.5 ms.

Uncached p95 got slower: 294.1 to 328.2 ms. Full scope and caveats in the graphic.

### 7/9

Some local reads were slower too. Google, Brave, and DuckDuckGo hit verification pages in this rerun. Hosted-provider benchmarks still need a fresh run with API keys.

I'm keeping the raw results with the code so the next change has something concrete to beat.

### 8/9

Try the released CLI:

cargo install local-search

or

npm install -g @kevinliu01/localsearch

The npm package builds the same native Rust CLI and requires Rust/Cargo. These install released 0.2.0. The redesign is unreleased.

MIT licensed. Choose your browser during setup.

### 9/9

Start with one task: read a page you're signed into and return a short summary with source links.

Docs: https://lsearch.dev/docs
Source: https://github.com/Kevin-Liu-01/Local-Search

Browser access is powerful. Keep the task specific.

## LinkedIn

Attach agent-browser.mp4. For a static post, use hero-portrait.png.

Your browser is already signed in. Your coding agent should be able to use it.

That's why I built local-search.

It's a small Rust CLI that connects a shell-capable agent to Chrome on your machine. The agent can search the web, read pages, extract records, and use the sessions in the browser profile you choose.

A concrete example: you're signed into LinkedIn and want your agent to read a page.

lsearch read https://www.linkedin.com/feed/ --format json

Connect to your existing Chrome and approve access first. Then the agent reads through that session. Public docs and other accessible pages use the same interface.

You can also keep agent work in a separate persistent profile. Your choice is remembered. On macOS/Linux, commands reuse the approved connection until it ends. Run lsearch disconnect when you're done; Chrome stays open.

Chrome's approval grants broad control. Use trusted agents and scope the task. Sites still control access, and verification pages stay blocked.

I've also been rebuilding the core. In one Bing workload with 20 samples per build, median uncached search went from 219.6 to 175.7 ms. Cached search went from 23.9 to 16.5 ms. Uncached p95 got slower, from 294.1 to 328.2 ms. That redesign is still unreleased, and the result is specific to this workload.

The released 0.2.0 CLI already supports the browser workflow shown here.

cargo install local-search

Claude Code, Codex, Cursor, OpenClaw, or any agent that can run a shell command. MIT licensed.

Start with one page and a clear task. I'll put the setup guide and source in the first comment.

### First comment

Setup guide: https://lsearch.dev/docs
Source: https://github.com/Kevin-Liu-01/Local-Search

The attached workflow is an illustration, not a timed recording. Connecting to existing Chrome requires approval. Performance figures refer to the unreleased development build; the benchmark report will be linked here once that work is pushed.
