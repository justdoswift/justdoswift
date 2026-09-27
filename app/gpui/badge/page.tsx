import type { Metadata } from "next";
import { BadgeDoc } from "@/components/badge-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Badge · GPUI",
  description: "Displays a badge or a component that looks like a badge. 桌面与网页共用 Rust 源码的 GPUI 徽章。",
  alternates: { canonical: "/gpui/badge" },
  openGraph: {
    title: "Badge · GPUI — Just Do Swift",
    description: "Displays a badge or a component that looks like a badge.",
    url: "/gpui/badge",
    type: "website",
  },
};

export default function BadgePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-badge" />
        <BadgeDoc />
      </div>
    </>
  );
}
