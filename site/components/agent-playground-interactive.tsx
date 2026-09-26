"use client";

import "./brainless.css";
import { useEffect, useRef } from "react";
import { ClaudeMessage } from "@/components/brainless/claude/claude-message";
import { ClaudeToolCall } from "@/components/brainless/claude/claude-tool-call";
import { CodexMessage } from "@/components/brainless/codex/codex-message";
import { CodexExec } from "@/components/brainless/codex/codex-exec";
import { CheckIcon, ChevronDownIcon, TerminalIcon } from "@/components/icons";
import { traceJson, type SearchTrace } from "@/lib/traces";

export type DemoAgent = "claude" | "codex" | "cursor";

// Presentation only: browser-workflow owns the shared agent/browser timeline.
export function AgentSearchPanel({ agent, trace, command, stage, instant }: {
  agent: DemoAgent;
  trace: SearchTrace;
  command: string;
  stage: number;
  instant: boolean;
}) {
  const viewport = useRef<HTMLDivElement>(null);
  const complete = stage >= 4;
  const prompt = `Search for “${trace.query}” and return three results.`;
  const Message = agent === "claude" ? ClaudeMessage : CodexMessage;
  const progress = ["Reading your request…", "Running lsearch…", "Searching in local Chrome…", "Reading the search results…"][stage];
  const toolStatus = <span className="paired-tool-status"><span aria-hidden={complete}>Running in local Chrome…</span><span aria-hidden={!complete}>3 results · View JSON</span></span>;

  useEffect(() => {
    const frame = window.requestAnimationFrame(() => {
      if (!viewport.current) return;
      // Reserved result space must not pull the viewport into an empty area.
      viewport.current.scrollTop = stage >= 4 && !instant ? viewport.current.scrollHeight : 0;
    });
    return () => window.cancelAnimationFrame(frame);
  }, [agent, stage, instant]);

  return (
    <div
      ref={viewport}
      className={`paired-agent-content paired-agent-content--${agent}`}
      data-instant={instant}
    >
      {agent === "cursor" ? (
        <>
          <div className="paired-cursor-heading"><b>Local browser search</b><span>Agent · Auto</span></div>
          <div className="paired-cursor-prompt">{prompt}</div>
          <p className="paired-agent-reply">I’ll search with your local browser.</p>
        </>
      ) : (
        <>
          <Message role="user" className="paired-agent-prompt">{prompt}</Message>
          <Message className="paired-agent-reply">I’ll search with your local browser.</Message>
        </>
      )}

      <div className="paired-agent-tool" style={{ visibility: stage >= 1 ? "visible" : "hidden" }} inert={stage < 1}>
        {agent === "claude" ? (
          <ClaudeToolCall tool="Bash" arg={command} result={toolStatus} status={complete ? "success" : "pending"}>
            {complete ? traceJson(trace) : undefined}
          </ClaudeToolCall>
        ) : agent === "codex" ? (
          <CodexExec command={command} result={toolStatus} status={complete ? "ok" : "run"}>
            {complete ? traceJson(trace) : undefined}
          </CodexExec>
        ) : (
          <div className="paired-cursor-tool">
            <div><TerminalIcon size={18} /><b>Terminal</b><CheckIcon size={18} style={{ visibility: complete ? "visible" : "hidden" }} /></div>
            <pre>{command}</pre>
            <details inert={!complete}><summary>{toolStatus}<ChevronDownIcon size={18} style={{ visibility: complete ? "visible" : "hidden" }} /></summary><pre>{traceJson(trace)}</pre></details>
          </div>
        )}
      </div>
      <div className="paired-agent-outcome">
        <div className="paired-agent-answer" style={{ visibility: complete ? "visible" : "hidden" }} inert={!complete}>
          <p><CheckIcon size={18} />Found three results.</p>
          <ol>
            {trace.results.map((result) => (
              <li key={result.url}><a href={result.url} target="_blank" rel="noreferrer"><b>{result.title}</b><span>{result.domain}</span></a></li>
            ))}
          </ol>
        </div>
        {!complete && <p className="paired-agent-progress"><span aria-hidden="true" />{progress}</p>}
      </div>
    </div>
  );
}
