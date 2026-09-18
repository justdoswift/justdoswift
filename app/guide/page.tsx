import type { Metadata } from "next";
import Link from "next/link";
import { ArrowRight, BookOpen, Code2, MousePointer2 } from "lucide-react";
import { SiteHeader } from "@/components/site-header";
import { SiteFooter } from "@/components/site-footer";
import { LibrarySidebar } from "@/components/library-sidebar";
import { LibraryResources } from "@/components/library-resources";

export const metadata: Metadata = { title: "Introduction", description: "How to use the Just Do Swift component collection." };
export default function GuidePage() {
  return <><SiteHeader /><div className="docs-shell"><LibrarySidebar active="guide" /><main id="main-content" className="guide-main"><div className="breadcrumb"><span>Getting started</span><span>/</span><span>Introduction</span></div><header className="page-heading"><h1>Make it your own.</h1><p>一个持续更新的 SwiftUI 组件与动效合集。<br />从预览到源码，找到适合你项目的交互细节。</p></header><div className="guide-steps"><section><span><MousePointer2 /></span><div><h2>01. Explore the interaction</h2><p>选择一个组件，在 Preview 中查看交互。当前示例使用浏览器演示，页面标记为 Interactive web preview；SwiftUI 录屏会单独标记。</p></div></section><section><span><Code2 /></span><div><h2>02. Copy the Swift source</h2><p>切换到 Code，复制或下载 Swift 源码，再根据 Usage 中的例子将组件加入视图。示例适合作为起点，请在你的 Xcode 项目中编译并适配。</p></div></section><section><span><BookOpen /></span><div><h2>03. Make the details yours</h2><p>阅读 How it works，调整颜色、布局和弹簧参数。上线前请结合你的实际场景检查系统版本、Dynamic Type、VoiceOver 与 Reduce Motion。</p></div></section></div><div className="guide-next"><div><h2>Start with a small interaction.</h2><p>Action Swap 是一个简单的状态反馈例子。</p></div><Link className="pill-button" href="/components/action-swap">Action Swap <ArrowRight /></Link></div><SiteFooter /></main><LibraryResources /></div></>;
}
