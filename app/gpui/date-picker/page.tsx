import type { Metadata } from "next";
import { DatePickerDoc } from "@/components/date-picker-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Date Picker · GPUI",
  description: "A button + popover + calendar composition for picking a date or a range — typed input, natural-language parsing and a time field.",
  alternates: { canonical: "/gpui/date-picker" },
  openGraph: {
    title: "Date Picker · GPUI — Just Do Swift",
    description: "A button + popover + calendar composition for picking a date or a range.",
    url: "/gpui/date-picker",
    type: "website",
  },
};

export default function DatePickerPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-date-picker" />
        <DatePickerDoc />
      </div>
    </>
  );
}
