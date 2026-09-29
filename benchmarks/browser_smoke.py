#!/usr/bin/env python3
"""Live correctness checks using only owned Chrome and synthetic localhost pages.

This tests traditional headless CDP plus the persistent existing-browser helper.
It does not exercise Chrome's interactive approval prompt or signed-in accounts.
The public CLI has no `wait --js`: promise evaluation is tested through `eval`,
and browser-side predicate waiting through `wait --selector`.
"""
from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import os
import shutil
import subprocess
import tempfile
import threading
import time
import urllib.request
from datetime import datetime, timezone
from pathlib import Path


class SmokeFailure(Exception):
    """A public-safe failure code, never a browser response body."""


class Fixture(http.server.BaseHTTPRequestHandler):
    counts: dict[str, int] = {}
    lock = threading.Lock()

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if path == "/second":
            time.sleep(.3)
        with self.lock:
            self.counts[path] = self.counts.get(path, 0) + 1
            count = self.counts[path]
        name = {"/first": "First fixture", "/second": "Second fixture",
                "/reload": "Reload fixture", "/delayed": "Delayed fixture"}.get(path, "Fixture")
        script = ("<script>setTimeout(() => {const p = document.createElement('p');"
                  "p.id = 'arrived'; p.textContent = 'Delayed content';"
                  "document.querySelector('main').append(p)}, 900)</script>"
                  if path == "/delayed" else "")
        body = (f"<!doctype html><title>{name}</title><main><h1>{name}</h1>"
                f"<p id='generation'>{count}</p></main>{script}").encode()
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Cache-Control", "no-store")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        try:
            self.wfile.write(body)
        except (BrokenPipeError, ConnectionResetError):
            pass

    def log_message(self, *_):
        pass


