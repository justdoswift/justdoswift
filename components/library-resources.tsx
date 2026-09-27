import Link from "next/link";
import { ArrowUpRight, Code2, ArrowRight, Braces } from "lucide-react";
import { Button } from "@/components/ui/button";

export function LibraryResources() {
  return (
    <aside className="resource-rail" aria-label="Resources">
      <div className="resource-card">
        <div className="resource-art" aria-hidden="true"><span className="art-orbit orbit-one" /><span className="art-orbit orbit-two" /><div className="art-code"><Braces /></div><span className="art-caption">Made with Rust &amp; GPUI</span></div>
        <div className="resource-copy"><h2>A little detail.<br />A better app.</h2><p>从一个交互开始。<br />预览动效，阅读源码，<br />让细节成为你的设计。</p><Button className="w-full rounded-full" size="sm" asChild><Link href="/gpui/accordion">Explore Accordion <ArrowUpRight /></Link></Button></div>
      </div>
      <div className="resource-note"><Code2 /><h3>Read. Run. Customize.</h3><p>完整的 Rust / GPUI 示例，桌面与网页共用组件源码。下载后运行，按你的项目调整。</p><Link href="/guide">Read the guide <ArrowRight /></Link></div>
      <div className="rail-footnote">Thoughtful interfaces.<br />One component at a time.</div>
    </aside>
  );
}
