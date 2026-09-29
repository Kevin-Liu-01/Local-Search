#!/usr/bin/env python3
"""Compare lsearch with live hosted structured-search APIs.

API keys are read from environment variables and are never printed or written.
Install the tokenizer with `python3 -m pip install tiktoken` before running.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import statistics
import subprocess
import sys
import tempfile
import time
from datetime import date
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
LSEARCH = ROOT / "target" / "release" / "lsearch"
ENCODING = None
ENGINE = "google"
LSEARCH_ENV = None
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
        "hobby_monthly_usd_billed_yearly": 16,
        "hobby_monthly_credits": 5_000,
        "hobby_usd_per_credit_equivalent": 0.0032,
        "pay_as_you_go": False,
        "source": "https://www.firecrawl.dev/pricing",
    },
}


def token_count(value: str) -> int:
    if ENCODING is None:
        raise RuntimeError("tokenizer_unavailable")
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
        raise RuntimeError(f"http_status_{error.code}") from error


def run_lsearch(query: str, limit: int) -> tuple[dict[str, Any], str, float]:
    started = time.perf_counter()
    result = subprocess.run(
        [str(LSEARCH), "search", query, "--engine", ENGINE, "--limit", str(limit), "--no-cache", "--json"],
        cwd=ROOT,
        env=LSEARCH_ENV,
        capture_output=True,
        text=True,
        timeout=90,
        check=False,
    )
    elapsed_ms = round((time.perf_counter() - started) * 1000, 1)
    if result.returncode != 0:
        raise RuntimeError(f"lsearch_exit_{result.returncode}")
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
    if not isinstance(payload, dict) or payload.get("error") or payload.get("success") is False:
        raise RuntimeError("provider_error_envelope")
    if provider == "lsearch":
        search = payload.get("search")
        if payload.get("ok") is not True or not isinstance(search, dict):
            raise RuntimeError("malformed_search_envelope")
        if search.get("blocked") is not False:
            raise RuntimeError("blocked_or_missing_blocked_flag")
        results = search.get("results")
    elif provider in {"exa", "tavily"}:
        results = payload.get("results")
    elif provider == "brave":
        results = payload.get("web", {}).get("results")
    elif provider == "firecrawl":
        results = payload.get("data", {}).get("web")
    else:
        raise RuntimeError("unknown_provider")
    if not isinstance(results, list) or not results:
        raise RuntimeError("empty_or_malformed_results")
    for item in results:
        if (not isinstance(item, dict) or not isinstance(item.get("title"), str)
                or not item["title"] or not isinstance(item.get("url"), str)
                or not item["url"].startswith(("https://", "http://"))):
            raise RuntimeError("malformed_result")
    return results


def sanitized_error(error: Exception) -> str:
    # Never serialize exception messages from network libraries or raw responses.
    if isinstance(error, RuntimeError):
        message = str(error)
        allowed = {"provider_error_envelope", "malformed_search_envelope",
                   "blocked_or_missing_blocked_flag", "unknown_provider",
                   "empty_or_malformed_results", "malformed_result", "malformed_snippet"}
        numeric_code = any(message.startswith(prefix) and message[len(prefix):].lstrip("-").isdigit()
                           for prefix in ("http_status_", "lsearch_exit_"))
        if message in allowed or numeric_code:
            return message
    return type(error).__name__


def normalize(provider: str, results: list[dict[str, Any]], snippet_chars: int | None = None) -> list[dict[str, Any]]:
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
        if not isinstance(snippet, str):
            raise RuntimeError("malformed_snippet")
        if snippet_chars is not None:
            snippet = snippet[:snippet_chars]
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
            if key.lower().startswith("x-ratelimit")
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
        return {"runs": len(rows), "successful": 0,
                "estimated_attempt_cost_total": sum_costs(rows)}

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
        "equal_budget_total_tokens": stats("equal_budget_total_tokens"),
        "equal_budget_tokens_per_result": stats("equal_budget_tokens_per_result"),
        "estimated_attempt_cost_total": sum_costs(rows),
    }


def sum_costs(rows: list[dict[str, Any]]) -> dict[str, float]:
    totals: dict[str, float] = {}
    for row in rows:
        for key, value in row["estimated_cost"].items():
            totals[key] = round(totals.get(key, 0.0) + float(value), 6)
    return totals


def main() -> int:
    global LSEARCH, ENGINE, LSEARCH_ENV, ENCODING
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--providers",
        default="lsearch,exa,brave,tavily,firecrawl",
        help="comma-separated providers",
    )
    parser.add_argument("--max-queries", type=int, default=len(QUERIES))
    parser.add_argument("--include-rows", action="store_true")
    parser.add_argument("--binary", type=Path, default=LSEARCH)
    parser.add_argument("--engine", choices=["google", "bing", "brave", "duckduckgo"], default="google")
    parser.add_argument("--config-dir", type=Path, help="explicit isolated, already-connected local-search config")
    parser.add_argument("--output", type=Path, help="save sanitized JSON report here")
    parser.add_argument("--date", type=date.fromisoformat, default=date.today(), help="recording date, YYYY-MM-DD")
    parser.add_argument("--snippet-chars", type=int, default=120, help="equal maximum snippet budget for comparison")
    parser.add_argument(
        "--env-file",
        type=Path,
        default=ROOT / ".env.bench.local",
        help="ignored local KEY=value file",
    )
    args = parser.parse_args()
    try:
        import tiktoken
    except ImportError:
        parser.error("install tiktoken in your benchmark environment before running")
    ENCODING = tiktoken.get_encoding("o200k_base")
    LSEARCH = args.binary.resolve()
    ENGINE = args.engine
    if not 1 <= args.max_queries <= len(QUERIES):
        parser.error("max-queries must be between 1 and 12")
    if not 0 <= args.snippet_chars <= 120:
        parser.error("snippet-chars must be between 0 and the native lsearch budget of 120")
    load_env_file(args.env_file)

    requested = [item.strip() for item in args.providers.split(",") if item.strip()]
    unknown = sorted(set(requested) - {"lsearch", *KEY_ENV})
    if unknown:
        raise SystemExit(f"unknown providers: {', '.join(unknown)}")
    if "lsearch" in requested and not LSEARCH.exists():
        raise SystemExit("build first: cargo build --release")
    if "lsearch" in requested and args.config_dir is None:
        parser.error("lsearch requires --config-dir pointing to an explicitly prepared isolated profile")

    missing = {
        provider: list(KEY_ENV[provider])
        for provider in requested
        if provider != "lsearch" and api_key(provider) is None
    }
    active = [provider for provider in requested if provider not in missing]
    cache_dir = None
    if "lsearch" in active:
        cache_dir = tempfile.TemporaryDirectory(prefix="lsearch-benchmark-")
        LSEARCH_ENV = {key: value for key, value in os.environ.items()
                       if not key.startswith(("LOCAL_SEARCH_", "LOCAL_BROWSER_"))}
        LSEARCH_ENV.update(LOCAL_SEARCH_CONFIG_DIR=str(args.config_dir.resolve()),
                           LOCAL_SEARCH_CACHE_DIR=cache_dir.name, NO_COLOR="1")
    rows: list[dict[str, Any]] = []
    for provider in active:
        for limit in (3, 10):
            for query in QUERIES[: args.max_queries]:
                request_shape: dict[str, Any] = {
                    "query": query,
                    "limit": limit,
                }
                try:
                    if provider == "lsearch":
                        payload, raw, elapsed = run_lsearch(query, limit)
                        headers: dict[str, str] = {}
                        request_shape["engine"] = ENGINE
                    else:
                        key = api_key(provider)
                        assert key is not None
                        payload, raw, headers, elapsed, options = run_hosted(
                            provider, query, limit, key
                        )
                        request_shape = options
                    results = provider_results(provider, payload)
                    normalized = normalize(provider, results)
                    equal_budget = normalize(provider, results, args.snippet_chars)
                    request_tokens = token_count(compact_json(request_shape))
                    common_request_tokens = token_count(compact_json({"query": query, "limit": limit}))
                    equal_total = common_request_tokens + token_count(compact_json(equal_budget))
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
                        "common_request_tokens": common_request_tokens,
                        "equal_budget_total_tokens": equal_total,
                        "equal_budget_tokens_per_result": round(equal_total / len(results), 2),
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
                        "error": sanitized_error(error),
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
            "date": args.date.isoformat(),
            "binary_sha256": hashlib.sha256(LSEARCH.read_bytes()).hexdigest() if "lsearch" in active else None,
            "engine": ENGINE if "lsearch" in active else None,
            "local_cache": "disabled for every request with --no-cache; not mixed cold/cache latency",
            "provider_order": "sequential; not randomized; provider/order/network effects remain",
            "encoding": "o200k_base",
            "queries": QUERIES[: args.max_queries],
            "limits": [3, 10],
            "token_scope": "serialized request shape plus raw or normalized JSON response",
            "native_tokens": "actual provider request object once plus native snippets/highlights; different content budgets",
            "equal_budget_tokens": "common {query,limit} request once plus normalized results with each snippet truncated to the same maximum",
            "equal_snippet_chars": args.snippet_chars,
            "equal_budget_caveat": "Equal maximum length does not equalize relevance, source ranking, or actual content length.",
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
        "pricing_date": "2026-07-21",
        "pricing_note": "Historical estimates, not verified current prices or an actual invoice. Failed requests may incur charges.",
        "summary": {
            provider: summarize([row for row in rows if row["provider"] == provider])
            for provider in active
        },
        "failures": [row for row in rows if not row["ok"]],
    }
    if args.include_rows:
        output["rows"] = rows
    print(json.dumps(output, indent=2))
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(output, indent=2) + "\n")
    if cache_dir is not None:
        cache_dir.cleanup()
    return 0 if not output["failures"] and not missing else 2


if __name__ == "__main__":
    raise SystemExit(main())