def browser_path(explicit: Path | None) -> str:
    if explicit:
        if not explicit.is_file():
            raise SmokeFailure("browser_executable_missing")
        return str(explicit.resolve())
    mac = Path("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    if mac.is_file():
        return str(mac)
    for executable in ("google-chrome", "chromium", "chromium-browser"):
        if found := shutil.which(executable):
            return found
    raise SmokeFailure("browser_executable_missing")


def isolated_env(root: Path) -> dict[str, str]:
    env = {key: value for key, value in os.environ.items()
           if not key.startswith(("LOCAL_SEARCH_", "LOCAL_BROWSER_"))}
    env.update(LOCAL_SEARCH_CONFIG_DIR=str(root / "config"),
               LOCAL_SEARCH_CACHE_DIR=str(root / "cache"), NO_COLOR="1")
    return env


class Runner:
    def __init__(self, binary: Path, env: dict[str, str]):
        self.binary, self.env = binary, env
        self.rows: list[dict] = []

    def call(self, *args: str, expected_error: str | None = None,
             timeout_ms: int = 5000) -> dict:
        completed = subprocess.run(
            [str(self.binary), "--timeout", str(timeout_ms), *args],
            env=self.env, capture_output=True, timeout=70, check=False)
        try:
            value = json.loads(completed.stderr if expected_error else completed.stdout)
        except (ValueError, UnicodeDecodeError):
            raise SmokeFailure("invalid_json_envelope") from None
        if expected_error:
            if completed.returncode == 0 or value.get("error", {}).get("code") != expected_error:
                raise SmokeFailure("unexpected_error_category")
        elif completed.returncode or value.get("ok") is not True:
            raise SmokeFailure("command_failed")
        return value

    def step(self, name: str, action):
        start = time.perf_counter()
        try:
            action()
        except (SmokeFailure, subprocess.TimeoutExpired, OSError, ValueError) as error:
            self.rows.append({"name": name, "ok": False,
                              "error": str(error) if isinstance(error, SmokeFailure) else type(error).__name__,
                              "elapsed_ms": round((time.perf_counter() - start) * 1000, 3)})
            raise
        self.rows.append({"name": name, "ok": True,
                          "elapsed_ms": round((time.perf_counter() - start) * 1000, 3)})


def require(condition: bool, code: str):
    if not condition:
        raise SmokeFailure(code)


def exercise(runner: Runner, base: str):
    for _ in range(3):
        for path, title in (("/first", "First fixture"), ("/second", "Second fixture")):
            page = runner.call("read", base + path, "--format", "json").get("page", {})
            require(page.get("url") == base + path and page.get("title") == title
                    and title in page.get("text", ""), "stale_navigation_read")


def version(port: int) -> dict:
    with urllib.request.urlopen(f"http://127.0.0.1:{port}/json/version", timeout=3) as response:
        return json.load(response)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--browser-path", type=Path)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    binary = args.binary.resolve()
    if not binary.is_file():
        parser.error("binary must exist")
    root = Path(tempfile.mkdtemp(prefix="lsearch-browser-smoke-"))
    profile = root / "profile"
    profile.mkdir()
    runner = Runner(binary, isolated_env(root))
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Fixture)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    base = f"http://127.0.0.1:{server.server_port}"
    browser = None
    report = {"schema_version": 1, "recorded_at": datetime.now(timezone.utc).isoformat(),
              "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "scope": "Synthetic localhost pages and an empty owned Chrome profile. No approval UI or signed-in-site test.",
              "checks": runner.rows}
    try:
        browser = subprocess.Popen(
            [browser_path(args.browser_path), "--headless=new", "--remote-debugging-port=0",
             "--remote-debugging-address=127.0.0.1", f"--user-data-dir={profile}",
             "--no-first-run", "--no-default-browser-check", "--disable-background-networking",
             "about:blank"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        deadline = time.monotonic() + 15
        marker = profile / "DevToolsActivePort"
        while not marker.is_file() and browser.poll() is None and time.monotonic() < deadline:
            time.sleep(.05)
        require(marker.is_file(), "owned_browser_start_failed")
        port = int(marker.read_text().splitlines()[0])
        original = version(port)
        report["browser_version"] = original.get("Browser")
        runner.step("connect_isolated_endpoint", lambda: runner.call("connect", f"http://127.0.0.1:{port}"))
        runner.step("delayed_navigation_never_reads_old_document", lambda: exercise(runner, base))
        runner.step("promise_evaluation", lambda: require(
            runner.call("eval", "Promise.resolve(true)").get("value") is True, "promise_not_awaited"))

        def delayed_dom():
            runner.call("open", base + "/delayed")
            runner.call("wait", "--selector", "#arrived")
            require(runner.call("eval", "document.querySelector('#arrived').textContent").get("value") == "Delayed content", "wait_returned_early")
        runner.step("delayed_dom_wait", delayed_dom)

        def reload_page():
            runner.call("open", base + "/reload")
            before = runner.call("eval", "document.querySelector('#generation').textContent")["value"]
            runner.call("reload")
            after = runner.call("eval", "document.querySelector('#generation').textContent")["value"]
            require(int(after) > int(before), "reload_returned_old_document")
        runner.step("reload_waits_for_new_document", reload_page)

        def history():
            runner.call("open", base + "/first")
            runner.call("open", base + "/second")
            runner.call("back")
            require(runner.call("eval", "location.href")["value"] == base + "/first", "back_returned_old_url")
            runner.call("forward")
            require(runner.call("eval", "location.href")["value"] == base + "/second", "forward_returned_old_url")
            runner.call("eval", "history.pushState({}, '', '#next'); true")
            runner.call("back")
            require(runner.call("eval", "location.href")["value"] == base + "/second", "same_document_back_failed")
            runner.call("forward")
            require(runner.call("eval", "location.hash")["value"] == "#next", "same_document_forward_failed")
        runner.step("cross_and_same_document_history", history)
        runner.step("javascript_errors_propagate", lambda: runner.call(
            "eval", "throw new Error('synthetic smoke failure')", expected_error="javascript_error"))
        runner.step("missing_selector_has_bounded_timeout", lambda: runner.call(
            "wait", "--selector", "#never-created", expected_error="timeout", timeout_ms=250))

        def cached_content():
            # Seed only a synthetic browser-scoped record; no search-provider
            # traffic is needed to verify the content extraction path.
            config_file = Path(runner.env["LOCAL_SEARCH_CONFIG_DIR"]) / "config.json"
            scope = json.loads(config_file.read_text())["endpoint"]
            query = "synthetic smoke content"
            digest = 0xcbf29ce484222325
            for byte in (scope + "\0google\0" + query).encode():
                digest = ((digest ^ byte) * 0x100000001b3) & 0xffffffffffffffff
            record = {"scope": scope, "engine": "google", "query": query,
                      "search": {"url": base, "title": "Synthetic search fixture", "blocked": False,
                                 "results": [{"rank": index + 1, "title": title, "url": base + path,
                                              "domain": "127.0.0.1", "snippet": "Synthetic fixture"}
                                             for index, (path, title) in enumerate(
                                                 (("/first", "First fixture"), ("/second", "Second fixture")))]}}
            cache = Path(runner.env["LOCAL_SEARCH_CACHE_DIR"])
            cache.mkdir(parents=True, exist_ok=True)
            path = cache / f"{digest:016x}.json"
            encoded = json.dumps(record)
            path.write_text(encoded)
            before = {tab["targetId"] for tab in runner.call("tabs", "list")["tabs"]}
            result = runner.call("search", query, "--limit", "2", "--with-content", "--content-chars", "8", "--json")
            contents = result.get("search", {}).get("contents", [])
            require(len(contents) == 2, "content_pages_missing")
            for page, suffix, prefix in zip(contents, ("/first", "/second"), ("First fi", "Second f")):
                require(page.get("url") == base + suffix and page.get("text") == prefix, "stale_or_unbounded_content")
            after = {tab["targetId"] for tab in runner.call("tabs", "list")["tabs"]}
            require(before == after, "temporary_content_target_not_closed")
            require(path.read_text() == encoded, "synthetic_cache_was_not_used")
        runner.step("cached_content_is_distinct_bounded_and_cleans_tab", cached_content)
        runner.step("connect_persistent_helper", lambda: runner.call("connect", "--existing", "--profile", str(profile)))

        def helper_reuse():
            config_file = Path(runner.env["LOCAL_SEARCH_CONFIG_DIR"]) / "config.json"
            session = json.loads(config_file.read_text()).get("session")
            require(isinstance(session, dict), "helper_session_missing")
            for number in range(3):
                require(runner.call("eval", str(number))["value"] == number, "helper_evaluation_failed")
                require(json.loads(config_file.read_text()).get("session") == session, "helper_session_replaced")
            exercise(runner, base)
            runner.call("connect", "--existing", "--profile", str(profile))
            require(json.loads(config_file.read_text()).get("session") == session, "explicit_connect_did_not_reuse_helper")
        runner.step("helper_reused_across_commands", helper_reuse)
        runner.step("helper_cached_content_and_target_cleanup", cached_content)

        def disconnect():
            runner.call("disconnect")
            require(browser.poll() is None and version(port).get("webSocketDebuggerUrl") == original.get("webSocketDebuggerUrl"), "disconnect_closed_chrome")
            runner.call("eval", "42", expected_error="browser_disconnected")
        runner.step("disconnect_preserves_browser_and_prevents_fallback", disconnect)
    except (SmokeFailure, subprocess.TimeoutExpired, OSError, ValueError) as error:
        report["error"] = str(error) if isinstance(error, SmokeFailure) else type(error).__name__
    finally:
        cleanup_ok = True
        # Disconnect only the helper saved in our temporary config, if any.
        config_file = root / "config" / "config.json"
        try:
            if config_file.is_file() and json.loads(config_file.read_text()).get("session"):
                runner.call("disconnect")
        except (SmokeFailure, subprocess.TimeoutExpired, OSError, ValueError):
            cleanup_ok = False
        if browser is not None and browser.poll() is None:
            browser.terminate()  # Exact owned PID; never escalate to SIGKILL.
            try:
                browser.wait(timeout=15)
            except subprocess.TimeoutExpired:
                cleanup_ok = False
        server.shutdown()
        server.server_close()
        if cleanup_ok:
            shutil.rmtree(root)
        report["cleanup_ok"] = cleanup_ok
    report["ok"] = "error" not in report and report["cleanup_ok"] and all(row["ok"] for row in runner.rows)
    serialized = json.dumps(report, indent=2) + "\n"
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(serialized)
    print(serialized, end="")
    return 0 if report["ok"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
