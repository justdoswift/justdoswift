import type { Metadata } from "next";
import { ContextMenuDoc } from "@/components/context-menu-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Context Menu · GPUI",
  description: "Displays a menu of actions triggered by a right click. 桌面与网页共用 Rust 源码的 GPUI 右键菜单。",
  alternates: { canonical: "/gpui/context-menu" },
  openGraph: {
    title: "Context Menu · GPUI — Just Do Swift",
    description: "Displays a menu of actions triggered by a right click.",
    url: "/gpui/context-menu",
    type: "website",
  },
};

export default function ContextMenuPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-context-menu" />
        <ContextMenuDoc />
      </div>
    </>
  );
}
