#!/usr/bin/env python3
"""Compare lsearch with live hosted structured-search APIs.

API keys are read from environment variables and are never printed or written.
Install the tokenizer with `python3 -m pip install tiktoken` before running.
"""

from __future__ import annotations

import argparse
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

import tiktoken


ROOT = Path(__file__).resolve().parents[1]
LSEARCH = ROOT / "target" / "release" / "lsearch"
ENCODING = tiktoken.get_encoding("o200k_base")
QUERIES = [
    "rust async cancellation",
    "chrome devtools protocol remote debugging",
    "tokio runtime docs.rs",
    "firecrawl alternatives",
    "github agent browser",
    "openai responses api docs",
    "best static site generator",
    "local first software",
    "princeton computer science curriculum",
    "coffee shops san francisco",
    "weather los angeles tomorrow",
    "latest sqlite release",
]
KEY_ENV = {
    "exa": ("EXA_API_KEY",),
    "brave": ("BRAVE_SEARCH_API_KEY", "BRAVE_API_KEY"),
    "tavily": ("TAVILY_API_KEY",),
    "firecrawl": ("FIRECRAWL_API_KEY",),
}
PRICING = {
    "lsearch": {"usd_per_request": 0.0},
    "exa": {
        "usd_per_request_up_to_10_results": 0.007,
        "usd_per_highlighted_page": 0.001,
        "source": "https://exa.ai/pricing",
    },
    "brave": {
        "usd_per_request": 0.005,
        "source": "https://api-dashboard.search.brave.com/documentation/pricing",
    },
    "tavily": {
        "credits_per_basic_search": 1,
        "payg_usd_per_credit": 0.008,
        "source": "https://docs.tavily.com/documentation/api-credits",
    },
    "firecrawl": {
        "credits_per_10_results_rounded_up": 2,
        "source": "https://www.firecrawl.dev/pricing",
    },
}


def token_count(value: str) -> int:
    return len(ENCODING.encode(value))


def compact_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def api_key(provider: str) -> str | None:
    return next((os.environ[name] for name in KEY_ENV[provider] if os.environ.get(name)), None)


def load_env_file(path: Path) -> None:
    if not path.exists():
        return
    allowed = {name for names in KEY_ENV.values() for name in names}
    for raw_line in path.read_text().splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        name, value = line.split("=", 1)
        name = name.strip()
        value = value.strip().strip("\"").strip("'")
        if name in allowed and value:
            os.environ.setdefault(name, value)


def http_json(
    url: str,
    *,
    method: str,
    headers: dict[str, str],
    body: dict[str, Any] | None,
) -> tuple[dict[str, Any], str, dict[str, str], float]:
    data = compact_json(body).encode() if body is not None else None
    request = urllib.request.Request(url, data=data, headers=headers, method=method)
    started = time.perf_counter()
    try:
        with urllib.request.urlopen(request, timeout=90) as response:
            raw = response.read().decode("utf-8")
            elapsed_ms = round((time.perf_counter() - started) * 1000, 1)
            return json.loads(raw), raw, dict(response.headers.items()), elapsed_ms
    except urllib.error.HTTPError as error:
        response_body = error.read().decode("utf-8", errors="replace")
        raise RuntimeError(f"HTTP {error.code}: {response_body[:500]}") from error


def run_lsearch(query: str, limit: int) -> tuple[dict[str, Any], str, float]:
    started = time.perf_counter()
    result = subprocess.run(
        [str(LSEARCH), "search", query, "--limit", str(limit)],
        cwd=ROOT,
        capture_output=True,
        text=True,
        timeout=90,
        check=False,
    )
    elapsed_ms = round((time.perf_counter() - started) * 1000, 1)
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or f"lsearch exited {result.returncode}")
    return json.loads(result.stdout), result.stdout, elapsed_ms


