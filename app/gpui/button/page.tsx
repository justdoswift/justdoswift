import type { Metadata } from "next";
import { ButtonDoc } from "@/components/button-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Button · GPUI",
  description: "Displays a button or a component that looks like a button. 桌面与网页共用 Rust 源码的 GPUI 按钮。",
  alternates: { canonical: "/gpui/button" },
  openGraph: {
    title: "Button · GPUI — Just Do Swift",
    description: "Displays a button or a component that looks like a button.",
    url: "/gpui/button",
    type: "website",
  },
};

export default function ButtonPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-button" />
        <ButtonDoc />
      </div>
    </>
  );
}
