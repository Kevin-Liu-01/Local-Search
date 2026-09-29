"""Execute the shipped extraction JS against small, deterministic DOM fixtures.

Run with: python3 -m unittest discover -s benchmarks/tests -v
Requires Node.js but no Python or JavaScript packages. These fixtures exercise
the DOM contract, not browser rendering or live search-engine compatibility.
"""

from __future__ import annotations

import base64
import json
from pathlib import Path
import re
import shutil
import subprocess
import unittest
from urllib.parse import quote


ROOT = Path(__file__).resolve().parents[2]
NODE = shutil.which("node")


def rust_script(name: str) -> str:
    source = (ROOT / "src/browser/scripts.rs").read_text()
    function = source.split(f"pub fn {name}(", 1)[1]
    match = re.search(r'r#"(.*?)"#', function, re.DOTALL)
    if match is None:
        raise AssertionError(f"missing raw JavaScript for {name}")
    return match.group(1)


def readiness(query: str, limit: int) -> str:
    # Rust format! uses the same {}, {{ and }} substitutions as str.format.
    # Inserted argument text is not interpreted as additional format syntax.
    return rust_script("search_ready").format(
        rust_script("search_results"), json.dumps(query, ensure_ascii=False), limit
    )


HARNESS = r"""
const vm = require('node:vm');
const input = JSON.parse(require('node:fs').readFileSync(0, 'utf8'));
const selectors = {
  google: 'a h3', bing: 'li.b_algo h2 a',
  brave: '.snippet[data-type="web"] a.l1', duckduckgo: '.result__a',
};
let state = input.fixture;
function anchor(item) {
  const container = {
    innerText: item.title + ' ' + (item.snippet || ''),
    querySelector: () => ({innerText: item.snippet || ''}),
  };
  const a = {
    href: new URL(item.url, state.href).href,
    innerText: item.title, textContent: item.title,
    querySelector: () => ({innerText: item.title}),
    closest: () => container, parentElement: container,
  };
  return a;
}
const document = {
  get title() { return state.title || 'Fixture search'; },
  get readyState() { return state.readyState || 'complete'; },
  get body() { return {innerText: state.text || ''}; },
  querySelector: () => state.challenge ? {} : null,
  querySelectorAll(selector) {
    if (selector !== selectors[state.engine]) return [];
    const links = (state.results || []).map(anchor);
    return state.engine === 'google' ? links.map(a => ({closest: () => a})) : links;
  },
};
const context = vm.createContext({
  URL, TextDecoder, Uint8Array,
  atob: value => Buffer.from(value, 'base64').toString('binary'),
  document, location: {get href() {return state.href;}},
  performance: {now: () => state.time || 0},
});
const output = [];
for (const update of input.steps || [{}]) {
  state = {...state, ...update};
  output.push(vm.runInContext(input.script, context, {timeout: 1000}));
}
process.stdout.write(JSON.stringify(output));
"""


def result(index: int = 1, **overrides: str) -> dict[str, str]:
    return {
        "url": f"https://example.org/guide/{index}",
        "title": f"Rust guide {index}",
        "snippet": "A short Rust reference.",
        **overrides,
    }


def fixture(engine: str = "google", query: str = "rust", **overrides):
    origins = {
        "google": "https://www.google.com/search",
        "bing": "https://www.bing.com/search",
        "brave": "https://search.brave.com/search",
        "duckduckgo": "https://html.duckduckgo.com/html/",
    }
    return {
        "engine": engine,
        "href": f"{origins[engine]}?q={quote(query)}",
        "results": [result()],
        **overrides,
    }


