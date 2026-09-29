import type { Metadata } from "next";
import { ComboboxDoc } from "@/components/combobox-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Combobox · GPUI",
  description: "Autocomplete input and command palette with a list of suggestions. 桌面与网页共用 Rust 源码的 GPUI 组合框。",
  alternates: { canonical: "/gpui/combobox" },
  openGraph: {
    title: "Combobox · GPUI — Just Do Swift",
    description: "Autocomplete input and command palette with a list of suggestions.",
    url: "/gpui/combobox",
    type: "website",
  },
};

export default function ComboboxPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-combobox" />
        <ComboboxDoc />
      </div>
    </>
  );
}
