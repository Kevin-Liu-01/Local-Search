#!/usr/bin/env python3
"""Live release benchmark: isolated Chrome, empty local cache, then verified reuse.

Only aggregate measurements and public query strings are saved, never page content.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import statistics
import subprocess
import tempfile
import time
import urllib.request
from datetime import datetime, timezone
from pathlib import Path

from hosted_search import QUERIES, compact_json, normalize, provider_results
from local_ab import cache_fingerprint, free_port, isolated_env, percentile, stderr_code


def call(binary, args, env):
    start = time.perf_counter()
    try:
        result = subprocess.run([str(binary), *args], env=env, capture_output=True,
                                timeout=65, check=False)
        elapsed = round((time.perf_counter() - start) * 1000, 3)
        if result.returncode:
            return {"ok": False, "elapsed_ms": elapsed,
                    "error": stderr_code(result.stderr) or "process_failed"}, None, None
        payload = json.loads(result.stdout)
        if not isinstance(payload, dict) or payload.get("ok") is not True:
            return {"ok": False, "elapsed_ms": elapsed, "error": "invalid_envelope"}, None, None
        return {"ok": True, "elapsed_ms": elapsed}, payload, result.stdout.decode()
    except subprocess.TimeoutExpired:
        return {"ok": False, "error": "process_timeout"}, None, None
    except (ValueError, OSError):
        return {"ok": False, "error": "invalid_response"}, None, None


def summarize(rows):
    attempted = [row for row in rows if not row.get("skipped")]
    good = [row for row in attempted if row["ok"]]
    report = {"planned": len(rows), "attempted": len(attempted), "usable": len(good),
              "failed": len(attempted) - len(good), "skipped": len(rows) - len(attempted),
              "depth_fulfilled": sum(row.get("depth_fulfilled", False) for row in good)}
    if good:
        values = [row["elapsed_ms"] for row in good]
        report["latency_ms"] = {"median": round(statistics.median(values), 3),
                                "p95": percentile(values, .95)}
        for key in ["stdout_tokens", "equal_budget_tokens_per_result"]:
            report["median_" + key] = round(statistics.median(row[key] for row in good), 2)
    return report


def blocked(row):
    return row.get("error") in {"search_blocked", "blocked_or_missing_blocked_flag"}


def measure(binary, command, env, encoding, query, limit):
    row, payload, raw = call(binary, command, env)
    if row["ok"]:
        try:
            results = provider_results("lsearch", payload)
            common = compact_json({"query": query, "limit": limit})
            tokens = len(encoding.encode(common)) + len(encoding.encode(compact_json(normalize("lsearch", results, 120))))
            row.update(result_count=len(results), depth_fulfilled=len(results) >= limit,
                       stdout_tokens=len(encoding.encode(raw)),
                       equal_budget_tokens_per_result=round(tokens / len(results), 2))
        except RuntimeError as error:
            # provider_results emits only known fixed codes, never response text.
            row.update(ok=False, error=str(error))
    return row, payload


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--engines", nargs="+", choices=["bing", "google", "brave", "duckduckgo"],
                        default=["bing", "google", "brave", "duckduckgo"])
    parser.add_argument("--max-queries", type=int, default=12)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    if not 1 <= args.max_queries <= len(QUERIES):
        parser.error("max-queries must be between 1 and 12")
    import tiktoken
    encoding = tiktoken.get_encoding("o200k_base")
    binary = args.binary.resolve()
    digest = hashlib.sha256(binary.read_bytes()).hexdigest()
    version = subprocess.check_output([str(binary), "--version"], text=True).strip()
    report = {"recorded_at": datetime.now(timezone.utc).isoformat(),
              "binary": {"version": version, "sha256": digest, "bytes": binary.stat().st_size},
              "environment": {"os": platform.system(), "os_release": platform.release(), "machine": platform.machine(),
                              "load_average_start": os.getloadavg()},
              "method": {"queries": QUERIES[:args.max_queries], "limits": [3, 10],
                         "uncached": "empty local-search cache for each query/depth; browser and OS caches not flushed",
                         "cached": "immediate identical repeat; results equal, cache files present and unchanged",
                         "latency": "CLI wall time including process startup; browser launch excluded; one sample per query/depth",
                         "order": "engines sequential; depth 3 then 10; uncached then cached for each query",
                         "tokenizer": "o200k_base", "equal_budget_tokens": "common query/limit request plus rank/title/url/snippet JSON; max 120 snippet characters",
                         "blocked_policy": "stop an engine after its first blocked request; no retries or bypass",
                         "profile": "fresh isolated headless Chrome per engine; no personal logins"},
              "engines": {}, "rows": []}
    with tempfile.TemporaryDirectory(prefix="lsearch-release-search-") as temp:
        root = Path(temp)
        for engine in args.engines:
            env = isolated_env(root / engine)
            port = free_port()
            setup, _, _ = call(binary, ["launch", "--headless", "--port", str(port)], env)
            entry = {"setup": setup}
            report["engines"][engine] = entry
            stop = None if setup["ok"] else "setup_failed"
            try:
                if setup["ok"]:
                    with urllib.request.urlopen(f"http://127.0.0.1:{port}/json/version", timeout=5) as response:
                        entry["browser"] = json.load(response).get("Browser")
                for limit in (3, 10):
                    for index, query in enumerate(QUERIES[:args.max_queries]):
                        base = {"engine": engine, "query": query, "limit": limit}
                        if stop:
                            for mode in ("uncached", "cached"):
                                report["rows"].append({**base, "mode": mode, "ok": False, "skipped": True, "reason": stop})
                            continue
                        env["LOCAL_SEARCH_CACHE_DIR"] = str(root / engine / f"cache-{limit}-{index}")
                        command = ["search", query, "--engine", engine, "--limit", str(limit), "--cache-ttl", "86400", "--json"]
                        cold, payload = measure(binary, command, env, encoding, query, limit)
                        report["rows"].append({**base, "mode": "uncached", **cold})
                        if cold["ok"]:
                            before = cache_fingerprint(env)
                            warm, cached = measure(binary, command, env, encoding, query, limit)
                            if warm["ok"] and (not before or before != cache_fingerprint(env) or payload["search"]["results"] != cached["search"]["results"]):
                                warm.update(ok=False, error="cache_reuse_not_verified")
                            report["rows"].append({**base, "mode": "cached", **warm})
                            if blocked(warm):
                                stop = "blocked"
                        else:
                            report["rows"].append({**base, "mode": "cached", "ok": False, "skipped": True, "reason": "uncached_failed"})
                            if blocked(cold):
                                stop = "blocked"
                        print(f"{engine} {limit} results query {index+1}: {'ok' if cold['ok'] else cold['error']}", flush=True)
            finally:
                entry["cleanup"], _, _ = call(binary, ["cleanup", "--kill", "--port", str(port)], env)
                for mode in ("uncached", "cached"):
                    entry[mode] = summarize([row for row in report["rows"] if row["engine"] == engine and row["mode"] == mode])
                # Retain completed evidence even if a later engine fails.
                args.output.parent.mkdir(parents=True, exist_ok=True)
                args.output.write_text(json.dumps(report, indent=2) + "\n")
    report["binary"]["unchanged"] = digest == hashlib.sha256(binary.read_bytes()).hexdigest()
    report["environment"]["load_average_end"] = os.getloadavg()
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    return 0 if report["binary"]["unchanged"] and all(row["ok"] for row in report["rows"]) and all(e["cleanup"]["ok"] for e in report["engines"].values()) else 2


if __name__ == "__main__":
    raise SystemExit(main())
