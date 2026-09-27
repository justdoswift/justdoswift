import type { Metadata } from "next";
import { AlertDoc } from "@/components/alert-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Alert · GPUI",
  description: "Displays a callout for user attention. 桌面与网页共用 Rust 源码的 GPUI 提示块。",
  alternates: { canonical: "/gpui/alert" },
  openGraph: {
    title: "Alert · GPUI — Just Do Swift",
    description: "Displays a callout for user attention.",
    url: "/gpui/alert",
    type: "website",
  },
};

export default function AlertPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-alert" />
        <AlertDoc />
      </div>
    </>
  );
}
