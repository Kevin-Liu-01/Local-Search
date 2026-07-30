"use client";

import dynamic from "next/dynamic";
import { useEffect, useRef, useState } from "react";

const GrainGradient = dynamic(
  () =>
    import("@paper-design/shaders-react").then(
      (module) => module.GrainGradient,
    ),
  { ssr: false },
);

export function PlaygroundShader() {
  const root = useRef<HTMLDivElement>(null);
  const [hasEntered, setHasEntered] = useState(false);
  const [isVisible, setIsVisible] = useState(false);
  const [reduceMotion, setReduceMotion] = useState(true);

  useEffect(() => {
    const motion = globalThis.matchMedia?.("(prefers-reduced-motion: reduce)");
    const syncMotion = () => setReduceMotion(motion?.matches ?? true);

    syncMotion();
    motion?.addEventListener("change", syncMotion);

    return () => motion?.removeEventListener("change", syncMotion);
  }, []);

  useEffect(() => {
    const element = root.current;
    if (!element) return;

    if (typeof IntersectionObserver === "undefined") {
      const timer = globalThis.setTimeout(() => {
        setHasEntered(true);
        setIsVisible(true);
      }, 0);
      return () => globalThis.clearTimeout(timer);
    }

    const observer = new IntersectionObserver(
      ([entry]) => {
        setIsVisible(entry.isIntersecting);
        if (entry.isIntersecting) setHasEntered(true);
      },
      { rootMargin: "240px 0px", threshold: 0.01 },
    );

    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  return (
    <div className="playground-shader" ref={root} aria-hidden="true">
      <span className="playground-shader__image playground-shader__image--sky" />
      <span className="playground-shader__image playground-shader__image--indigo" />
      <span className="playground-shader__image playground-shader__image--aurora" />

      {hasEntered ? (
        <GrainGradient
          className="playground-shader__canvas"
          width="100%"
          height="100%"
          colors={["#f3ffff", "#c9f1ef", "#7ec9c9", "#168b90", "#0d6f74"]}
          colorBack="#c9f1ef"
          softness={0.82}
          intensity={0.2}
          noise={0.38}
          shape="wave"
          speed={isVisible && !reduceMotion ? 0.08 : 0}
          scale={1.08}
          rotation={8}
          offsetX={0.03}
          offsetY={0.08}
          fit="cover"
          minPixelRatio={1}
          maxPixelCount={720_000}
        />
      ) : null}

      <span className="playground-shader__veil" />
    </div>
  );
}
