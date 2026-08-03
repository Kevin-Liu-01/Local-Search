#!/usr/bin/env node

"use strict";

const { spawnSync } = require("node:child_process");
const { existsSync, mkdirSync } = require("node:fs");
const path = require("node:path");

const packageJson = require("../package.json");
const packageRoot = path.resolve(__dirname, "..");
const cargoRoot = path.join(packageRoot, "vendor");
const nativeVersion = packageJson.config.nativeVersion;
const executable = process.platform === "win32" ? "lsearch.exe" : "lsearch";
const nativeBinary = path.join(cargoRoot, "bin", executable);
const cargo = process.env.CARGO || "cargo";

if (process.env.LOCALSEARCH_SKIP_INSTALL === "1") {
  console.log("localsearch: skipped native installation (LOCALSEARCH_SKIP_INSTALL=1)");
  process.exit(0);
}

const cargoCheck = spawnSync(cargo, ["--version"], {
  encoding: "utf8",
  stdio: ["ignore", "pipe", "pipe"],
});

if (cargoCheck.status !== 0) {
  console.error(
    [
      "localsearch requires Rust and Cargo to install its native CLI.",
      "Install Rust from https://rustup.rs, then run `npm rebuild @kevinliu01/localsearch`.",
      cargoCheck.error ? `Cargo check failed: ${cargoCheck.error.message}` : "Cargo was not found on PATH.",
    ].join("\n"),
  );
  process.exit(1);
}

mkdirSync(cargoRoot, { recursive: true });

console.log(`localsearch: installing native local-search ${nativeVersion}`);
const install = spawnSync(
  cargo,
  [
    "install",
    "local-search",
    "--version",
    `=${nativeVersion}`,
    "--locked",
    "--force",
    "--root",
    cargoRoot,
  ],
  { stdio: "inherit" },
);

if (install.error || install.status !== 0 || !existsSync(nativeBinary)) {
  if (install.error) {
    console.error(`localsearch: Cargo failed: ${install.error.message}`);
  }
  console.error("localsearch: native installation did not produce the lsearch binary.");
  process.exit(install.status || 1);
}

const verify = spawnSync(nativeBinary, ["--version"], {
  encoding: "utf8",
  stdio: ["ignore", "pipe", "inherit"],
});

if (verify.status !== 0) {
  console.error("localsearch: installed binary failed its version check.");
  process.exit(verify.status || 1);
}

console.log(`localsearch: ready (${verify.stdout.trim()})`);
