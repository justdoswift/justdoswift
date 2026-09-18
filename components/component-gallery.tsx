"use client";

import Link from "next/link";
import { ArrowUpRight, Search, X } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import { PreviewStage } from "@/components/preview-stage";
import { categories, swiftComponents } from "@/lib/components";

export function ComponentGallery() {
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState<(typeof categories)[number]>("All");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    function handleShortcut(event: KeyboardEvent) {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        inputRef.current?.focus();
      }
    }
    window.addEventListener("keydown", handleShortcut);
    return () => window.removeEventListener("keydown", handleShortcut);
  }, []);

  const filtered = useMemo(() => {
    const normalized = query.trim().toLowerCase();
    return swiftComponents.filter((component) => {
      const matchesCategory = category === "All" || component.category === category;
      const matchesSearch =
        !normalized ||
        `${component.title} ${component.description} ${component.category}`.toLowerCase().includes(normalized);
      return matchesCategory && matchesSearch;
    });
  }, [category, query]);

  return (
    <section className="library-section" id="library">
      <div className="shell">
        <div className="section-heading-row">
          <div>
            <span className="section-kicker">The library</span>
            <h2>Built to feel native.</h2>
          </div>
          <p>每一个组件都包含可复现的交互、完整 SwiftUI 源码和实现说明。</p>
        </div>

        <div className="library-toolbar">
          <div className="category-tabs" role="tablist" aria-label="Component categories">
            {categories.map((item) => (
              <button
                type="button"
                role="tab"
                aria-selected={category === item}
                className={category === item ? "selected" : ""}
                onClick={() => setCategory(item)}
                key={item}
              >
                {item}
              </button>
            ))}
          </div>
          <label className="library-search">
            <Search />
            <input
              ref={inputRef}
              value={query}
              onChange={(event) => setQuery(event.target.value)}
              placeholder="Search components"
              aria-label="Search components"
            />
            {query ? (
              <button type="button" onClick={() => setQuery("")} aria-label="Clear search">
                <X />
              </button>
            ) : (
              <kbd>⌘K</kbd>
            )}
          </label>
        </div>

        {filtered.length ? (
          <div className="component-grid">
            {filtered.map((component, index) => (
              <article className="component-card" key={component.slug} style={{ "--delay": `${index * 45}ms` } as React.CSSProperties}>
                <Link
                  href={`/components/${component.slug}`}
                  className="card-hit"
                  aria-label={`View ${component.title}`}
                />
                <div className="card-preview" aria-hidden>
                  <PreviewStage {...component} compact />
                  <span className="card-access">{component.access}</span>
                </div>
                <div className="card-copy">
                  <div>
                    <span>{component.category}</span>
                    <h3>{component.title}</h3>
                  </div>
                  <ArrowUpRight className="card-arrow" />
                  <p>{component.description}</p>
                  <div className="card-meta">
                    <span>{component.ios}</span>
                    <span>{component.swift}</span>
                  </div>
                </div>
              </article>
            ))}
          </div>
        ) : (
          <div className="empty-state">
            <Search />
            <h3>No components found</h3>
            <p>试试别的关键词，或者切换到 All。</p>
            <button type="button" onClick={() => { setQuery(""); setCategory("All"); }}>
              Reset filters
            </button>
          </div>
        )}
      </div>
    </section>
  );
}
