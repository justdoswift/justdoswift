import type { Metadata, Viewport } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://justdoswift.com"),
  title: {
    default: "Just Do Swift — Beautiful SwiftUI components",
    template: "%s — Just Do Swift",
  },
  description:
    "Curated SwiftUI components and motion recipes with live previews, production-ready source code and implementation notes.",
  keywords: ["SwiftUI", "Swift", "iOS", "components", "animation", "Apple development"],
  openGraph: {
    title: "Just Do Swift — Beautiful SwiftUI components",
    description: "Preview the motion. Understand the details. Own the SwiftUI source.",
    url: "https://justdoswift.com",
    siteName: "Just Do Swift",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "Just Do Swift",
    description: "Beautiful SwiftUI components, ready to use.",
  },
};

export const viewport: Viewport = {
  colorScheme: "dark",
  themeColor: "#070707",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="zh-CN">
      <body>{children}</body>
    </html>
  );
}
