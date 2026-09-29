import type { Metadata } from "next";
import { CommandDoc } from "@/components/command-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Command · GPUI",
  description: "Fast, composable, unstyled command menu. 桌面与网页共用 Rust 源码的 GPUI 命令面板。",
  alternates: { canonical: "/gpui/command" },
  openGraph: {
    title: "Command · GPUI — Just Do Swift",
    description: "Fast, composable, unstyled command menu.",
    url: "/gpui/command",
    type: "website",
  },
};

export default function CommandPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-command" />
        <CommandDoc />
      </div>
    </>
  );
}
