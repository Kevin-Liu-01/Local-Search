#!/usr/bin/env python3
"""Alternating A/B benchmark. Uses only owned, empty browser profiles.

No response bodies, cookie data, endpoints, or account identifiers are persisted.
The localhost page is a synthetic fixture, not a live search quality benchmark.
"""
from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import math
import os
import platform
import socket
import statistics
import subprocess
import tempfile
import threading
import time
import urllib.request
from datetime import datetime, timezone
from pathlib import Path


def isolated_env(directory: Path) -> dict[str, str]:
    env = {k: v for k, v in os.environ.items()
           if not k.startswith(("LOCAL_SEARCH_", "LOCAL_BROWSER_"))}
    env.update(LOCAL_SEARCH_CONFIG_DIR=str(directory / "config"),
               LOCAL_SEARCH_CACHE_DIR=str(directory / "cache"), NO_COLOR="1")
    return env


def percentile(values: list[float], fraction: float) -> float:
    ordered = sorted(values)
    position = (len(ordered) - 1) * fraction
    low, high = math.floor(position), math.ceil(position)
    return round(ordered[low] + (ordered[high] - ordered[low]) * (position - low), 3)


def summarize(rows: list[dict]) -> dict:
    attempted = [row for row in rows if not row.get("skipped")]
    good = [row for row in attempted if row["ok"]]
    result = {"samples": len(rows), "successes": len(good),
              "attempted": len(attempted), "skipped": len(rows) - len(attempted),
              "failures": len(attempted) - len(good)}
    if good:
        values = [row["elapsed_ms"] for row in good]
        result["latency_ms"] = {"min": min(values), "median": statistics.median(values),
                                "p95": percentile(values, .95), "max": max(values)}
        result["median_stdout_bytes"] = statistics.median(row["stdout_bytes"] for row in good)
        tokens = [row["response_tokens"] for row in good if row.get("response_tokens") is not None]
        if tokens:
            result["median_response_tokens"] = statistics.median(tokens)
    return result


def validate(payload: object, scenario: str) -> str | None:
    if not isinstance(payload, dict) or payload.get("ok") is not True:
        return "invalid_success_envelope"
    if "search" in scenario or scenario == "synthetic_cache_hit":
        search = payload.get("search")
        if not isinstance(search, dict):
            return "malformed_search"
        if search.get("blocked") is not False:
            return "blocked_or_missing_blocked_flag"
        results = search.get("results")
        if not isinstance(results, list) or len(results) < 3:
            return "insufficient_results"
        if any(not isinstance(item, dict) or not isinstance(item.get("url"), str)
               or not item["url"].startswith(("https://", "http://"))
               or not isinstance(item.get("title"), str) or not item["title"] for item in results):
            return "malformed_results"
        if scenario == "synthetic_cache_hit" and any(item.get("domain") != "example.invalid" for item in results):
            return "synthetic_cache_not_used"
    elif scenario == "local_read":
        page = payload.get("page")
        if not isinstance(page, dict) or "benchmark fixture" not in json.dumps(page).lower():
            return "unexpected_fixture_read"
    elif scenario == "local_eval" and payload.get("value") != 42:
        return "unexpected_eval_result"
    elif scenario == "delayed_dom_arm" and payload.get("value") != "armed":
        return "delayed_dom_not_armed"
    elif scenario == "delayed_dom_ready" and payload.get("result") != {"ok": True}:
        return "delayed_dom_not_ready"
    elif scenario == "fixture_visible" and payload.get("value") != "visible":
        return "fixture_not_visible"
    return None


