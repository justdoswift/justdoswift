"use client";

import { Check, Copy, Play, Sparkles } from "lucide-react";
import { ActionSwapBlurButton } from "@/components/motion/action-swap-blur";

const BUTTON_STATES = [
  { id: "copy", label: "Copy code", icon: <Copy className="size-4" /> },
  { id: "copied", label: "Copied", icon: <Check className="size-4" /> },
];

export function HeroShowcase() {
  return (
    <div className="hero-showcase" aria-label="SwiftUI component preview">
      <div className="showcase-glow" />
      <div className="showcase-window">
        <div className="window-chrome">
          <div><span /><span /><span /></div>
          <p><Sparkles /> ActionSwap.swift</p>
          <button type="button" aria-label="Play preview"><Play /></button>
        </div>
        <div className="showcase-content">
          <div className="showcase-code" aria-hidden>
            <span><i className="purple">struct</i> <i className="blue">ActionSwap</i>: View &#123;</span>
            <span>&nbsp;&nbsp;<i className="purple">@State</i> <i className="blue">private var</i> isComplete = <i className="orange">false</i></span>
            <span>&nbsp;</span>
            <span>&nbsp;&nbsp;<i className="blue">var</i> body: <i className="cyan">some View</i> &#123;</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;<i className="yellow">Button</i> &#123;</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;copySource()</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;&#125; label: &#123;</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;<i className="yellow">Label</i>(<i className="green">&quot;Copy code&quot;</i>,</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;systemImage: <i className="green">&quot;doc.on.doc&quot;</i>)</span>
            <span>&nbsp;&nbsp;&nbsp;&nbsp;&#125;</span>
            <span>&nbsp;&nbsp;&#125;</span>
            <span>&#125;</span>
          </div>
          <div className="showcase-preview">
            <div className="preview-device">
              <div className="device-status"><span>9:41</span><i /></div>
              <div className="device-copy">
                <small>MICRO INTERACTION</small>
                <strong>Make feedback<br />feel immediate.</strong>
                <p>Tap the component to see the state change.</p>
              </div>
              <ActionSwapBlurButton
                items={BUTTON_STATES}
                variant="primary"
                size="lg"
                className="hero-demo-button"
              />
              <span className="device-home" />
            </div>
          </div>
        </div>
      </div>
      <div className="floating-pill floating-pill-top"><span /> Swift 6 ready</div>
      <div className="floating-pill floating-pill-bottom"><Check /> Reduce Motion</div>
    </div>
  );
}
