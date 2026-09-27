import type { Metadata } from "next";
import { AlertDialogDoc } from "@/components/alert-dialog-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Alert Dialog · GPUI",
  description: "A modal dialog that interrupts the user with important content and expects a response. 桌面与网页共用 Rust 源码的 GPUI 确认对话框。",
  alternates: { canonical: "/gpui/alert-dialog" },
  openGraph: {
    title: "Alert Dialog · GPUI — Just Do Swift",
    description: "A modal dialog that interrupts the user with important content and expects a response.",
    url: "/gpui/alert-dialog",
    type: "website",
  },
};

export default function AlertDialogPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-alert-dialog" />
        <AlertDialogDoc />
      </div>
    </>
  );
}
