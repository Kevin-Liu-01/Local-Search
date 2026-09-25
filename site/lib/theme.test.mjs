import assert from "node:assert/strict";
import { test } from "node:test";
import { runInNewContext } from "node:vm";
import { THEME_INIT_SCRIPT, THEME_STORAGE_KEY } from "./theme.ts";

function boot({ saved = null, dark = false, blocked = false } = {}) {
  const root = { dataset: {} };
  runInNewContext(THEME_INIT_SCRIPT, {
    document: { documentElement: root },
    localStorage: {
      getItem(key) {
        assert.equal(key, THEME_STORAGE_KEY);
        if (blocked) throw new Error("Storage is blocked");
        return saved;
      },
    },
    matchMedia(query) {
      assert.equal(query, "(prefers-color-scheme: dark)");
      return { matches: dark };
    },
  });
  return root.dataset.theme;
}

test("first visits follow the system before the page paints", () => {
  assert.equal(boot(), "light");
  assert.equal(boot({ dark: true }), "dark");
});

test("a saved theme wins over the system preference", () => {
  assert.equal(boot({ saved: "light", dark: true }), "light");
  assert.equal(boot({ saved: "dark" }), "dark");
});

test("unknown saved values fall back to the system", () => {
  for (const saved of ["", "system", "null", "unexpected"]) {
    assert.equal(boot({ saved }), "light");
    assert.equal(boot({ saved, dark: true }), "dark");
  }
});

test("blocked storage does not prevent the page from loading", () => {
  assert.equal(boot({ blocked: true }), "light");
  assert.equal(boot({ blocked: true, dark: true }), "dark");
});
