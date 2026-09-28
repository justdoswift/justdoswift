import type { Metadata } from "next";
import { CalendarDoc } from "@/components/calendar-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Calendar · GPUI",
  description: "A date field component for picking a single date or a range — month/year dropdowns, presets, date-time composition and disabled dates.",
  alternates: { canonical: "/gpui/calendar" },
  openGraph: {
    title: "Calendar · GPUI — Just Do Swift",
    description: "A date field component for picking a single date or a range of dates.",
    url: "/gpui/calendar",
    type: "website",
  },
};

export default function CalendarPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-calendar" />
        <CalendarDoc />
      </div>
    </>
  );
}
