import type { Metadata } from "next";
import { ChartDoc } from "@/components/chart-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Chart · GPUI",
  description: "Hand-painted GPUI charts — grouped and stacked bars, line, area and donut with hover tooltips, legends and theming.",
  alternates: { canonical: "/gpui/chart" },
  openGraph: {
    title: "Chart · GPUI — Just Do Swift",
    description: "Hand-painted GPUI charts with hover tooltips, legends and theming.",
    url: "/gpui/chart",
    type: "website",
  },
};

export default function ChartPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-chart" />
        <ChartDoc />
      </div>
    </>
  );
}
