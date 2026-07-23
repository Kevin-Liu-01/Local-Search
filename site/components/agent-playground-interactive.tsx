"use client";

import "./brainless.css";

import { ClaudeHeader } from "@/components/brainless/claude/claude-header";
import { ClaudeMessage } from "@/components/brainless/claude/claude-message";
import { ClaudePrompt } from "@/components/brainless/claude/claude-prompt";
import { ClaudeThinking } from "@/components/brainless/claude/claude-thinking";
import { ClaudeToolCall } from "@/components/brainless/claude/claude-tool-call";
import { CodexExec } from "@/components/brainless/codex/codex-exec";
import { CodexHeader } from "@/components/brainless/codex/codex-header";
import { CodexMessage } from "@/components/brainless/codex/codex-message";
import { CodexPrompt } from "@/components/brainless/codex/codex-prompt";
import { CodexWorking } from "@/components/brainless/codex/codex-working";
import {
  CheckIcon,
  ChevronDownIcon,
  ClaudeBrandIcon,
  CodexBrandIcon,
  CursorBrandIcon,
  PlayIcon,
  RotateIcon,
  SearchIcon,
  StopIcon,
} from "@/components/icons";
import { traces, traceJson, type SearchTrace } from "@/lib/traces";
import { useEffect, useMemo, useState } from "react";

type Agent = "claude" | "codex" | "cursor";

const agents: { id: Agent; label: string }[] = [
  { id: "claude", label: "Claude Code" },
  { id: "codex", label: "Codex" },
  { id: "cursor", label: "Cursor" },
];

const agentIcons = {
  claude: ClaudeBrandIcon,
  codex: CodexBrandIcon,
  cursor: CursorBrandIcon,
};

export function AgentPlaygroundInteractive() {
  const [agent, setAgent] = useState<Agent>("claude");
  const [traceId, setTraceId] = useState(traces[0].id);
  const [step, setStep] = useState(0);
  const trace = useMemo(() => traces.find((item) => item.id === traceId) ?? traces[0], [traceId]);
  const running = step > 0 && step < 4;
  const complete = step === 4;

  useEffect(() => {
    if (!running) return;
    const timer = window.setTimeout(() => setStep((current) => Math.min(current + 1, 4)), 650);
    return () => window.clearTimeout(timer);
  }, [running, step]);

  function runTrace() {
    setStep(1);
  }

  function selectTrace(id: string) {
    setTraceId(id);
    setStep(0);
  }

  return (
    <div className="playground__frame">
        <div className="playground__toolbar">
          <div className="agent-tabs" role="tablist" aria-label="Coding agent interface">
            {agents.map((item) => {
              const AgentIcon = agentIcons[item.id];
              return (
                <button
                  key={item.id}
                  type="button"
                  role="tab"
                  aria-selected={agent === item.id}
                  className={agent === item.id ? "agent-tab is-active" : "agent-tab"}
                  onClick={() => setAgent(item.id)}
                >
                  <AgentIcon size={15} />{item.label}
                </button>
              );
            })}
          </div>
          <a href="https://github.com/theswerd/brainless" target="_blank" rel="noreferrer">
            Agent shells by brainless ↗
          </a>
        </div>

        <div className="trace-presets" aria-label="Captured search traces">
          {traces.map((item) => (
            <button
              key={item.id}
              type="button"
              className={trace.id === item.id ? "trace-preset is-active" : "trace-preset"}
              onClick={() => selectTrace(item.id)}
            >
              <SearchIcon size={14} />
              <span><b>{item.label}</b><small>{item.query}</small></span>
              <span className="trace-latency">{item.latency}</span>
            </button>
          ))}
        </div>

        <div className="agent-window">
          <div className="agent-window__bar">
            <span className="window-dots" aria-hidden><i /><i /><i /></span>
            <span>{agent === "cursor" ? "local-search — Cursor" : `Terminal — ${agent}`}</span>
            <span>~/repos/Local-Search</span>
          </div>
          <div className="agent-surface" role="tabpanel" aria-live="polite">
            {agent === "claude" && (
              <ClaudeTrace trace={trace} step={step} running={running} onRun={runTrace} />
            )}
            {agent === "codex" && (
              <CodexTrace trace={trace} step={step} running={running} onRun={runTrace} />
            )}
            {agent === "cursor" && (
              <CursorTrace trace={trace} step={step} running={running} onRun={runTrace} />
            )}
          </div>
        </div>

        <div className="playground__footer">
          <div>
            <span className={running ? "status-dot is-running" : complete ? "status-dot is-done" : "status-dot"} />
            {running ? "Running local trace" : complete ? `Completed in ${trace.latency}` : "Ready to run"}
          </div>
          <button type="button" className="run-button" onClick={runTrace} disabled={running}>
            {running ? <StopIcon size={16} /> : complete ? <RotateIcon size={16} /> : <PlayIcon size={16} />}
            {running ? "Running…" : complete ? "Replay trace" : "Run search"}
          </button>
        </div>
    </div>
  );
}

