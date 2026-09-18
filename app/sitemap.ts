import type { MetadataRoute } from "next";
import { swiftComponents } from "@/lib/components";

export default function sitemap(): MetadataRoute.Sitemap {
  const updated = new Date("2026-09-18");
  return [
    { url: "https://justdoswift.com", lastModified: updated, changeFrequency: "weekly", priority: 1 },
    { url: "https://justdoswift.com/guide", lastModified: updated, changeFrequency: "monthly", priority: 0.6 },
    ...swiftComponents.map((component) => ({
      url: `https://justdoswift.com/components/${component.slug}`,
      lastModified: updated,
      changeFrequency: "monthly" as const,
      priority: 0.8,
    })),
  ];
}
