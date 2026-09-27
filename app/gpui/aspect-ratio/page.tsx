import type { Metadata } from "next";
import { AspectRatioDoc } from "@/components/aspect-ratio-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Aspect Ratio · GPUI",
  description: "Displays content within a desired ratio. 桌面与网页共用 Rust 源码的 GPUI 比例容器。",
  alternates: { canonical: "/gpui/aspect-ratio" },
  openGraph: {
    title: "Aspect Ratio · GPUI — Just Do Swift",
    description: "Displays content within a desired ratio.",
    url: "/gpui/aspect-ratio",
    type: "website",
  },
};

export default function AspectRatioPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-aspect-ratio" />
        <AspectRatioDoc />
      </div>
    </>
  );
}
