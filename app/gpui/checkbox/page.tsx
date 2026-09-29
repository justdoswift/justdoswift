import type { Metadata } from "next";
import { CheckboxDoc } from "@/components/checkbox-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Checkbox · GPUI",
  description: "A control that allows the user to toggle between checked and not checked. 桌面与网页共用 Rust 源码的 GPUI 复选框。",
  alternates: { canonical: "/gpui/checkbox" },
  openGraph: {
    title: "Checkbox · GPUI — Just Do Swift",
    description: "A control that allows the user to toggle between checked and not checked.",
    url: "/gpui/checkbox",
    type: "website",
  },
};

export default function CheckboxPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-checkbox" />
        <CheckboxDoc />
      </div>
    </>
  );
}
