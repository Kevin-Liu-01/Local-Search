"use client";

import dynamic from "next/dynamic";
import { useEffect, useRef, useState, type ReactNode } from "react";

const AgentPlaygroundInteractive = dynamic(
  () =>
    import("./agent-playground-interactive").then(
      (module) => module.AgentPlaygroundInteractive,
    ),
  { ssr: false },
);

export function AgentPlaygroundLoader({ fallback }: { fallback: ReactNode }) {
  const root = useRef<HTMLDivElement>(null);
  const [active, setActive] = useState(false);

  useEffect(() => {
    const element = root.current;
    if (!element || active) return;

    if (typeof IntersectionObserver === "undefined") {
      const timer = globalThis.setTimeout(() => setActive(true), 0);
      return () => globalThis.clearTimeout(timer);
    }

    const observer = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          setActive(true);
          observer.disconnect();
        }
      },
      { threshold: 0.01 },
    );

    observer.observe(element);
    return () => observer.disconnect();
  }, [active]);

  return (
    <div ref={root} aria-busy={!active}>
      {active ? <AgentPlaygroundInteractive /> : fallback}
    </div>
  );
}
