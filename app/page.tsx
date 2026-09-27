import type { Metadata } from "next";
import { AccordionDoc } from "@/components/accordion-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Accordion · GPUI",
  description: "A vertically stacked set of interactive headings that each reveal a section of content. 第一个 GPUI 组件：预览交互、阅读 Rust 源码，把同一份组件带进桌面应用。",
  alternates: { canonical: "/" },
};

export default function HomePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-accordion" />
        <AccordionDoc />
      </div>
    </>
  );
}