def invoke(binary: Path, args: list[str], env: dict, scenario: str,
           encoder=None, timeout: float = 45) -> dict:
    start = time.perf_counter_ns()
    try:
        completed = subprocess.run([str(binary), *args], env=env, capture_output=True,
                                   timeout=timeout, check=False)
        row = {"elapsed_ms": round((time.perf_counter_ns() - start) / 1e6, 3),
               "stdout_bytes": len(completed.stdout), "ok": completed.returncode == 0}
        if completed.returncode:
            row["error"] = stderr_code(completed.stderr) or "process_exit_" + str(completed.returncode)
        elif scenario != "cli_version":
            try:
                error = validate(json.loads(completed.stdout), scenario)
                if error:
                    row.update(ok=False, error=error)
            except (ValueError, UnicodeDecodeError):
                row.update(ok=False, error="invalid_json")
        if row["ok"] and encoder:
            row["response_tokens"] = len(encoder.encode(completed.stdout.decode()))
        return row
    except subprocess.TimeoutExpired:
        return {"ok": False, "error": "process_timeout", "stdout_bytes": 0,
                "elapsed_ms": round((time.perf_counter_ns() - start) / 1e6, 3)}


def stderr_code(stderr: bytes) -> str | None:
    allowed = {"browser_not_found", "browser_not_configured", "browser_disconnected",
               "browser_approval_timeout", "browser_approval_denied", "browser_connection_failed",
               "browser_busy", "update_check_failed", "target_not_found", "unsupported",
               "protocol_error", "timeout", "invalid_argument", "javascript_error", "io_error",
               "json_error", "url_error", "http_error", "websocket_error", "search_blocked"}
    for line in stderr.splitlines():
        try:
            payload = json.loads(line)
            if isinstance(payload, dict) and payload.get("ok") is False:
                error = payload.get("error")
                code = error.get("code") if isinstance(error, dict) else None
                if code in allowed:
                    return code
        except (ValueError, UnicodeDecodeError, TypeError):
            pass
    return None


DELAYED_DOM_ARM = """(() => {
  clearTimeout(window.__lsearchBenchmarkTimer);
  document.querySelector('#lsearch-benchmark-ready')?.remove();
  window.__lsearchBenchmarkTimer = setTimeout(() => {
    const element = document.createElement('div');
    element.id = 'lsearch-benchmark-ready';
    element.dataset.ready = 'true';
    document.body.appendChild(element);
  }, 75);
  return 'armed';
})()"""
DELAYED_DOM_SELECTOR = '#lsearch-benchmark-ready[data-ready="true"]'


def invoke_delayed_wait(binary: Path, env: dict, encoder=None) -> dict:
    """Time both CLI processes: replace old marker, arm timer, wait until ready."""
    started = time.perf_counter_ns()
    arm = invoke(binary, ["eval", DELAYED_DOM_ARM, "--timeout", "3000"],
                 env, "delayed_dom_arm", encoder)
    if not arm["ok"]:
        return {**arm, "failed_step": "arm", "command_count": 1}
    ready = invoke(binary, ["wait", "--selector", DELAYED_DOM_SELECTOR, "--timeout", "3000"],
                   env, "delayed_dom_ready", encoder)
    row = {"ok": ready["ok"], "elapsed_ms": round((time.perf_counter_ns() - started) / 1e6, 3),
           "stdout_bytes": arm["stdout_bytes"] + ready["stdout_bytes"], "command_count": 2,
           "arm_ms": arm["elapsed_ms"], "wait_ms": ready["elapsed_ms"]}
    if not ready["ok"]:
        row.update(error=ready["error"], failed_step="wait")
    if arm.get("response_tokens") is not None and ready.get("response_tokens") is not None:
        row["response_tokens"] = arm["response_tokens"] + ready["response_tokens"]
    return row


class Fixture(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        body = (b"<!doctype html><html><head><title>Benchmark fixture</title></head>"
                b"<body><main><h1>Benchmark fixture</h1><p>Public synthetic content. "
                b"No cookies or account data.</p><a href='/docs'>Documentation</a>"
                b"</main></body></html>")
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, *_):
        pass


def free_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def source_hash(root: Path) -> str:
    digest = hashlib.sha256()
    paths = [root / "Cargo.toml", root / "Cargo.lock", *sorted((root / "src").rglob("*.rs"))]
    for path in paths:
        digest.update(str(path.relative_to(root)).encode())
        digest.update(b"\0")
        digest.update(path.read_bytes())
        digest.update(b"\0")
    return digest.hexdigest()


