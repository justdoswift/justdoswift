import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import {
  Accessibility,
  ArrowLeft,
  ArrowRight,
  Box,
  Check,
  Code2,
  Gauge,
  Layers3,
  Sparkles,
} from "lucide-react";
import { ComponentWorkbench } from "@/components/component-workbench";
import { CopyCodeButton } from "@/components/copy-code-button";
import { PreviewStage } from "@/components/preview-stage";
import { SiteFooter } from "@/components/site-footer";
import { SiteHeader } from "@/components/site-header";
import { getComponent, swiftComponents } from "@/lib/components";

type PageProps = { params: Promise<{ slug: string }> };

export function generateStaticParams() {
  return swiftComponents.map((component) => ({ slug: component.slug }));
}

export async function generateMetadata({ params }: PageProps): Promise<Metadata> {
  const { slug } = await params;
  const component = getComponent(slug);
  if (!component) return {};
  return {
    title: component.title,
    description: component.description,
    alternates: { canonical: `/components/${component.slug}` },
  };
}

export default async function ComponentPage({ params }: PageProps) {
  const { slug } = await params;
  const component = getComponent(slug);
  if (!component) notFound();
  const related = swiftComponents.filter((item) => item.slug !== component.slug).slice(0, 3);

  return (
    <main>
      <SiteHeader />
      <div className="shell detail-shell">
        <aside className="component-rail" aria-label="Component library">
          <Link href="/#library" className="rail-back"><ArrowLeft /> All components</Link>
          <span className="rail-heading">Components</span>
          <nav>
            {swiftComponents.map((item) => (
              <Link
                href={`/components/${item.slug}`}
                key={item.slug}
                className={item.slug === component.slug ? "active" : ""}
              >
                <span style={{ background: item.accent }} />
                {item.title}
              </Link>
            ))}
          </nav>
          <div className="rail-note">
            <Sparkles />
            <p><strong>Built for Swift 6</strong>Carefully reviewed for modern SwiftUI.</p>
          </div>
        </aside>

        <article className="detail-main">
          <div className="detail-breadcrumb">
            <Link href="/#library">Components</Link><span>/</span><span>{component.category}</span>
          </div>
          <header className="detail-header">
            <div>
              <div className="detail-labels">
                <span>{component.access}</span>
                <span>Updated Sep 2026</span>
              </div>
              <h1>{component.title}</h1>
              <p>{component.longDescription}</p>
            </div>
            <CopyCodeButton code={component.source} />
          </header>

          <div className="detail-content-grid">
            <ComponentWorkbench component={component} />
            <aside className="component-facts">
              <span className="facts-title">Component details</span>
              <dl>
                <div><dt>Platform</dt><dd><Box /> SwiftUI</dd></div>
                <div><dt>Minimum</dt><dd>{component.ios}</dd></div>
                <div><dt>Language</dt><dd>{component.swift}</dd></div>
                <div><dt>Category</dt><dd>{component.category}</dd></div>
                <div><dt>Dependencies</dt><dd>None</dd></div>
              </dl>
              <div className="facts-checks">
                <span><Check /> Reduce Motion</span>
                <span><Check /> VoiceOver labels</span>
                <span><Check /> Dynamic Type</span>
              </div>
            </aside>
          </div>

          <section className="detail-notes">
            <div className="detail-section-heading">
              <span className="section-kicker">Implementation notes</span>
              <h2>Why it works.</h2>
            </div>
            <div className="note-grid">
              <article><span><Layers3 /></span><h3>Stable layout</h3><p>状态变化不会突然改变外部尺寸，文本和图标始终共享同一个视觉锚点。</p></article>
              <article><span><Gauge /></span><h3>Spring, tuned</h3><p>弹簧参数强调快速反馈，并在过冲产生干扰之前自然停止。</p></article>
              <article><span><Accessibility /></span><h3>Motion aware</h3><p>开启 Reduce Motion 后自动退化为清晰的状态替换，语义信息保持完整。</p></article>
            </div>
          </section>

          <section className="related-section">
            <div className="related-heading"><div><span className="section-kicker">Keep exploring</span><h2>Related components</h2></div><Link href="/#library">View all <ArrowRight /></Link></div>
            <div className="related-grid">
              {related.map((item) => (
                <article key={item.slug}>
                  <Link href={`/components/${item.slug}`} className="card-hit" aria-label={`View ${item.title}`} />
                  <div className="related-preview"><PreviewStage {...item} compact /></div>
                  <div><span>{item.category}</span><h3>{item.title}</h3><ArrowRight /></div>
                </article>
              ))}
            </div>
          </section>
        </article>
      </div>
      <SiteFooter />
    </main>
  );
}
