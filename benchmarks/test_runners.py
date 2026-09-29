"""Offline tests. No browser, provider traffic, or credentials required."""
import os
import unittest
import json
import tempfile
from pathlib import Path
from unittest.mock import patch

import hosted_search
import local_ab


class RunnerTests(unittest.TestCase):
    def test_isolation_drops_inherited_browser_overrides(self):
        with patch.dict(os.environ, {"LOCAL_SEARCH_CDP": "private", "LOCAL_BROWSER_CDP": "private"}):
            env = local_ab.isolated_env(Path("/tmp/owned"))
        self.assertNotIn("LOCAL_SEARCH_CDP", env)
        self.assertNotIn("LOCAL_BROWSER_CDP", env)
        self.assertEqual(env["LOCAL_SEARCH_CONFIG_DIR"], "/tmp/owned/config")
        self.assertEqual(env["LOCAL_SEARCH_CACHE_DIR"], "/tmp/owned/cache")

    def test_percentiles_are_interpolated(self):
        self.assertEqual(local_ab.percentile([10, 30, 20], .5), 20)
        self.assertEqual(local_ab.percentile([10, 30, 20], .95), 29)

    def test_failures_are_not_fast_samples(self):
        rows = [{"ok": True, "elapsed_ms": 20, "stdout_bytes": 50},
                {"ok": False, "elapsed_ms": 1, "stdout_bytes": 0}]
        summary = local_ab.summarize(rows)
        self.assertEqual(summary["failures"], 1)
        self.assertEqual(summary["latency_ms"]["median"], 20)

    def test_local_search_requires_valid_unblocked_results(self):
        result = {"title": "Example", "url": "https://example.com/"}
        payload = {"ok": True, "search": {"blocked": False, "results": [result] * 3}}
        self.assertIsNone(local_ab.validate(payload, "search_cold"))
        payload["search"]["blocked"] = True
        self.assertIsNotNone(local_ab.validate(payload, "search_cold"))
        payload["search"]["blocked"] = False
        payload["search"]["results"] = [result]
        self.assertEqual(local_ab.validate(payload, "search_cached"), "insufficient_results")

    def test_hosted_rejects_error_empty_and_malformed_results(self):
        cases = [({"error": "private content"}, "exa"),
                 ({"results": []}, "exa"),
                 ({"results": [{"title": "x", "url": "javascript:void(0)"}]}, "tavily"),
                 ({"ok": True, "search": {"blocked": True, "results": []}}, "lsearch")]
        for payload, provider in cases:
            with self.subTest(provider=provider), self.assertRaises(RuntimeError):
                hosted_search.provider_results(provider, payload)

    def test_errors_do_not_disclose_content_or_credentials(self):
        self.assertEqual(hosted_search.sanitized_error(ValueError("token=secret")), "ValueError")
        self.assertEqual(hosted_search.sanitized_error(RuntimeError("token=secret")), "RuntimeError")
        self.assertEqual(hosted_search.sanitized_error(RuntimeError("sk_live_secret")), "RuntimeError")
        self.assertEqual(hosted_search.sanitized_error(RuntimeError("http_status_429")), "http_status_429")

    def test_fixture_eval_has_known_value(self):
        self.assertIsNone(local_ab.validate({"ok": True, "value": 42}, "local_eval"))
        self.assertEqual(local_ab.validate({"ok": True, "value": 0}, "local_eval"), "unexpected_eval_result")

    def test_equal_snippet_budget_is_applied_after_provider_normalization(self):
        results = [{"title": "Example", "url": "https://example.com", "highlights": ["abcdef", "ghi"]}]
        self.assertEqual(hosted_search.normalize("exa", results)[0]["snippet"], "abcdef ghi")
        self.assertEqual(hosted_search.normalize("exa", results, 4)[0]["snippet"], "abcd")

    def test_failed_attempts_still_have_cost_estimates(self):
        result = hosted_search.summarize([{"ok": False, "estimated_cost": {"usd": .005}}])
        self.assertEqual(result["estimated_attempt_cost_total"], {"usd": .005})

    def test_skipped_samples_are_not_attempted_failures(self):
        summary = local_ab.summarize([local_ab.skipped("blocked"),
                                     {"ok": False, "error": "search_blocked"}])
        self.assertEqual(summary["attempted"], 1)
        self.assertEqual(summary["failures"], 1)
        self.assertEqual(summary["skipped"], 1)

    def test_stderr_keeps_only_known_code_not_message(self):
        data = json.dumps({"ok": False, "error": {"code": "timeout", "message": "private content"}}).encode()
        self.assertEqual(local_ab.stderr_code(data), "timeout")
        self.assertIsNone(local_ab.stderr_code(b'{"ok":false,"error":{"code":"sk_private"}}'))
        self.assertIsNone(local_ab.stderr_code(b'not JSON private content'))

    def test_synthetic_cache_is_scoped_and_validated(self):
        with tempfile.TemporaryDirectory() as temporary:
            env = local_ab.isolated_env(Path(temporary))
            local_ab.seed_synthetic_cache(env, "ws://127.0.0.1:9999/devtools/browser/synthetic")
            paths = list(Path(env["LOCAL_SEARCH_CACHE_DIR"]).glob("*.json"))
            self.assertEqual(len(paths), 1)
            payload = json.loads(paths[0].read_text())
            self.assertEqual(payload["scope"], "ws://127.0.0.1:9999/devtools/browser/synthetic")
            self.assertIsNone(local_ab.validate({"ok": True, "search": payload["search"]}, "synthetic_cache_hit"))
            local_ab.seed_synthetic_cache(env, "ws://127.0.0.1:9999/devtools/browser/other")
            self.assertEqual(len(local_ab.cache_fingerprint(env)), 2)

    def test_blocked_policy_recognizes_validated_flags_and_error_codes(self):
        self.assertTrue(local_ab.is_blocked({"error": "search_blocked"}))
        self.assertTrue(local_ab.is_blocked({"error": "blocked_or_missing_blocked_flag"}))
        self.assertFalse(local_ab.is_blocked({"error": "timeout"}))

    def test_delayed_dom_validates_actual_ready_result(self):
        self.assertIsNone(local_ab.validate({"ok": True, "value": "armed"}, "delayed_dom_arm"))
        self.assertEqual(local_ab.validate({"ok": True, "value": False}, "delayed_dom_arm"), "delayed_dom_not_armed")
        self.assertIsNone(local_ab.validate({"ok": True, "result": {"ok": True}}, "delayed_dom_ready"))
        self.assertEqual(local_ab.validate({"ok": True, "result": False}, "delayed_dom_ready"), "delayed_dom_not_ready")
        self.assertEqual(local_ab.validate({"ok": True, "result": True}, "delayed_dom_ready"), "delayed_dom_not_ready")
        self.assertIsNone(local_ab.validate({"ok": True, "value": "visible"}, "fixture_visible"))
        self.assertEqual(local_ab.validate({"ok": True, "value": "hidden"}, "fixture_visible"), "fixture_not_visible")
        self.assertIn("clearTimeout", local_ab.DELAYED_DOM_ARM)
        self.assertIn("?.remove()", local_ab.DELAYED_DOM_ARM)
        self.assertIn("}, 75)", local_ab.DELAYED_DOM_ARM)

    def test_delayed_dom_measures_both_commands(self):
        responses = [{"ok": True, "elapsed_ms": 10, "stdout_bytes": 10, "response_tokens": 3},
                     {"ok": True, "elapsed_ms": 80, "stdout_bytes": 20, "response_tokens": 5}]
        with patch.object(local_ab, "invoke", side_effect=responses) as run:
            row = local_ab.invoke_delayed_wait(Path("/fixture/lsearch"), {})
        self.assertTrue(row["ok"])
        self.assertEqual(row["command_count"], 2)
        self.assertEqual(row["stdout_bytes"], 30)
        self.assertEqual(row["response_tokens"], 8)
        self.assertEqual(run.call_args_list[1].args[1][:3], ["wait", "--selector", local_ab.DELAYED_DOM_SELECTOR])

    def test_delayed_dom_does_not_wait_if_arming_failed(self):
        failure = {"ok": False, "error": "timeout", "elapsed_ms": 10, "stdout_bytes": 0}
        with patch.object(local_ab, "invoke", return_value=failure) as run:
            row = local_ab.invoke_delayed_wait(Path("/fixture/lsearch"), {})
        self.assertEqual(run.call_count, 1)
        self.assertEqual(row["failed_step"], "arm")


if __name__ == "__main__":
    unittest.main()
