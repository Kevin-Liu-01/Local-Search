"use client";

import { useEffect, useRef, useState } from "react";
import { CheckIcon, CopyIcon } from "@/components/icons";

export function DocsCode({ children, label = "Command" }: { children: string; label?: string }) {
  const [status, setStatus] = useState<"idle" | "copied" | "failed">("idle");
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  useEffect(() => () => clearTimeout(timer.current), []);

  async function copy() {
    clearTimeout(timer.current);
    try {
      await navigator.clipboard.writeText(children);
      setStatus("copied");
      timer.current = setTimeout(() => setStatus("idle"), 3000);
    } catch {
      setStatus("failed");
    }
  }

  return (
    <div className="docs-code">
      <div className="docs-code__bar">
        <span>{label}</span>
        <button type="button" onClick={copy} aria-label={`Copy ${label.toLowerCase()}`}>
          {status === "copied" ? <CheckIcon size={17} /> : <CopyIcon size={17} />}
          {status === "copied" ? "Copied" : "Copy"}
        </button>
      </div>
      <pre><code>{children}</code></pre>
      <span className={status === "failed" ? "docs-code__error" : "sr-only"} role="status">
        {status === "failed" ? "Could not copy. Select and copy the text above." : status === "copied" ? "Copied to clipboard." : ""}
      </span>
    </div>
  );
}
