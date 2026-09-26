# localsearch

The npm distribution of [`local-search`](https://github.com/Kevin-Liu-01/Local-Search):
a local browser API for agents. Search the web, read and extract pages, interact
with sites, and use approved existing Chrome sessions or a separate persistent
profile through one CLI.

```sh
npm install -g @kevinliu01/localsearch
lsearch connect --existing
# Or choose a separate profile: lsearch connect --managed
lsearch "rust browser automation" --engine google --limit 5 --json
```

## How it works

Version 0.2.0 installs native `local-search` 0.2.0. Existing Chrome requires
Chrome 144+ on macOS or Linux: enable `chrome://inspect/#remote-debugging`,
then approve the connection. Commands reuse that approved connection until
`lsearch disconnect` or a connection loss. Upgrading an older existing-browser
configuration requires one explicit reconnect. On Windows, use a managed profile.

This package is a thin distribution bridge, not a JavaScript rewrite. During
installation it uses Cargo to install the exact matching `local-search` Rust
crate into the npm package, then exposes these commands:

- `lsearch` — preferred CLI
- `localsearch` — npm-friendly alias
- `local-search` — compatibility alias
- `local-browser` — legacy compatibility alias

Node.js 18+ and a working Rust/Cargo installation are required. Install Rust
from [rustup.rs](https://rustup.rs) if `cargo --version` is unavailable.

If npm lifecycle scripts were disabled, install the native binary afterward:

```sh
npm rebuild @kevinliu01/localsearch
```

See the [full documentation](https://github.com/Kevin-Liu-01/Local-Search#readme)
and [website](https://lsearch.dev).

## License

MIT
