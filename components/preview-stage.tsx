"use client";

import {
  Archive,
  Check,
  Code2,
  Copy,
  CreditCard,
  Home,
  Layers3,
  Plus,
  Share2,
  Sparkles,
} from "lucide-react";
import { useEffect, useState, type CSSProperties } from "react";
import type { SwiftComponent } from "@/lib/components";
import { cn } from "@/lib/utils";

type PreviewStageProps = Pick<SwiftComponent, "variant" | "accent" | "title" | "video" | "poster"> & {
  interactive?: boolean;
  compact?: boolean;
};

export function PreviewStage({
  variant,
  accent,
  title,
  video,
  poster,
  interactive = false,
  compact = false,
}: PreviewStageProps) {
  const [active, setActive] = useState(false);
  const [dock, setDock] = useState(0);
  const [toastCount, setToastCount] = useState(3);
  const [sheetOpen, setSheetOpen] = useState(true);

  useEffect(() => {
    if (interactive) return;
    const interval = window.setInterval(() => {
      setActive((current) => !current);
      setDock((current) => (current + 1) % 3);
      setSheetOpen((current) => !current);
    }, 2300);
    return () => window.clearInterval(interval);
  }, [interactive]);

  const style = { "--accent": accent } as CSSProperties;

  if (video) {
    return (
      <div className={cn("preview-stage video-preview", compact && "preview-stage-compact")} style={style}>
        <video autoPlay muted loop playsInline preload="metadata" poster={poster} aria-label={`${title} preview`}>
          <source src={video} type="video/mp4" />
        </video>
      </div>
    );
  }

  return (
    <div
      className={cn("preview-stage", `preview-${variant}`, compact && "preview-stage-compact")}
      style={style}
      aria-label={`${title} interactive preview`}
    >
      <div className="stage-orb stage-orb-one" />
      <div className="stage-orb stage-orb-two" />

      {variant === "swap" && (
        <button
          type="button"
          className={cn("demo-swap", active && "is-active")}
          onClick={() => setActive((current) => !current)}
          tabIndex={interactive ? 0 : -1}
        >
          <span className="swap-icon">{active ? <Check /> : <Copy />}</span>
          <span className="swap-label">{active ? "Copied" : "Copy code"}</span>
        </button>
      )}

      {variant === "toggle" && (
        <div className="demo-toggle" role="group" aria-label="Preview mode">
          {["Design", "Code"].map((label, index) => {
            const selected = active ? index === 1 : index === 0;
            return (
              <button
                type="button"
                key={label}
                onClick={() => setActive(index === 1)}
                className={selected ? "selected" : ""}
                tabIndex={interactive ? 0 : -1}
              >
                {index === 0 ? <Sparkles /> : <Code2 />}
                {label}
              </button>
            );
          })}
        </div>
      )}

      {variant === "card" && (
        <div className="demo-spatial-card">
          <div className="spatial-card-top">
            <span className="spatial-chip"><Sparkles /></span>
            <span className="spatial-badge">PRO</span>
          </div>
          <div>
            <strong>Studio</strong>
            <span>Build without limits</span>
          </div>
          <div className="spatial-price">$12 <small>/ month</small></div>
        </div>
      )}

      {variant === "dock" && (
        <div className="demo-phone-surface">
          <div className="phone-widget">
            <span>Today</span>
            <strong>18,240</strong>
            <small>Steps</small>
          </div>
          <div className="demo-dock">
            {[Home, Layers3, CreditCard].map((Icon, index) => (
              <button
                type="button"
                key={index}
                className={dock === index ? "selected" : ""}
                onClick={() => setDock(index)}
                tabIndex={interactive ? 0 : -1}
                aria-label={["Home", "Library", "Wallet"][index]}
              >
                <Icon />
                {dock === index && <span>{["Home", "Library", "Wallet"][index]}</span>}
              </button>
            ))}
          </div>
        </div>
      )}

      {variant === "toast" && (
        <div className="toast-demo-wrap">
          <div className="toast-stack">
            {[0, 1, 2].slice(0, Math.min(3, toastCount)).map((index) => (
              <div className="demo-toast" key={`${toastCount}-${index}`} style={{ "--index": index } as CSSProperties}>
                <span>{index === 0 ? <Check /> : index === 1 ? <Share2 /> : <Archive />}</span>
                <div>
                  <strong>{["Saved to Library", "Link copied", "Ready to archive"][index]}</strong>
                  <small>{index === 0 ? "Just now" : `${index + 1}m ago`}</small>
                </div>
              </div>
            ))}
          </div>
          {interactive && (
            <button type="button" className="toast-add" onClick={() => setToastCount((count) => (count % 3) + 1)}>
              <Plus /> New toast
            </button>
          )}
        </div>
      )}

      {variant === "sheet" && (
        <div className="sheet-phone">
          <div className="sheet-trigger-row">
            <div><span /><span /><span /></div>
            <button type="button" onClick={() => setSheetOpen(true)} tabIndex={interactive ? 0 : -1}>
              <Plus />
            </button>
          </div>
          <div className="sheet-content-lines"><span /><span /><span /></div>
          <div className={cn("demo-sheet", sheetOpen && "is-open")}>
            <button
              type="button"
              aria-label="Close sheet"
              className="sheet-handle"
              onClick={() => setSheetOpen(false)}
              tabIndex={interactive ? 0 : -1}
            />
            <strong>Quick actions</strong>
            <div className="sheet-actions">
              <span><Copy /> Duplicate</span>
              <span><Share2 /> Share</span>
              <span><Archive /> Archive</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