def run_hosted(
    provider: str, query: str, limit: int, key: str
) -> tuple[dict[str, Any], str, dict[str, str], float, dict[str, Any]]:
    if provider == "exa":
        body = {
            "query": query,
            "numResults": limit,
            "type": "fast",
            "contents": {"highlights": True},
        }
        payload, raw, headers, elapsed = http_json(
            "https://api.exa.ai/search",
            method="POST",
            headers={"Content-Type": "application/json", "x-api-key": key},
            body=body,
        )
        return payload, raw, headers, elapsed, body
    if provider == "brave":
        params = urllib.parse.urlencode({"q": query, "count": limit})
        request_shape = {"q": query, "count": limit}
        payload, raw, headers, elapsed = http_json(
            f"https://api.search.brave.com/res/v1/web/search?{params}",
            method="GET",
            headers={"Accept": "application/json", "X-Subscription-Token": key},
            body=None,
        )
        return payload, raw, headers, elapsed, request_shape
    if provider == "tavily":
        body = {
            "query": query,
            "search_depth": "basic",
            "max_results": limit,
            "include_answer": False,
            "include_raw_content": False,
            "include_images": False,
        }
        payload, raw, headers, elapsed = http_json(
            "https://api.tavily.com/search",
            method="POST",
            headers={
                "Authorization": f"Bearer {key}",
                "Content-Type": "application/json",
            },
            body=body,
        )
        return payload, raw, headers, elapsed, body
    if provider == "firecrawl":
        body = {"query": query, "limit": limit, "sources": ["web"]}
        payload, raw, headers, elapsed = http_json(
            "https://api.firecrawl.dev/v2/search",
            method="POST",
            headers={
                "Authorization": f"Bearer {key}",
                "Content-Type": "application/json",
            },
            body=body,
        )
        return payload, raw, headers, elapsed, body
    raise ValueError(f"unknown provider: {provider}")


def provider_results(provider: str, payload: dict[str, Any]) -> list[dict[str, Any]]:
    if provider == "lsearch":
        return payload.get("search", {}).get("results", [])
    if provider == "exa":
        return payload.get("results", [])
    if provider == "brave":
        return payload.get("web", {}).get("results", [])
    if provider == "tavily":
        return payload.get("results", [])
    if provider == "firecrawl":
        return payload.get("data", {}).get("web", [])
    return []


def normalize(provider: str, results: list[dict[str, Any]]) -> list[dict[str, Any]]:
    normalized = []
    for rank, item in enumerate(results, 1):
        if provider == "exa":
            snippet = " ".join(item.get("highlights") or [])
        elif provider == "brave":
            snippet = item.get("description") or ""
        elif provider == "tavily":
            snippet = item.get("content") or ""
        elif provider == "firecrawl":
            snippet = item.get("description") or ""
        else:
            snippet = item.get("snippet") or ""
        normalized.append(
            {
                "rank": rank,
                "title": item.get("title") or "",
                "url": item.get("url") or "",
                "snippet": snippet,
            }
        )
    return normalized


def usage(provider: str, payload: dict[str, Any], headers: dict[str, str]) -> Any:
    if provider == "exa":
        return payload.get("costDollars")
    if provider == "brave":
        return {
            key: value
            for key, value in headers.items()
            if key.lower().startswith("x-ratelimit") or key.lower().startswith("x-request")
        }
    if provider == "tavily":
        return payload.get("usage")
    if provider == "firecrawl":
        return {"creditsUsed": payload.get("creditsUsed")}
    return None


