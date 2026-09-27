import type { Metadata } from "next";
import { AccordionDoc } from "@/components/accordion-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Accordion · GPUI",
  description: "A vertically stacked set of interactive headings that each reveal a section of content. 桌面与网页共用 Rust 源码的 GPUI 手风琴。",
  alternates: { canonical: "/gpui/accordion" },
  openGraph: {
    title: "Accordion · GPUI — Just Do Swift",
    description: "A vertically stacked set of interactive headings that each reveal a section of content.",
    url: "/gpui/accordion",
    type: "website",
  },
};

export default function AccordionPage() {
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
