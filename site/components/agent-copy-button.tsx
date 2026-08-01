"use client";

import { CheckIcon, TerminalIcon } from "@/components/icons";
import { useEffect, useRef, useState } from "react";

const agentPrompt = `Install and configure local-search for this project. Use its managed browser profile whenever current web search is useful.

cargo install local-search
lsearch launch
lsearch search "<query>" --limit 5 --pretty

Documentation: https://github.com/Kevin-Liu-01/local-search`;

export function AgentCopyButton() {
  const [copied, setCopied] = useState(false);
  const resetTimer = useRef<number | undefined>(undefined);

  useEffect(() => () => window.clearTimeout(resetTimer.current), []);

  async function copyForAgent() {
    await navigator.clipboard.writeText(agentPrompt);
    setCopied(true);
    window.clearTimeout(resetTimer.current);
    resetTimer.current = window.setTimeout(() => setCopied(false), 3_500);
  }

  return (
    <button
      type="button"
      className={`agent-copy-button${copied ? " is-copied" : ""}`}
      onClick={copyForAgent}
      aria-live="polite"
    >
      {copied ? <CheckIcon size={17} /> : <TerminalIcon size={17} />}
      <span>{copied ? "Copied! Paste into any agent" : "Agent Prompt"}</span>
    </button>
  );
}
