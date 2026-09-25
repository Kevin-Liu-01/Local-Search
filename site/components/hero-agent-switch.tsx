"use client";

import { useState, type ReactNode } from "react";

const agents = [
  { name: "Claude Code", asset: "claude", monochrome: false },
  { name: "Codex", asset: "codex", monochrome: true },
  { name: "Cursor", asset: "cursor-mono", monochrome: true },
  { name: "OpenClaw", asset: "openclaw", monochrome: false },
];

export function HeroAgentSwitch({ children }: { children: ReactNode }) {
  const [selected, setSelected] = useState(0);
  const [instant, setInstant] = useState(true);
  const current = agents[selected];
  const next = agents[(selected + 1) % agents.length];

  return (
    <g className="hero-iso-agent" data-agent={current.name} data-instant={instant}>
      <g aria-hidden="true">
        {children}
        <g transform="matrix(.866 .5 -.866 .5 190 319)">
          {agents.map((agent, index) => (
            <image
              key={agent.name}
              className={`hero-iso-agent__mark${agent.monochrome ? " hero-iso-agent__mark--light" : ""}`}
              href={`/brand/${agent.asset}.svg`}
              width="64"
              height="64"
              style={{ opacity: index === selected ? 1 : 0 }}
            />
          ))}
        </g>
        <polygon className="hero-iso-agent__focus" points="84,357 211,284 306,339 306,411 179,484 84,429" />
      </g>
      <foreignObject x="84" y="284" width="222" height="200">
        <button
          className="hero-agent-switch"
          type="button"
          aria-label={`Agent: ${current.name}. Switch to ${next.name}`}
          title={`${current.name} · Click to switch agent`}
          onClick={(event) => {
            setInstant(event.detail === 0);
            setSelected((index) => (index + 1) % agents.length);
          }}
        />
      </foreignObject>
    </g>
  );
}
