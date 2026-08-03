#!/usr/bin/env node

"use strict";

const { existsSync, readFileSync } = require("node:fs");
const path = require("node:path");

const packageJson = require("../package.json");
const nativeVersion = packageJson.config?.nativeVersion;
const packageRoot = path.resolve(__dirname, "..");
const repositoryRoot = path.resolve(packageRoot, "..", "..");
const cargoManifest = path.join(repositoryRoot, "Cargo.toml");
const requiredFiles = [
  "bin/lsearch.cjs",
  "scripts/install.cjs",
  "README.md",
  "LICENSE",
];

for (const file of requiredFiles) {
  if (!existsSync(path.join(packageRoot, file))) {
    throw new Error(`npm package is missing ${file}`);
  }
}

if (existsSync(cargoManifest)) {
  const manifest = readFileSync(cargoManifest, "utf8");
  const version = manifest.match(/^version\s*=\s*"([^"]+)"/m)?.[1];
  if (version !== nativeVersion) {
    throw new Error(
      `npm bridge targets Cargo ${nativeVersion || "unknown"}, but the repository is ${version || "unknown"}`,
    );
  }
}

console.log(`localsearch npm ${packageJson.version} -> Rust ${nativeVersion}: package metadata verified`);
