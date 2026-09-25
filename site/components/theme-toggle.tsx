"use client";

import { useId, useState, useSyncExternalStore } from "react";
import { THEME_STORAGE_KEY, type Theme } from "@/lib/theme";

// Keeps a manual choice for this visit even if browser storage is unavailable.
let visitChoice: Theme | null = null;

function savedTheme(): Theme | null {
  try {
    const value = localStorage.getItem(THEME_STORAGE_KEY);
    return value === "light" || value === "dark" ? value : null;
  } catch {
    return null;
  }
}

function subscribe(callback: () => void) {
  const root = document.documentElement;
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const observer = new MutationObserver(callback);
  observer.observe(root, { attributes: true, attributeFilter: ["data-theme"] });

  function syncPreference() {
    root.dataset.theme = visitChoice ?? savedTheme() ?? (media.matches ? "dark" : "light");
  }
  function syncStorage(event: StorageEvent) {
    if (event.key !== THEME_STORAGE_KEY && event.key !== null) return;
    visitChoice = null;
    syncPreference();
  }
  media.addEventListener("change", syncPreference);
  window.addEventListener("storage", syncStorage);
  syncPreference();
  return () => {
    observer.disconnect();
    media.removeEventListener("change", syncPreference);
    window.removeEventListener("storage", syncStorage);
  };
}

const getTheme = (): Theme => document.documentElement.dataset.theme === "dark" ? "dark" : "light";
const getServerTheme = (): Theme => "light";

export function ThemeToggle() {
  const theme = useSyncExternalStore(subscribe, getTheme, getServerTheme);
  const maskId = `theme-eclipse-${useId()}`;
  const [animate, setAnimate] = useState(false);
  const label = theme === "dark" ? "Switch to light mode" : "Switch to dark mode";

  function toggle() {
    const next = getTheme() === "dark" ? "light" : "dark";
    visitChoice = next;
    document.documentElement.dataset.theme = next;
    try { localStorage.setItem(THEME_STORAGE_KEY, next); } catch { /* The toggle still works without persistence. */ }
  }

  return (
    <button
      type="button"
      className="theme-toggle"
      data-animate={animate}
      onClick={(event) => {
        // Keyboard changes and the initial saved theme settle immediately.
        setAnimate(event.detail !== 0);
        toggle();
      }}
      aria-label={label}
      title={label}
    >
      <svg className="theme-dial" width="40" height="40" viewBox="0 0 48 48" fill="none" aria-hidden="true">
        <defs>
          <mask id={maskId} maskUnits="userSpaceOnUse" x="0" y="0" width="48" height="48">
            <rect width="48" height="48" fill="white" />
            <circle className="theme-dial__eclipse" cx="42" cy="10" r="9" fill="black" />
          </mask>
        </defs>
        <g className="theme-dial__rays">
          {Array.from({ length: 8 }, (_, index) => (
            <g key={index} transform={`rotate(${index * 45} 24 24)`}>
              <path className="theme-dial__ray" d="M24 9.5v3" />
            </g>
          ))}
        </g>
        <g className="theme-dial__body">
          <circle className="theme-dial__disc" cx="24" cy="24" r="8" mask={`url(#${maskId})`} />
        </g>
        <g className="theme-dial__stars">
          <path className="theme-dial__star theme-dial__star--one" d="m34 10 .9 2.1L37 13l-2.1.9L34 16l-.9-2.1L31 13l2.1-.9Z" />
          <path className="theme-dial__star theme-dial__star--two" d="m37 22 .6 1.4 1.4.6-1.4.6L37 26l-.6-1.4-1.4-.6 1.4-.6Z" />
          <circle className="theme-dial__star theme-dial__star--three" cx="26" cy="9" r="1" />
        </g>
      </svg>
    </button>
  );
}