def cache_fingerprint(env: dict) -> tuple:
    root = Path(env["LOCAL_SEARCH_CACHE_DIR"])
    return tuple((str(path.relative_to(root)), path.stat().st_mtime_ns, path.stat().st_size)
                 for path in sorted(root.rglob("*.json")))


SYNTHETIC_QUERY = "local-search synthetic benchmark fixture"


def seed_synthetic_cache(env: dict, scope: str) -> None:
    # This is the documented v0.2.0/candidate cache format, not a web result.
    value = 0xcbf29ce484222325
    for byte in (scope + "\0google\0" + SYNTHETIC_QUERY).encode():
        value = ((value ^ byte) * 0x100000001b3) & ((1 << 64) - 1)
    cache = Path(env["LOCAL_SEARCH_CACHE_DIR"])
    cache.mkdir(parents=True, exist_ok=True)
    record = {"scope": scope, "engine": "google", "query": SYNTHETIC_QUERY,
              "search": {"blocked": False, "results": [
                  {"rank": rank, "title": f"Synthetic fixture {rank}",
                   "domain": "example.invalid", "url": f"https://example.invalid/{rank}",
                   "snippet": "Synthetic cache fixture. No network search was performed."}
                  for rank in range(1, 4)]}}
    (cache / f"{value:016x}.json").write_text(json.dumps(record))


def is_blocked(row: dict) -> bool:
    return row.get("error") in {"blocked_or_missing_blocked_flag", "search_blocked"}


def skipped(reason: str) -> dict:
    return {"ok": False, "skipped": True, "skip_reason": reason}


