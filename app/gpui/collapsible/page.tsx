import type { Metadata } from "next";
import { CollapsibleDoc } from "@/components/collapsible-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Collapsible · GPUI",
  description: "An interactive component which expands/collapses a panel. 桌面与网页共用 Rust 源码的 GPUI 折叠面板。",
  alternates: { canonical: "/gpui/collapsible" },
  openGraph: {
    title: "Collapsible · GPUI — Just Do Swift",
    description: "An interactive component which expands/collapses a panel.",
    url: "/gpui/collapsible",
    type: "website",
  },
};

export default function CollapsiblePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-collapsible" />
        <CollapsibleDoc />
      </div>
    </>
  );
}
