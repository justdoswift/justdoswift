import type { Metadata } from "next";
import { AttachmentDoc } from "@/components/attachment-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Attachment · GPUI",
  description: "Displays a file or image attachment with media, metadata, upload state, and actions. 桌面与网页共用 Rust 源码的 GPUI 附件卡片。",
  alternates: { canonical: "/gpui/attachment" },
  openGraph: {
    title: "Attachment · GPUI — Just Do Swift",
    description: "Displays a file or image attachment with media, metadata, upload state, and actions.",
    url: "/gpui/attachment",
    type: "website",
  },
};

export default function AttachmentPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-attachment" />
        <AttachmentDoc />
      </div>
    </>
  );
}
