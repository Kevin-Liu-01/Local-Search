"use client";

import { CargoBrandIcon, CheckIcon, CopyIcon, NpmBrandIcon } from "@/components/icons";
import { useState } from "react";

export function CopyCommand({
  value,
  compact = false,
  showBrand = true,
}: {
  value: string;
  compact?: boolean;
  showBrand?: boolean;
}) {
  const [copied, setCopied] = useState(false);
  const isCargoCommand = value.startsWith("cargo ");
  const isNpmCommand = value.startsWith("npm ");

  async function copy() {
    await navigator.clipboard.writeText(value);
    setCopied(true);
  }

  return (
    <button
      type="button"
      className={compact ? "copy-command copy-command--compact" : "copy-command"}
      onClick={copy}
      aria-label={`Copy command: ${value}`}
    >
      <code>
        {showBrand ? (
          isCargoCommand ? (
            <CargoBrandIcon size={18} />
          ) : isNpmCommand ? (
            <NpmBrandIcon size={18} />
          ) : (
            <span className="copy-command__prompt">$</span>
          )
        ) : null}
        {value.split(/(\s+)/).map((word, index) => /^\s+$/.test(word) ? word : (
          <span className="copy-command__word" key={index}>{word}</span>
        ))}
      </code>
      <span className="copy-command__action">
        {copied ? <CheckIcon size={18} /> : <CopyIcon size={18} />}
        {copied ? "Copied" : "Copy"}
      </span>
      <span className="sr-only" role="status">{copied ? "Command copied to clipboard" : ""}</span>
    </button>
  );
}
