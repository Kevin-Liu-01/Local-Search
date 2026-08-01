import { AgentPlaygroundLoader } from "@/components/agent-playground-loader";
import { PlaygroundShader } from "@/components/playground-shader";
import {
  ClaudeBrandIcon,
  CodexBrandIcon,
  CursorBrandIcon,
} from "@/components/icons";
import { traces } from "@/lib/traces";

const agents = [
  { label: "Claude Code", icon: ClaudeBrandIcon },
  { label: "Codex", icon: CodexBrandIcon },
  { label: "Cursor", icon: CursorBrandIcon },
];

export function AgentPlayground() {
  return (
    <section className="playground" id="demo" aria-label="Interactive local-search traces">
      <PlaygroundShader />

      <div className="playground__workspace">
        <AgentPlaygroundLoader fallback={<AgentPlaygroundPreview />} />
      </div>

      <p className="trace-provenance">
        Captured July 21, 2026 with the managed local Chrome profile. Results can change with the search engine.
      </p>
    </section>
  );
}

function AgentPlaygroundPreview() {
  const trace = traces[0];
  const prompt = `Search for “${trace.query}” and return three results.`;

  return (
    <div className="playground__frame" aria-label="Claude Code local-search preview">
      <div className="agent-window">
        <div className="agent-window__bar">
          <span className="window-dots" aria-hidden><i /><i /><i /></span>
          <div className="agent-tabs agent-tabs--terminal" aria-label="Coding agent interface">
            {agents.map((agent, index) => {
              const AgentIcon = agent.icon;
              return (
                <span className={index === 0 ? "agent-tab is-active" : "agent-tab"} key={agent.label}>
                  <AgentIcon size={15} />{agent.label}
                </span>
              );
            })}
          </div>
          <span>~/repos/Local-Search</span>
        </div>
        <div className="agent-surface demo-preview">
          <div className="demo-preview__header">
            <span className="demo-preview__logo"><ClaudeBrandIcon size={25} /></span>
            <div><b>Claude Code</b><small>Opus 4.8 · Claude Max</small></div>
          </div>
          <div className="demo-preview__rule" />
          <p className="demo-preview__meta">Kevin · local-search playground · ~/repos/Local-Search</p>
          <div className="demo-preview__spacer" />
          <div className="demo-preview__prompt"><span>❯</span>{prompt}</div>
          <p className="demo-preview__mode">⏵⏵ auto mode on · ← for agents</p>
        </div>
      </div>
    </div>
  );
}