def estimated_cost(provider: str, limit: int) -> dict[str, float]:
    if provider == "lsearch":
        return {"usd": 0.0}
    if provider == "exa":
        return {"usd": round(0.007 + 0.001 * limit, 6)}
    if provider == "brave":
        return {"usd": 0.005}
    if provider == "tavily":
        return {"credits": 1.0, "payg_usd": 0.008}
    if provider == "firecrawl":
        return {"credits": float(2 * ((limit + 9) // 10))}
    return {}


def summarize(rows: list[dict[str, Any]]) -> dict[str, Any]:
    successful = [row for row in rows if row["ok"]]
    if not successful:
        return {"runs": len(rows), "successful": 0}

    def stats(key: str) -> dict[str, float]:
        values = [float(row[key]) for row in successful]
        return {
            "min": min(values),
            "median": statistics.median(values),
            "mean": round(statistics.mean(values), 2),
            "max": max(values),
        }

    return {
        "runs": len(rows),
        "successful": len(successful),
        "fulfilled_requested_depth": sum(
            bool(row["fulfilled_requested_depth"]) for row in successful
        ),
        "result_count": stats("result_count"),
        "raw_total_tokens": stats("raw_total_tokens"),
        "raw_tokens_per_result": stats("raw_tokens_per_result"),
        "normalized_total_tokens": stats("normalized_total_tokens"),
        "normalized_tokens_per_result": stats("normalized_tokens_per_result"),
        "latency_ms": stats("latency_ms"),
        "estimated_cost_total": sum_costs(successful),
    }


def sum_costs(rows: list[dict[str, Any]]) -> dict[str, float]:
    totals: dict[str, float] = {}
    for row in rows:
        for key, value in row["estimated_cost"].items():
            totals[key] = round(totals.get(key, 0.0) + float(value), 6)
    return totals


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--providers",
        default="lsearch,exa,brave,tavily,firecrawl",
        help="comma-separated providers",
    )
    parser.add_argument("--max-queries", type=int, default=len(QUERIES))
    parser.add_argument("--include-rows", action="store_true")
    parser.add_argument(
        "--env-file",
        type=Path,
        default=ROOT / ".env.bench.local",
        help="ignored local KEY=value file",
    )
    args = parser.parse_args()
    load_env_file(args.env_file)

    requested = [item.strip() for item in args.providers.split(",") if item.strip()]
    unknown = sorted(set(requested) - {"lsearch", *KEY_ENV})
    if unknown:
        raise SystemExit(f"unknown providers: {', '.join(unknown)}")
    if "lsearch" in requested and not LSEARCH.exists():
        raise SystemExit("build first: cargo build --release")

    missing = {
        provider: list(KEY_ENV[provider])
        for provider in requested
        if provider != "lsearch" and api_key(provider) is None
    }
    active = [provider for provider in requested if provider not in missing]
    cache_dir = None
    if "lsearch" in active:
        cache_dir = tempfile.TemporaryDirectory(prefix="lsearch-benchmark-")
        os.environ["LOCAL_SEARCH_CACHE_DIR"] = cache_dir.name
    rows: list[dict[str, Any]] = []
    for provider in active:
        for limit in (3, 10):
            for query in QUERIES[: args.max_queries]:
                request_shape: dict[str, Any] = {
                    "provider": provider,
                    "query": query,
                    "limit": limit,
                }
                try:
                    if provider == "lsearch":
                        payload, raw, elapsed = run_lsearch(query, limit)
                        headers: dict[str, str] = {}
                    else:
                        key = api_key(provider)
                        assert key is not None
                        payload, raw, headers, elapsed, options = run_hosted(
                            provider, query, limit, key
                        )
                        request_shape["options"] = options
                    results = provider_results(provider, payload)
                    normalized = normalize(provider, results)
                    request_tokens = token_count(compact_json(request_shape))
                    row = {
                        "provider": provider,
                        "query": query,
                        "limit": limit,
                        "ok": len(results) > 0,
                        "fulfilled_requested_depth": len(results) >= limit,
                        "result_count": len(results),
                        "request_tokens": request_tokens,
                        "raw_response_tokens": token_count(raw),
                        "raw_total_tokens": request_tokens + token_count(raw),
                        "raw_tokens_per_result": round(
                            (request_tokens + token_count(raw)) / len(results), 2
                        )
                        if results
                        else 0.0,
                        "normalized_response_tokens": token_count(compact_json(normalized)),
                        "normalized_total_tokens": request_tokens
                        + token_count(compact_json(normalized)),
                        "normalized_tokens_per_result": round(
                            (
                                request_tokens
                                + token_count(compact_json(normalized))
                            )
                            / len(results),
                            2,
                        )
                        if results
                        else 0.0,
                        "latency_ms": elapsed,
                        "usage": usage(provider, payload, headers),
                        "estimated_cost": estimated_cost(provider, limit),
                    }
                except Exception as error:  # noqa: BLE001 - benchmark records failures
                    row = {
                        "provider": provider,
                        "query": query,
                        "limit": limit,
                        "ok": False,
                        "error": str(error),
                        "estimated_cost": estimated_cost(provider, limit),
                    }
                rows.append(row)
                print(
                    f"finished provider={provider} limit={limit} query={query!r}",
                    file=sys.stderr,
                    flush=True,
                )

    output: dict[str, Any] = {
        "method": {
            "date": "2026-07-21",
            "encoding": "o200k_base",
            "queries": QUERIES[: args.max_queries],
            "limits": [3, 10],
            "token_scope": "serialized request shape plus raw or normalized JSON response",
            "content": {
                "lsearch": "search snippets",
                "exa": "highlights",
                "brave": "descriptions",
                "tavily": "basic-search content",
                "firecrawl": "descriptions without scrapeOptions",
            },
        },
        "active_providers": active,
        "missing_credentials": missing,
        "pricing": {provider: PRICING[provider] for provider in requested},
        "summary": {
            provider: summarize([row for row in rows if row["provider"] == provider])
            for provider in active
        },
        "failures": [row for row in rows if not row["ok"]],
    }
    if args.include_rows:
        output["rows"] = rows
    print(json.dumps(output, indent=2))
    if cache_dir is not None:
        cache_dir.cleanup()
    return 0 if not output["failures"] and not missing else 2


if __name__ == "__main__":
    raise SystemExit(main())
