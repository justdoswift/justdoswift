"use client";

import { Braces, Eye, Play, RotateCcw } from "lucide-react";
import { useState } from "react";
import { CopyCodeButton } from "@/components/copy-code-button";
import { PreviewStage } from "@/components/preview-stage";
import type { SwiftComponent } from "@/lib/components";

type Tab = "preview" | "usage" | "source";

export function ComponentWorkbench({ component }: { component: SwiftComponent }) {
  const [tab, setTab] = useState<Tab>("preview");
  const [previewKey, setPreviewKey] = useState(0);

  return (
    <section className="workbench">
      <div className="workbench-toolbar">
        <div className="workbench-tabs" role="tablist" aria-label="Component content">
          <button type="button" role="tab" aria-selected={tab === "preview"} className={tab === "preview" ? "selected" : ""} onClick={() => setTab("preview")}>
            <Eye /> Preview
          </button>
          <button type="button" role="tab" aria-selected={tab === "usage"} className={tab === "usage" ? "selected" : ""} onClick={() => setTab("usage")}>
            <Play /> Usage
          </button>
          <button type="button" role="tab" aria-selected={tab === "source"} className={tab === "source" ? "selected" : ""} onClick={() => setTab("source")}>
            <Braces /> Source
          </button>
        </div>
        {tab === "preview" ? (
          <button type="button" className="tool-action" onClick={() => setPreviewKey((value) => value + 1)}>
            <RotateCcw /> Replay
          </button>
        ) : (
          <CopyCodeButton code={tab === "usage" ? component.usage : component.source} compact />
        )}
      </div>

      <div className="workbench-body">
        {tab === "preview" && (
          <div className="workbench-preview" key={previewKey}>
            <PreviewStage {...component} interactive />
            <div className="preview-hint"><span /> Interact with the preview</div>
          </div>
        )}
        {tab === "usage" && (
          <CodeBlock code={component.usage} label="Example.swift" />
        )}
        {tab === "source" && (
          <CodeBlock code={component.source} label={`${component.title.replaceAll(" ", "")}.swift`} />
        )}
      </div>
    </section>
  );
}

function CodeBlock({ code, label }: { code: string; label: string }) {
  return (
    <div className="code-panel">
      <div className="code-filebar">
        <span className="code-dot red" />
        <span className="code-dot yellow" />
        <span className="code-dot green" />
        <span className="code-filename">{label}</span>
      </div>
      <pre>
        <code>
          {code.split("\n").map((line, index) => (
            <span className="code-line" key={index}>
              <span className="line-number">{index + 1}</span>
              <span>{line || " "}</span>
            </span>
          ))}
        </code>
      </pre>
    </div>
  );
}
