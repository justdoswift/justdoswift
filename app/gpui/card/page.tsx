import type { Metadata } from "next";
import { CardDoc } from "@/components/card-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Card · GPUI",
  description: "A card container with header, content, and footer sections — spacing scale, edge-to-edge media and bordered sections.",
  alternates: { canonical: "/gpui/card" },
  openGraph: {
    title: "Card · GPUI — Just Do Swift",
    description: "A card container with header, content, and footer sections.",
    url: "/gpui/card",
    type: "website",
  },
};

export default function CardPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-card" />
        <CardDoc />
      </div>
    </>
  );
}