@unittest.skipUnless(NODE, "Node.js is required to execute the shipped browser scripts")
class SearchScriptsTest(unittest.TestCase):
    def execute(self, data, *, script=None, steps=None):
        completed = subprocess.run(
            [NODE, "-e", HARNESS],
            input=json.dumps({
                "fixture": data,
                "script": script or rust_script("search_results"),
                "steps": steps,
            }),
            text=True,
            capture_output=True,
            check=True,
            timeout=5,
        )
        return json.loads(completed.stdout)

    def test_supported_engine_selectors_return_ranked_results(self):
        for engine in ("google", "bing", "brave", "duckduckgo"):
            with self.subTest(engine=engine):
                page = self.execute(fixture(engine))[0]
                self.assertFalse(page["blocked"])
                self.assertEqual(
                    page["results"], [{"rank": 1, "domain": "example.org", **result()}]
                )

    def test_captcha_query_and_result_text_are_not_a_challenge(self):
        data = fixture(
            query="captcha bot detection",
            title="captcha bot detection - Google Search",
            text="Solve the challenge: CAPTCHA design documentation",
            results=[result(title="CAPTCHA and bot detection explained")],
        )
        page = self.execute(data)[0]
        self.assertFalse(page["blocked"])
        self.assertEqual(len(page["results"]), 1)
        self.assertEqual(
            self.execute(data, script=readiness("captcha bot detection", 1)), [True]
        )

    def test_usable_results_take_precedence_over_embedded_challenge_widget(self):
        self.assertFalse(self.execute(fixture(challenge=True))[0]["blocked"])

    def test_empty_captcha_query_is_not_itself_blocked(self):
        page = self.execute(fixture(
            query="captcha", title="captcha - Search",
            text="No results for captcha", results=[],
        ))[0]
        self.assertFalse(page["blocked"])
        self.assertEqual(page["results"], [])

    def test_genuine_challenge_completes_readiness_as_blocked(self):
        for changes in (
            {"challenge": True},
            {"text": "Our systems have detected unusual traffic from your computer network."},
            {"href": "https://www.google.com/sorry/index"},
        ):
            with self.subTest(changes=changes):
                data = fixture(results=[], **changes)
                self.assertTrue(self.execute(data)[0]["blocked"])
                self.assertEqual(
                    self.execute(data, script=readiness("rust", 10)), [True]
                )

    def test_external_fragment_destinations_are_preserved(self):
        urls = [
            "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules",
            "https://example.org/search?q=rust#details",
        ]
        page = self.execute(fixture(
            results=[result(i, url=url) for i, url in enumerate(urls)]
        ))[0]
        self.assertEqual([item["url"] for item in page["results"]], urls)

    def test_unsafe_protocols_and_engine_navigation_are_excluded(self):
        urls = [
            "javascript:alert(1)", "data:text/plain,hello", "file:///tmp/local",
            "mailto:a@example.org", "ftp://example.org/",
            "https://www.google.com/search?q=other",
        ]
        page = self.execute(fixture(
            results=[result(i, url=url) for i, url in enumerate(urls)] + [result()]
        ))[0]
        self.assertEqual(
            [item["url"] for item in page["results"]], [result()["url"]]
        )

    def test_unicode_bing_redirect_is_decoded_as_utf8(self):
        destination = "https://example.org/日本語?q=café#説明"
        encoded = base64.urlsafe_b64encode(destination.encode()).decode().rstrip("=")
        page = self.execute(fixture("bing", results=[
            result(url=f"https://www.bing.com/ck/a?u=a1{encoded}")
        ]))[0]
        expected = (
            "https://example.org/%E6%97%A5%E6%9C%AC%E8%AA%9E"
            "?q=caf%C3%A9#%E8%AA%AC%E6%98%8E"
        )
        self.assertEqual(page["results"][0]["url"], expected)

    def test_duckduckgo_redirect_is_unwrapped_before_protocol_check(self):
        destination = "https://example.org/guide#details"
        links = [
            result(url=f"https://duckduckgo.com/l/?uddg={quote(destination, safe='')}"),
            result(2, url="https://duckduckgo.com/l/?uddg=javascript%3Aalert(1)"),
        ]
        page = self.execute(fixture("duckduckgo", results=links))[0]
        self.assertEqual([item["url"] for item in page["results"]], [destination])

    def test_duplicates_are_removed_and_ranks_remain_contiguous(self):
        page = self.execute(fixture(results=[result(), result(), result(2)]))[0]
        self.assertEqual([item["rank"] for item in page["results"]], [1, 2])

    def test_requested_ten_results_wait_for_hydration(self):
        stages = [
            {"time": 0, "readyState": "loading",
             "results": [result(i) for i in range(3)]},
            {"time": 100, "readyState": "interactive",
             "results": [result(i) for i in range(7)]},
            {"time": 150, "readyState": "interactive",
             "results": [result(i) for i in range(10)]},
        ]
        self.assertEqual(
            self.execute(fixture(), script=readiness("rust", 10), steps=stages),
            [False, False, True],
        )

    def test_settled_page_can_finish_with_zero_or_fewer_results(self):
        for count in (0, 2):
            with self.subTest(count=count):
                data = fixture(
                    results=[result(i) for i in range(count)],
                    text="No results found" if count == 0 else "Rust search results",
                )
                self.assertEqual(
                    self.execute(data, script=readiness("rust", 10), steps=[
                        {"time": 0}, {"time": 249}, {"time": 250},
                    ]),
                    [False, False, True],
                )

    def test_changed_result_count_resets_settling_window(self):
        stages = [
            {"time": 0}, {"time": 200, "results": [result(), result(2)]},
            {"time": 449}, {"time": 450},
        ]
        self.assertEqual(
            self.execute(fixture(), script=readiness("rust", 10), steps=stages),
            [False, False, False, True],
        )

    def test_empty_loading_shell_is_not_a_completed_search(self):
        for ready_state in ("loading", "interactive", "complete"):
            with self.subTest(ready_state=ready_state):
                self.assertEqual(
                    self.execute(
                        fixture(results=[], readyState=ready_state),
                        script=readiness("rust", 10),
                        steps=[{"time": 0}, {"time": 250}, {"time": 1000}],
                    ),
                    [False, False, False],
                )

    def test_different_query_never_reuses_old_readiness(self):
        self.assertEqual(
            self.execute(
                fixture(query="old", results=[result(i) for i in range(10)]),
                script=readiness("new", 10),
                steps=[{"time": 0}, {"time": 1000}],
            ),
            [False, False],
        )


if __name__ == "__main__":
    unittest.main()
