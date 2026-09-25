# Security

`local-search` controls a browser profile that may already be signed in to your
accounts. Treat the Chrome DevTools Protocol endpoint as sensitive local
authority.

- Bind debugging endpoints to localhost only.
- Do not expose `--remote-debugging-port` on a public interface.
- Avoid committing generated HAR, MHTML, HTML, screenshot, PDF, or cookie output.
- Cookie values are redacted by default; use `cookies list --show-values` only in
  private terminals.
- `request` runs `fetch` inside the browser, so it can carry ambient cookies and
  local storage credentials for the page origin.

Choose browser authority explicitly:

- `connect --existing` uses Chrome 144+'s opt-in debugging endpoint. Enable
  `chrome://inspect/#remote-debugging` yourself and approve Chrome's prompt.
  The CLI reads the selected user-data root's `DevToolsActivePort`; it never
  copies cookie databases or relaunches your everyday profile with flags.
  On macOS/Linux, a local helper keeps that approved connection open across CLI
  commands. This preserves broad browser access between commands; it does not
  suppress or bypass Chrome's consent for a new connection. Run `lsearch
  disconnect` to end access without closing Chrome or deleting cookies.
  A lost connection is never automatically reopened. Reconnecting explicitly
  requires Chrome's approval again.
- `connect --managed` / `launch` explicitly select a separate persistent profile
  under the local-search config directory, with loopback CDP. Sign in there only
  to accounts you want your agent to access.

Chrome 136+ ignores command-line debugging flags on its default profile. Do not
work around this by exporting credentials or weakening Chrome's security settings.
See [Chrome's configuration guide](https://developer.chrome.com/docs/devtools/agents/get-started/configuration).

The browser selection is only saved after a successful CDP round trip. Missing,
stale, or unreachable selected endpoints return `browser_disconnected`,
without discovery or launch fallback. Explicit `--cdp` is transient. With no
selection, browser commands request a choice rather than silently opening Chrome.
Initial approval timeouts use `browser_approval_timeout`; an explicit HTTP
permission rejection uses `browser_approval_denied`. An ambiguous handshake
failure uses `browser_connection_failed`, not a claim that the user denied access.
Search caches are scoped to the browser session and checked only after connection.
Ordinary page commands create/reuse a background work tab, not an arbitrary
personal tab; `--target` and `tabs use` deliberately grant control of a named tab.

CDP approval is broad browser authority, not per-site authorization. Returned
signed-in page data goes to the calling agent and may be sent to its model provider.
Only connect accounts you trust that agent with; keep tasks within your authority.
Disable remote debugging in Chrome to revoke future existing-browser connections.

## Persistent helper boundary

The helper accepts no TCP/HTTP connections. Its Unix sockets are mode 0600 inside
a newly created mode-0700 local directory. Other OS users and web pages cannot
use that transport. Other processes running as the same OS user remain in the
trust boundary, just as with the CLI itself. Do not share that account with
untrusted programs.

One command leases the upstream connection at a time. Request IDs are remapped,
page-session events are scoped to that lease, and page sessions are detached when
the command exits. Late responses cannot reach the next command. The helper
does not save page data, cookies, queries, or protocol logs. Its environment is
cleared when spawned. Browser choice changes are guarded by a process lock.

If startup fails or the parent abandons setup before committing the verified
selection, the helper closes. Disconnect drops the transport, never sends
`Browser.close`, and can revoke access while a command is running. A helper or
browser crash requires an explicit reconnect. The managed-profile path remains
independent and does not need this helper.

## Release checks

Interactive welcome/setup can request public version metadata from crates.io,
or registry.npmjs.org for the npm bridge. These optional checks run outside the
browser using system `curl`, with HTTPS verification, no redirects or `.curlrc`,
a bounded response size, and a two-second timeout. They send a fixed User-Agent
and package URL, not queries, cookies, page content, account identifiers, or
browser/profile data. As with any network request, the registry sees the request's
source IP; system proxy environment settings may apply.

Automatic checks use a daily local cache and never install or execute downloaded
code. They are skipped in pipelines, CI, `--json` / `--pretty` mode, and ordinary
search/browser commands. Set `LOCAL_SEARCH_NO_UPDATE_CHECK=1` to opt out of
automatic checks. The explicit `lsearch update-check` command still fetches
release metadata when requested. Update commands use fixed package names;
untrusted registry text is never interpolated into a shell command.
