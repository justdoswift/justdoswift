import type { MetadataRoute } from "next";
import { gpuiComponents } from "@/lib/gpui-components";

export default function sitemap(): MetadataRoute.Sitemap {
  const updated = new Date("2026-09-25");
  return [
    { url: "https://justdoswift.com", lastModified: updated, changeFrequency: "weekly", priority: 1 },
    { url: "https://justdoswift.com/guide", lastModified: updated, changeFrequency: "monthly", priority: 0.6 },
    ...gpuiComponents.map((component) => ({
      url: `https://justdoswift.com${component.href}`,
      lastModified: updated,
      changeFrequency: "monthly" as const,
      priority: 0.9,
    })),
  ];
}
