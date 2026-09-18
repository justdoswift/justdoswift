import Link from "next/link";
import { ArrowUpRight, Code2, ArrowRight, Braces } from "lucide-react";

export function LibraryResources() {
  return (
    <aside className="resource-rail" aria-label="Resources">
      <div className="resource-card">
        <div className="resource-art" aria-hidden="true"><span className="art-orbit orbit-one" /><span className="art-orbit orbit-two" /><div className="art-code"><Braces /></div><span className="art-caption">Made for SwiftUI</span></div>
        <div className="resource-copy"><h2>A little detail.<br />A better app.</h2><p>从一个交互开始。<br />预览动效，阅读源码，<br />让细节成为你的设计。</p><Link className="pill-button" href="/components/action-swap">Explore Action Swap <ArrowUpRight /></Link></div>
      </div>
      <div className="resource-note"><Code2 /><h3>Copy. Paste. Customize.</h3><p>独立的 SwiftUI 示例，附使用方式与源码。按你的项目自由调整。</p><Link href="/guide">Read the guide <ArrowRight /></Link></div>
      <div className="rail-footnote">Thoughtful interfaces.<br />One component at a time.</div>
    </aside>
  );
}
