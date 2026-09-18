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
import { useEffect, useRef, useState, type CSSProperties } from "react";
import { useInView, useReducedMotion } from "motion/react";
import type { SwiftComponent } from "@/lib/components";
import { cn } from "@/lib/utils";

type PreviewStageProps = Pick<SwiftComponent, "variant" | "accent" | "title" | "video" | "poster"> & {
  interactive?: boolean;
  compact?: boolean;
  paused?: boolean;
};

export function PreviewStage({
  variant,
  accent,
  title,
  video,
  poster,
  interactive = false,
  compact = false,
  paused = false,
}: PreviewStageProps) {
  const [active, setActive] = useState(false);
  const [dock, setDock] = useState(0);
  const [toastCount, setToastCount] = useState(3);
  const [sheetOpen, setSheetOpen] = useState(true);
  const stage = useRef<HTMLDivElement>(null);
  const media = useRef<HTMLVideoElement>(null);
  const [mediaFailed, setMediaFailed] = useState(false);
  const inView = useInView(stage);
  const reducedMotion = useReducedMotion();

  useEffect(() => {
    if (!video) return;
    function updatePlayback() {
      if (!media.current) return;
      if (!inView || document.hidden || reducedMotion !== false || paused) media.current.pause();
      else void media.current.play().catch(() => { /* Playback controls remain available. */ });
    }
    updatePlayback();
    document.addEventListener("visibilitychange", updatePlayback);
    return () => document.removeEventListener("visibilitychange", updatePlayback);
  }, [inView, reducedMotion, video, paused]);

  useEffect(() => {
    if (video || interactive || reducedMotion !== false || paused || !inView || document.hidden) return;
    const interval = window.setInterval(() => {
      setActive((current) => !current);
      setDock((current) => (current + 1) % 3);
      setSheetOpen((current) => !current);
    }, 2300);
    return () => window.clearInterval(interval);
  }, [video, interactive, reducedMotion, inView, paused]);

  const style = { "--accent": accent } as CSSProperties;

  if (video) {
    return (
      <div ref={stage} className={cn("preview-stage video-preview", compact && "preview-stage-compact")} style={style}>
        <video ref={media} controls={interactive} muted loop playsInline preload={interactive ? "metadata" : "none"} poster={poster} aria-label={`${title} preview`} onError={() => setMediaFailed(true)}>
          <source src={video} type="video/mp4" />
        </video>
        {mediaFailed && interactive && <p className="video-error">视频暂时无法播放。可下载 MP4 后查看，源码仍可正常使用。</p>}
      </div>
    );
  }

  return (
    <div
      ref={stage}
      className={cn("preview-stage", `preview-${variant}`, compact && "preview-stage-compact")}
      style={style}
      aria-label={`${title} interactive preview`}
    >

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
                aria-pressed={selected}
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
                aria-pressed={dock === index}
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
            <button type="button" onClick={() => setSheetOpen(true)} tabIndex={interactive ? 0 : -1} aria-label="Open quick actions" aria-expanded={sheetOpen}>
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
