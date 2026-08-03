#!/usr/bin/env node

"use strict";

const { spawnSync } = require("node:child_process");
const { existsSync } = require("node:fs");
const path = require("node:path");

const executable = process.platform === "win32" ? "lsearch.exe" : "lsearch";
const nativeBinary = path.resolve(__dirname, "..", "vendor", "bin", executable);

if (!existsSync(nativeBinary)) {
  console.error(
    [
      "localsearch: the native lsearch binary is missing.",
      "Run `npm rebuild @kevinliu01/localsearch` with Rust/Cargo installed.",
      "If lifecycle scripts were disabled, reinstall without `--ignore-scripts`.",
    ].join("\n"),
  );
  process.exit(1);
}

const result = spawnSync(nativeBinary, process.argv.slice(2), {
  env: process.env,
  stdio: "inherit",
});

if (result.error) {
  console.error(`localsearch: failed to start native lsearch: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status ?? 1);