def initial_fixture_target(binary: Path, env: dict, fixture_url: str) -> str:
    response = subprocess.run([str(binary), "tabs", "list"], env=env, capture_output=True,
                              timeout=10, check=False)
    if response.returncode:
        raise ValueError("fixture_tabs_failed")
    payload = json.loads(response.stdout)
    targets = [tab["targetId"] for tab in payload.get("tabs", [])
               if tab.get("url") == fixture_url and isinstance(tab.get("targetId"), str)]
    if len(targets) != 1:
        raise ValueError("fixture_tab_not_unique")
    return targets[0]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, required=True)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--samples", type=int, default=20)
    parser.add_argument("--warmups", type=int, default=3)
    parser.add_argument("--live", action="store_true", help="owned Chrome plus localhost fixture")
    parser.add_argument("--search", action="store_true", help="public network searches; implies --live")
    parser.add_argument("--engine", choices=["google", "bing", "brave", "duckduckgo"], default="google")
    parser.add_argument("--query", default="rust async cancellation")
    parser.add_argument("--browser-path", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if args.samples < 1 or args.warmups < 0:
        parser.error("samples must be positive and warmups nonnegative")
    binaries = {"baseline": args.baseline.resolve(), "candidate": args.candidate.resolve()}
    if any(not path.is_file() for path in binaries.values()):
        parser.error("both binaries must exist")
    checkout = Path(__file__).resolve().parents[1]
    started_at = datetime.now(timezone.utc).isoformat()
    source_before = source_hash(checkout)
    binary_before = {name: {"sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                            "bytes": path.stat().st_size} for name, path in binaries.items()}
    revision = subprocess.run(["git", "rev-parse", "HEAD"], cwd=checkout,
                              capture_output=True, text=True, check=False).stdout.strip()
    dirty = subprocess.run(["git", "status", "--porcelain"], cwd=checkout,
                           capture_output=True, text=True, check=False).stdout != ""
    encoder = None
    try:
        import tiktoken
        encoder = tiktoken.get_encoding("o200k_base")
    except ImportError:
        pass
    rows, setup, cleanup = [], {}, {}
    fixture_targets = {}
    blocked = set()
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Fixture)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    fixture_url = f"http://127.0.0.1:{server.server_port}/"
    with tempfile.TemporaryDirectory(prefix="lsearch-ab-") as temporary:
        root = Path(temporary)
        envs = {name: isolated_env(root / name) for name in binaries}
        ports = {name: free_port() for name in binaries}
        scenarios = {"cli_version": ["--version"]}
        try:
            if args.live or args.search:
                for name, binary in binaries.items():
                    command = ["launch", "--headless", "--port", str(ports[name]), "--url", fixture_url]
                    if args.browser_path:
                        command += ["--browser-path", str(args.browser_path)]
                    setup[name] = invoke(binary, command, envs[name], "setup")
                    if setup[name]["ok"]:
                        try:
                            with urllib.request.urlopen(f"http://127.0.0.1:{ports[name]}/json/version", timeout=3) as response:
                                version = json.load(response)
                            setup[name]["browser_version"] = version.get("Browser")
                            setup[name]["protocol_version"] = version.get("Protocol-Version")
                            scope = version.get("webSocketDebuggerUrl")
                            if not isinstance(scope, str) or not scope.startswith("ws://127.0.0.1:"):
                                raise ValueError("invalid_owned_endpoint")
                            seed_synthetic_cache(envs[name], scope)
                            fixture_targets[name] = initial_fixture_target(binary, envs[name], fixture_url)
                        except (OSError, ValueError):
                            setup[name].update(ok=False, error="owned_endpoint_metadata_unavailable")
                if all(row["ok"] for row in setup.values()):
                    scenarios.update(local_eval=["eval", "6 * 7"],
                                     local_read=["read", fixture_url, "--format", "json"])
                    scenarios["delayed_dom_wait"] = []  # Two CLI calls, measured together below.
                    scenarios["synthetic_cache_hit"] = [
                        "search", SYNTHETIC_QUERY, "--engine", "google", "--limit", "3",
                        "--cache-ttl", "86400", "--json", "--timeout", "1000"]
                    if args.search:
                        search = ["search", args.query, "--engine", args.engine, "--limit", "3", "--json"]
                        scenarios["search_cold"] = [*search, "--no-cache"]
                        scenarios["search_cached"] = [*search, "--cache-ttl", "86400"]
            for scenario, command in scenarios.items():
                restore_targets = {}
                if scenario == "delayed_dom_wait":
                    for name, binary in binaries.items():
                        config = json.loads((Path(envs[name]["LOCAL_SEARCH_CONFIG_DIR"]) / "config.json").read_text())
                        restore_targets[name] = config.get("target_id")
                        selected = invoke(binary, ["tabs", "use", fixture_targets[name]], envs[name], "setup")
                        if selected["ok"]:
                            selected = invoke(binary, ["eval", "document.visibilityState"], envs[name], "fixture_visible")
                        setup[name + "_wait_visible"] = selected
                # Warmups are separately recorded, never folded into measured latency.
                for repetition in range(args.warmups + args.samples):
                    names = list(binaries) if repetition % 2 == 0 else list(reversed(binaries))
                    for name in names:
                        if scenario == "delayed_dom_wait" and not setup[name + "_wait_visible"]["ok"]:
                            rows.append({"binary": name, "scenario": scenario,
                                         "sample": repetition - args.warmups,
                                         "warmup": repetition < args.warmups, **skipped("fixture_not_visible")})
                            continue
                        is_public_search = scenario.startswith("search_")
                        is_cached = scenario in {"search_cached", "synthetic_cache_hit"}
                        if is_public_search and name in blocked:
                            row = skipped("earlier_search_blocked")
                            rows.append({"binary": name, "scenario": scenario,
                                         "sample": repetition - args.warmups,
                                         "warmup": repetition < args.warmups, **row})
                            continue
                        if scenario == "search_cached" and repetition == 0:
                            # Cache preparation has the same depth and exact query.
                            before_prime = cache_fingerprint(envs[name])
                            prime = invoke(binaries[name], command, envs[name], scenario, encoder)
                            if prime["ok"] and cache_fingerprint(envs[name]) == before_prime:
                                prime.update(ok=False, error="cache_not_created")
                            setup[name + "_cache_prime"] = prime
                            if is_blocked(prime):
                                blocked.add(name)
                        if scenario == "search_cached" and not setup[name + "_cache_prime"]["ok"]:
                            row = skipped("cache_prime_failed")
                        else:
                            cache_before = cache_fingerprint(envs[name]) if is_cached else None
                            row = (invoke_delayed_wait(binaries[name], envs[name], encoder)
                                   if scenario == "delayed_dom_wait" else
                                   invoke(binaries[name], command, envs[name], scenario, encoder))
                            if is_cached and row["ok"]:
                                unchanged = cache_before == cache_fingerprint(envs[name])
                                row["primed_cache_unchanged"] = unchanged
                                if not unchanged:
                                    row.update(ok=False, error="unexpected_cache_rewrite")
                        if is_public_search and is_blocked(row):
                            blocked.add(name)
                        rows.append({"binary": name, "scenario": scenario,
                                     "sample": repetition - args.warmups,
                                     "warmup": repetition < args.warmups, **row})
                for name, target in restore_targets.items():
                    if target:
                        setup[name + "_wait_restore"] = invoke(binaries[name], ["tabs", "use", target], envs[name], "setup")
        finally:
            if args.live or args.search:
                for name, binary in binaries.items():
                    # Only our temporary config and port are in scope. Never force-kill.
                    cleanup[name] = invoke(binary, ["cleanup", "--kill", "--port", str(ports[name])],
                                           envs[name], "cleanup")
            server.shutdown()
            server.server_close()
    measured = [row for row in rows if not row["warmup"]]
    source_after = source_hash(checkout)
    binary_after = {name: {"sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                           "bytes": path.stat().st_size} for name, path in binaries.items()}
    changed = source_before != source_after or binary_before != binary_after
    output = {
        "schema_version": 2,
        "recorded_at": datetime.now(timezone.utc).isoformat(),
        "started_at": started_at,
        "environment": {"os": platform.system(), "os_release": platform.release(),
                        "machine": platform.machine(), "python": platform.python_version()},
        "source_context": {"checkout_head": revision, "checkout_dirty": dirty,
                           "checkout_rust_source_sha256": source_before,
                           "checkout_rust_source_sha256_end": source_after,
                           "changed_during_run": changed,
                           "note": "Binary hashes identify the measured builds; HEAD alone does not describe uncommitted candidates."},
        "binaries": binary_before,
        "binaries_end": binary_after,
        "method": {"samples": args.samples, "warmups": args.warmups, "order": "alternating A/B then B/A",
                   "tokenizer": "o200k_base" if encoder else "unavailable; token metrics omitted",
                   "browser": "isolated headless managed profile per binary" if args.live or args.search else None,
                   "local_fixture": "synthetic localhost HTML, not live web performance",
                   "delayed_dom_wait": "verified visible initial fixture tab; two CLI processes timed together: eval cancels old timer/removes marker and requests 75ms DOM insertion (browser delay not guaranteed); wait --selector requires the ready marker; prior work tab restored afterward",
                   "synthetic_cache_hit": "manually seeded three-result fixture in a live-browser-scoped cache; verifies connected browser but performs no live search; not a search quality/latency claim",
                   "blocked_policy": "first blocked public search stops further network attempts and cache priming for that binary; remaining planned samples are skipped, not failed attempts",
                   "search_engine": args.engine if args.search else None,
                   "search_query": args.query if args.search else None,
                   "cold_definition": "--no-cache; browser and OS are warm",
                   "cached_definition": "same query/depth primed once, TTL 86400s; cache JSON presence and unchanged metadata checked; no browser/network cache flush",
                   "timing": "wall time including a new CLI process; warmups excluded; linear p95 interpolation",
                   "token_scope": "stdout only; not a hosted-provider normalized-token comparison"},
        "setup": setup, "cleanup": cleanup,
        "warmup_summary": {scenario: {name: summarize([row for row in rows if row["warmup"] and row["scenario"] == scenario and row["binary"] == name])
                                       for name in binaries} for scenario in scenarios},
        "summary": {scenario: {name: summarize([row for row in measured if row["scenario"] == scenario and row["binary"] == name])
                                for name in binaries} for scenario in scenarios},
        "rows": rows,
    }
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(output, indent=2) + "\n")
    print(json.dumps(output["summary"], indent=2))
    return 2 if changed or any(not row["ok"] for row in [*rows, *setup.values(), *cleanup.values()]) else 0


if __name__ == "__main__":
    raise SystemExit(main())
