import type { Metadata, Viewport } from "next";
import "./globals.css";

export const metadata: Metadata = {
  metadataBase: new URL("https://justdoswift.com"),
  title: {
    default: "Just Do Swift — Native GPUI components",
    template: "%s — Just Do Swift",
  },
  description:
    "Native GPUI components for desktop apps. Explore Accordion, try live WebAssembly previews, and use the same Rust source in your app.",
  keywords: ["GPUI", "Rust", "desktop", "components", "animation", "WebAssembly"],
  icons: { icon: "/favicon.svg" },
  openGraph: {
    title: "Just Do Swift — Native GPUI components",
    description: "Desktop components. Live previews. The same Rust source.",
    url: "https://justdoswift.com",
    siteName: "Just Do Swift",
    type: "website",
  },
  twitter: {
    card: "summary_large_image",
    title: "Just Do Swift",
    description: "Native GPUI components, live previews and Rust source examples.",
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
