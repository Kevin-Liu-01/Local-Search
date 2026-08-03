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
      aria-label="Copy command"
    >
      <code>
        {showBrand ? (
          isCargoCommand ? (
            <CargoBrandIcon size={18} />
          ) : isNpmCommand ? (
            <NpmBrandIcon size={18} />
          ) : (
            <span>$</span>
          )
        ) : null}
        {value}
      </code>
      <span className="copy-command__action">
        {copied ? <CheckIcon size={15} /> : <CopyIcon size={15} />}
        {copied ? "Copied" : "Copy"}
      </span>
    </button>
  );
}
