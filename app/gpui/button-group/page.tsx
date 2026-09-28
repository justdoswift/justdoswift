import type { Metadata } from "next";
import { ButtonGroupDoc } from "@/components/button-group-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Button Group · GPUI",
  description: "A container that groups related buttons together with consistent styling. 横纵排布、分隔线与 split 按钮组。",
  alternates: { canonical: "/gpui/button-group" },
  openGraph: {
    title: "Button Group · GPUI — Just Do Swift",
    description: "A container that groups related buttons together with consistent styling.",
    url: "/gpui/button-group",
    type: "website",
  },
};

export default function ButtonGroupPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-button-group" />
        <ButtonGroupDoc />
      </div>
    </>
  );
}
