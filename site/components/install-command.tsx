"use client";

import { CargoBrandIcon, NpmBrandIcon } from "@/components/icons";
import { CopyCommand } from "@/components/copy-command";
import { useState } from "react";

type Installer = "cargo" | "npm";

const commands: Record<Installer, string> = {
  cargo: "cargo install local-search",
  npm: "npm install -g @kevinliu01/localsearch",
};

export function InstallCommand({ initial = "cargo" }: { initial?: Installer }) {
  const [installer, setInstaller] = useState<Installer>(initial);

  return (
    <div className={`install-command install-command--${installer}`}>
      <div className="install-command__switch" role="group" aria-label="Choose a package manager">
        <span className="install-command__indicator" aria-hidden="true" />
        <button
          type="button"
          className={installer === "cargo" ? "is-active" : ""}
          aria-pressed={installer === "cargo"}
          onClick={() => setInstaller("cargo")}
        >
          <CargoBrandIcon size={20} /> Cargo
        </button>
        <button
          type="button"
          className={installer === "npm" ? "is-active" : ""}
          aria-pressed={installer === "npm"}
          onClick={() => setInstaller("npm")}
        >
          <NpmBrandIcon size={20} /> npm
        </button>
      </div>
      <CopyCommand key={installer} value={commands[installer]} showBrand={false} />
    </div>
  );
}
