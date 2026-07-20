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

Chrome 136 and newer ignore remote debugging flags against the default profile
unless browser debugging is explicitly enabled or a separate `--user-data-dir` is
used. The most reliable normal-profile path is the dynamic endpoint Chrome writes
to `DevToolsActivePort` after local remote debugging is enabled.

For daily use, prefer `lsearch launch`. It starts a separate persistent
Chrome profile under the local-search config directory, binds CDP to
`127.0.0.1`, and avoids repeatedly attaching a debugger to your main Chrome
profile.
