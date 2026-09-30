#!/usr/bin/env python3
"""Join recorded search results on query AND depth. Never compare unequal subsets."""
import argparse
import json
import statistics
from pathlib import Path


def compare(local, hosted):
    names = {"lsearch": "local-search · Bing", "exa": "Exa", "brave": "Brave Search",
             "tavily": "Tavily", "firecrawl": "Firecrawl"}
    groups = {"lsearch": [r for r in local["rows"] if r["mode"] == "uncached"]}
    groups.update({p: [r for r in hosted["rows"] if r["provider"] == p] for p in names if p != "lsearch"})
    successes = {p: {(r["query"], r["limit"]): r for r in rows if r["ok"]} for p, rows in groups.items()}
    matched = set.intersection(*(set(rows) for rows in successes.values()))
    if not matched:
        raise ValueError("No common successful query/depth pairs")
    providers = []
    for provider, label in names.items():
        values = [successes[provider][key]["equal_budget_tokens_per_result"] for key in matched]
        providers.append({"id": provider, "label": label,
                          "tokens": round(statistics.median(values), 2),
                          "successful": len(successes[provider]), "attempted": len(groups[provider])})
    return {"date": hosted["method"]["date"], "version": local["binary"]["version"],
            "metric": "Median tokens per result; common query/limit request and normalized JSON, 120-character snippet cap",
            "matched": [{"query": q, "limit": n} for q, n in sorted(matched)],
            "providers": providers}


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--local", type=Path, required=True)
    parser.add_argument("--hosted", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    data = compare(json.loads(args.local.read_text()), json.loads(args.hosted.read_text()))
    args.output.write_text(json.dumps(data, indent=2) + "\n")
