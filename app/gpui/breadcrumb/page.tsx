import type { Metadata } from "next";
import { BreadcrumbDoc } from "@/components/breadcrumb-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Breadcrumb · GPUI",
  description: "Displays the path to the current resource using a hierarchy of links. 桌面与网页共用 Rust 源码的 GPUI 面包屑。",
  alternates: { canonical: "/gpui/breadcrumb" },
  openGraph: {
    title: "Breadcrumb · GPUI — Just Do Swift",
    description: "Displays the path to the current resource using a hierarchy of links.",
    url: "/gpui/breadcrumb",
    type: "website",
  },
};

export default function BreadcrumbPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-breadcrumb" />
        <BreadcrumbDoc />
      </div>
    </>
  );
}
