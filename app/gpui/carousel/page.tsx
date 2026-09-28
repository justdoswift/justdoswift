import type { Metadata } from "next";
import { CarouselDoc } from "@/components/carousel-doc";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";

export const metadata: Metadata = {
  title: "Carousel · GPUI",
  description: "An Embla-style slide carousel — animated track, prev/next controls, loop, vertical axis and arrow-key navigation.",
  alternates: { canonical: "/gpui/carousel" },
  openGraph: {
    title: "Carousel · GPUI — Just Do Swift",
    description: "An Embla-style slide carousel with loop, orientation and keyboard navigation.",
    url: "/gpui/carousel",
    type: "website",
  },
};

export default function CarouselPage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="gpui-carousel" />
        <CarouselDoc />
      </div>
    </>
  );
}
