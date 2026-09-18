"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState, type KeyboardEvent } from "react";
import { PreviewStage } from "@/components/preview-stage";
import { CodeBlock } from "@/components/code-block";
import type { SwiftComponent } from "@/lib/components";

const tabs = ["Preview", "Code"] as const;

export function ComponentWorkbench({ component }: { component: SwiftComponent }) {
  const [tab, setTab] = useState<(typeof tabs)[number]>("Preview");
  const [previewKey, setPreviewKey] = useState(0);
  const [dark, setDark] = useState(false);
  const filename = `${component.title.replaceAll(" ", "")}.swift`;

  function handleKeys(event: KeyboardEvent<HTMLButtonElement>, index: number) {
    let next = index;
    if (event.key === "ArrowRight") next = (index + 1) % tabs.length;
    else if (event.key === "ArrowLeft") next = (index + tabs.length - 1) % tabs.length;
    else if (event.key === "Home") next = 0;
    else if (event.key === "End") next = tabs.length - 1;
    else return;
    event.preventDefault();
    setTab(tabs[next]);
    document.getElementById(`workbench-tab-${tabs[next]}`)?.focus();
  }

  return (
    <section className="workbench" id="preview" aria-label={`${component.title} examples`}>
      <div className="example-heading"><h2>Overview</h2><code>{filename}</code></div>
      <div className="workbench-toolbar">
        <div className="workbench-tabs" role="tablist" aria-label="Component content">
          {tabs.map((name, index) => <button key={name} type="button" id={`workbench-tab-${name}`} role="tab" aria-selected={tab === name} aria-controls="workbench-panel" tabIndex={tab === name ? 0 : -1} className={tab === name ? "selected" : ""} onClick={() => setTab(name)} onKeyDown={(event) => handleKeys(event, index)}>{name}</button>)}
        </div>
        <div className="workbench-actions">
          {tab === "Preview" && <>{!component.video && <button type="button" className="tool-action" onClick={() => setDark((value) => !value)} aria-label={dark ? "Light preview" : "Dark preview"} title="Toggle preview background">{dark ? <Sun /> : <Moon />}</button>}<button type="button" className="tool-action" onClick={() => setPreviewKey((value) => value + 1)} aria-label="Replay preview" title="Replay preview"><RotateCcw /></button></>}
          <a className="tool-action" href={`/components/${component.slug}/source`} download={filename} aria-label="Download Swift source" title="Download Swift source"><Download /></a>
        </div>
      </div>
      <div className="workbench-body" id="workbench-panel" role="tabpanel" aria-labelledby={`workbench-tab-${tab}`} tabIndex={0}>
        {tab === "Preview" ? <div className={`workbench-preview${component.video ? " is-video" : ""}`} data-preview-theme={dark ? "dark" : "light"} key={previewKey}><PreviewStage {...component} interactive /><div className="preview-caption"><span>{component.video ? (component.videoKind === "remotion" ? "Remotion 演示 · 非原生录屏" : component.videoKind === "swiftui" ? "SwiftUI recording" : "Video preview") : "Interactive web preview"}</span>{component.video ? <a href={component.video} download={`${component.slug}.mp4`}>Download MP4 <Download size={11}/></a> : <span>Try it out <span aria-hidden="true">↗</span></span>}</div></div> : <CodeBlock code={component.source} label={filename} />}
      </div>
    </section>
  );
}
