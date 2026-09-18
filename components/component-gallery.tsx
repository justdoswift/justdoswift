"use client";

import Link from "next/link";
import { useSearchParams, useRouter } from "next/navigation";
import { ArrowUpRight, Search, X } from "lucide-react";
import { useMemo, useState } from "react";
import { PreviewStage } from "@/components/preview-stage";
import { categories, swiftComponents } from "@/lib/components";

export function ComponentGallery() {
  const [query, setQuery] = useState("");
  const params = useSearchParams();
  const router = useRouter();
  const candidate = params.get("category");
  const category = categories.find((item) => item === candidate) ?? "All";
  const filtered = useMemo(() => swiftComponents.filter((component) => (category === "All" || component.category === category) && `${component.title} ${component.description} ${component.category}`.toLowerCase().includes(query.trim().toLowerCase())), [category, query]);

  return (
    <section className="library-section" id="library" aria-label="Browse components">
      <div className="library-toolbar">
        <div className="category-tabs" aria-label="Filter by category">
          {categories.map((item) => <button type="button" aria-pressed={category === item} className={category === item ? "selected" : ""} onClick={() => router.replace(item === "All" ? "/" : `/?category=${item}`, { scroll: false })} key={item}>{item}{item === "All" && <span>{swiftComponents.length}</span>}</button>)}
        </div>
        <div className="library-search"><Search /><input value={query} onChange={(event) => setQuery(event.target.value)} placeholder="Filter…" aria-label="Filter components" />{query && <button type="button" onClick={() => setQuery("")} aria-label="Clear filter"><X /></button>}</div>
      </div>
      <p className="result-count" aria-live="polite">{filtered.length} {filtered.length === 1 ? "component" : "components"}</p>
      {filtered.length ? (
        <div className="component-grid">
          {filtered.map((component) => (
            <article className="component-card" key={component.slug}>
              <Link href={`/components/${component.slug}`} className="card-hit" aria-label={`View ${component.title}`} />
              <div className={`card-preview${component.video ? " card-preview-video" : ""}`} aria-hidden="true"><PreviewStage {...component} compact /><span className="preview-type">{component.videoKind === "remotion" ? "Remotion · 8s" : component.video ? "MP4" : "Preview"}</span><span className="card-open"><ArrowUpRight /></span></div>
              <div className="card-copy"><div className="card-title-row"><h2>{component.title}</h2><span>{component.ios}</span></div><p>{component.description}</p><div className="card-category">{component.category}<span>SwiftUI</span></div></div>
            </article>
          ))}
        </div>
      ) : <div className="empty-state"><Search /><h2>No components found</h2><p>试试其他关键词，或查看全部组件。</p><button className="pill-button" type="button" onClick={() => { setQuery(""); router.replace("/", { scroll: false }); }}>Clear filters</button></div>}
    </section>
  );
}
