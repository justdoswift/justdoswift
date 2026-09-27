import type { Metadata } from "next";
import { BubbleDoc } from "@/components/bubble-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Bubble · GPUI",
  description: "Displays conversational content in a message bubble. 桌面与网页共用 Rust 源码的 GPUI 聊天气泡。",
  alternates: { canonical: "/gpui/bubble" },
  openGraph: {
    title: "Bubble · GPUI — Just Do Swift",
    description: "Displays conversational content in a message bubble.",
    url: "/gpui/bubble",
    type: "website",
  },
};

export default function BubblePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-bubble" />
        <BubbleDoc />
      </div>
    </>
  );
}