function ClaudeTrace({ trace, step, running, onRun }: TraceProps) {
  return (
    <div className="trace-stack trace-stack--claude">
      <ClaudeHeader
        user="Kevin"
        model="Opus 4.8 · Claude Max"
        org="local-search playground"
        cwd="~/repos/Local-Search"
        tips={["Use lsearch when current web context is required"]}
        whatsNew={["Structured browser search with zero API credits"]}
      />
      <ClaudeMessage role="user">Search for “{trace.query}” and return three results.</ClaudeMessage>
      {step >= 1 && <ClaudeMessage>I’ll search through the managed local browser and keep the output compact.</ClaudeMessage>}
      {step >= 2 && (
        <ClaudeToolCall
          tool="Bash"
          arg={trace.command}
          result={step >= 4 ? `3 results in ${trace.latency}` : "Running in local Chrome…"}
          status={step >= 4 ? "success" : "pending"}
          defaultOpen={step >= 4}
        >
          {traceJson(trace)}
        </ClaudeToolCall>
      )}
      {running && <ClaudeThinking verbs={["Searching", "Parsing", "Normalizing"]} showTokens={false} />}
      {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
      {step >= 4 && <TraceResults trace={trace} />}
      <ClaudePrompt placeholder="Select a capture above or replay this trace" onKeyDown={(event) => event.key === "Enter" && onRun()} />
    </div>
  );
}

function CodexTrace({ trace, step, running, onRun }: TraceProps) {
  return (
    <div className="trace-stack trace-stack--codex">
      <CodexHeader model="gpt-5.6-sol high" directory="~/repos/Local-Search" />
      <CodexMessage role="user">Search for “{trace.query}” and return three results.</CodexMessage>
      {step >= 1 && <CodexMessage>I’ll use the local browser and return the normalized output.</CodexMessage>}
      {step >= 2 && (
        <CodexExec
          command={trace.command}
          result={step >= 4 ? `→ 3 results · ${trace.latency}` : "→ running"}
          status={step >= 4 ? "ok" : "run"}
          defaultOpen={step >= 4}
        >
          {traceJson(trace)}
        </CodexExec>
      )}
      {running && <CodexWorking label={step < 3 ? "Searching" : "Normalizing"} />}
      {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
      {step >= 4 && <TraceResults trace={trace} />}
      <CodexPrompt directory="~/repos/Local-Search" placeholder="Press Enter to replay this trace" onKeyDown={(event) => event.key === "Enter" && onRun()} />
    </div>
  );
}

function CursorTrace({ trace, step, running, onRun }: TraceProps) {
  return (
    <div className="cursor-trace">
      <aside className="cursor-rail">
        <b>LOCAL-SEARCH</b>
        <span className="is-selected">⌁ Search with local browser</span>
        <span>◫ Compare result schemas</span>
        <span>✓ Benchmark token usage</span>
        <small>main · local</small>
      </aside>
      <div className="cursor-thread">
        <div className="cursor-thread__header"><b>Search with local browser</b><span>Agent · Auto</span></div>
        <div className="cursor-thread__body">
          <div className="cursor-user">Search for “{trace.query}” and return three results.</div>
          {step >= 1 && <p>I’ll run the query through your managed browser and inspect the normalized response.</p>}
          {step >= 2 && (
            <details className="cursor-tool" open={step >= 4}>
              <summary><span>›_</span><b>Run local search</b><small>{step >= 4 ? `✓ ${trace.latency}` : "running"}</small><ChevronDownIcon size={13} /></summary>
              <pre>{trace.command}</pre>
              {step >= 4 && <pre className="cursor-json">{traceJson(trace)}</pre>}
            </details>
          )}
          {running && <p className="cursor-working"><span />{step < 3 ? "Searching the web…" : "Normalizing three results…"}</p>}
          {step >= 3 && <TraceSteps trace={trace} complete={completeStep(step)} />}
          {step >= 4 && <TraceResults trace={trace} />}
        </div>
        <div className="cursor-composer">
          <input aria-label="Cursor prompt" placeholder="Press Enter to replay this trace" onKeyDown={(event) => event.key === "Enter" && onRun()} />
          <button type="button" onClick={onRun} aria-label="Run trace"><PlayIcon size={14} /></button>
        </div>
      </div>
    </div>
  );
}

function TraceSteps({ trace, complete }: { trace: SearchTrace; complete: boolean }) {
  return (
    <div className="trace-steps" aria-label="local-search execution trace">
      <span><CheckIcon size={12} /> Attach managed Chrome</span>
      <span><CheckIcon size={12} /> Navigate DuckDuckGo</span>
      <span><CheckIcon size={12} /> Extract {trace.results.length} organic results</span>
      <span className={complete ? "is-complete" : "is-pending"}>{complete ? <CheckIcon size={12} /> : <span className="mini-loader" />} Normalize JSON stdout</span>
    </div>
  );
}

function TraceResults({ trace }: { trace: SearchTrace }) {
  return (
    <div className="trace-results">
      {trace.results.map((result) => (
        <a key={result.url} href={result.url} target="_blank" rel="noreferrer">
          <span>{String(result.rank).padStart(2, "0")}</span>
          <div><b>{result.title}</b><small>{result.domain} · {result.snippet}</small></div>
        </a>
      ))}
    </div>
  );
}

function completeStep(step: number) {
  return step >= 4;
}

type TraceProps = {
  trace: SearchTrace;
  step: number;
  running: boolean;
  onRun: () => void;
};
