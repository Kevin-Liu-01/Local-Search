"""Offline checks for live-search accounting and chart comparability."""
import json
import subprocess
import unittest
from unittest.mock import patch

import comparison
import search


class SearchTests(unittest.TestCase):
    def test_failures_and_skips_do_not_improve_latency(self):
        summary = search.summarize([
            {"ok": True, "elapsed_ms": 100, "stdout_tokens": 50,
             "equal_budget_tokens_per_result": 20, "depth_fulfilled": True},
            {"ok": False, "elapsed_ms": 1}, {"ok": False, "skipped": True}])
        self.assertEqual(summary["latency_ms"]["median"], 100)
        self.assertEqual((summary["attempted"], summary["failed"], summary["skipped"]), (2, 1, 1))

    def test_block_stops_public_requests(self):
        self.assertTrue(search.blocked({"error": "search_blocked"}))
        self.assertTrue(search.blocked({"error": "blocked_or_missing_blocked_flag"}))
        self.assertFalse(search.blocked({"error": "timeout"}))

    def test_errors_do_not_save_browser_content(self):
        process = subprocess.CompletedProcess([], 1, b"private", json.dumps({"ok": False, "error": {
            "code": "timeout", "message": "private signed-in content"}}).encode())
        with patch.object(search.subprocess, "run", return_value=process):
            row, payload, raw = search.call("lsearch", [], {})
        self.assertEqual(row["error"], "timeout")
        self.assertNotIn("private", json.dumps(row))
        self.assertIsNone(payload)
        self.assertIsNone(raw)

    def test_chart_matches_query_and_depth_not_only_success_counts(self):
        base = {"query": "one", "limit": 3, "ok": True, "equal_budget_tokens_per_result": 50}
        local = {"binary": {"version": "test"}, "rows": [{**base, "mode": "uncached"},
                 {**base, "limit": 10, "mode": "uncached", "equal_budget_tokens_per_result": 1}]}
        hosted = {"method": {"date": "test"}, "rows": [
            {**base, "provider": p} for p in ("exa", "brave", "tavily", "firecrawl")]}
        result = comparison.compare(local, hosted)
        self.assertEqual(result["matched"], [{"query": "one", "limit": 3}])
        self.assertEqual(result["providers"][0]["tokens"], 50)
        self.assertEqual(result["providers"][0]["successful"], 2)

    def test_no_common_samples_is_not_a_zero_token_chart(self):
        with self.assertRaises(ValueError):
            comparison.compare({"rows": []}, {"rows": []})


if __name__ == "__main__":
    unittest.main()
