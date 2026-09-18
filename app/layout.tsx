import type { Metadata, Viewport } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://justdoswift.com"),
  title: {
    default: "Just Do Swift — Beautiful SwiftUI components",
    template: "%s — Just Do Swift",
  },
  description:
    "A growing collection of SwiftUI components and motion recipes with previews, Swift source code and implementation notes.",
  keywords: ["SwiftUI", "Swift", "iOS", "components", "animation", "Apple development"],
  icons: { icon: "/favicon.svg" },
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
    description: "SwiftUI components, interactive previews and source examples.",
  },
};

export const viewport: Viewport = {
  colorScheme: "light dark",
  themeColor: "#fdfdfd",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="zh-CN" data-scroll-behavior="smooth">
      <body>{children}</body>
    </html>
  );
}
