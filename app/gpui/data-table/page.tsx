import type { Metadata } from "next";
import { DataTableDoc } from "@/components/data-table-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Data Table · GPUI",
  description: "Powerful table and datagrids — sorting, filtering, pagination, row selection, column visibility and row actions. 桌面与网页共用 Rust 源码的 GPUI 数据表格。",
  alternates: { canonical: "/gpui/data-table" },
  openGraph: {
    title: "Data Table · GPUI — Just Do Swift",
    description: "Powerful table and datagrids with sorting, filtering, pagination and row actions.",
    url: "/gpui/data-table",
    type: "website",
  },
};

export default function DataTablePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-data-table" />
        <DataTableDoc />
      </div>
    </>
  );
}
