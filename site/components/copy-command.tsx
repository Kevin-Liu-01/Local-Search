"use client";

import { CargoBrandIcon, CheckIcon, CopyIcon } from "@/components/icons";
import { useState } from "react";

export function CopyCommand({ value, compact = false }: { value: string; compact?: boolean }) {
  const [copied, setCopied] = useState(false);
  const isCargoCommand = value.startsWith("cargo ");

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
        {isCargoCommand ? <CargoBrandIcon size={18} /> : <span>$</span>}
        {value}
      </code>
      <span className="copy-command__action">
        {copied ? <CheckIcon size={15} /> : <CopyIcon size={15} />}
        {copied ? "Copied" : "Copy"}
      </span>
    </button>
  );
}
