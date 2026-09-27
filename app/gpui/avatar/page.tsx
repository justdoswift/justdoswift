import type { Metadata } from "next";
import { AvatarDoc } from "@/components/avatar-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Avatar · GPUI",
  description: "An image element with a fallback for representing the user. 桌面与网页共用 Rust 源码的 GPUI 头像。",
  alternates: { canonical: "/gpui/avatar" },
  openGraph: {
    title: "Avatar · GPUI — Just Do Swift",
    description: "An image element with a fallback for representing the user.",
    url: "/gpui/avatar",
    type: "website",
  },
};

export default function AvatarPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-avatar" />
        <AvatarDoc />
      </div>
    </>
  );
}
