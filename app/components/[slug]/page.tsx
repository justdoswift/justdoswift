import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { ArrowUpRight, ChevronRight } from "lucide-react";
import { ComponentWorkbench } from "@/components/component-workbench";
import { CopyCodeButton } from "@/components/copy-code-button";
import { CodeBlock } from "@/components/code-block";
import { SiteFooter } from "@/components/site-footer";
import { SiteHeader } from "@/components/site-header";
import { LibrarySidebar } from "@/components/library-sidebar";
import { getComponent, swiftComponents } from "@/lib/components";
import { componentNotes } from "@/lib/component-notes";

type PageProps = { params: Promise<{ slug: string }> };
export function generateStaticParams() { return swiftComponents.map((component) => ({ slug: component.slug })); }
export async function generateMetadata({ params }: PageProps): Promise<Metadata> {
  const { slug } = await params;
  const component = getComponent(slug);
  return component ? { title: component.title, description: component.description, alternates: { canonical: `/components/${slug}` } } : {};
}

export default async function ComponentPage({ params }: PageProps) {
  const { slug } = await params;
  const component = getComponent(slug);
  if (!component) notFound();
  const related = swiftComponents.filter((item) => item.slug !== slug).sort((a, b) => Number(b.category === component.category) - Number(a.category === component.category)).slice(0, 3);
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active={slug} />
        <main id="main-content" className="detail-main">
          <div className="breadcrumb"><Link href="/">Components</Link><ChevronRight /><span>{component.title}</span></div>
          <header className="detail-heading"><div><h1>{component.title}</h1><p>{component.description}</p></div><CopyCodeButton code={component.source} compact /></header>
          <ComponentWorkbench component={component} />
          <section className="doc-section" id="usage"><h2>Usage</h2><p>将 Code 中的组件加入项目，然后在你的视图里调用。</p>{component.demo && <p className="source-verification">内含 {component.demo} 和 #Preview。已通过 Swift 6 / iOS 17 编译检查。</p>}<CodeBlock code={component.usage} label="Example.swift" /></section>
          <section className="doc-section" id="notes"><h2>How it works</h2><div className="implementation-notes">{componentNotes[slug]?.map((note) => <div key={note.title}><h3>{note.title}</h3><p>{note.text}</p></div>)}</div></section>
          <section className="doc-section" id="related"><h2>Related components</h2><div className="related-grid">{related.map((item) => <Link key={item.slug} href={`/components/${item.slug}`}><h3>{item.title}<ArrowUpRight /></h3><p>{item.description}</p></Link>)}</div></section>
          <SiteFooter />
        </main>
        <aside className="resource-rail detail-rail" aria-label="On this page"><div className="on-this-page"><span>On this page</span><a href="#preview">Overview</a><a href="#usage">Usage</a><a href="#notes">How it works</a><a href="#related">Related components</a></div><div className="component-facts"><h2>At a glance</h2><dl><div><dt>Framework</dt><dd>SwiftUI</dd></div><div><dt>Platform</dt><dd>{component.ios}</dd></div><div><dt>Category</dt><dd>{component.category}</dd></div><div><dt>Source</dt><dd>Swift</dd></div></dl></div><Link className="guide-callout" href="/guide"><span>New here?</span><strong>Make it your own.<ArrowUpRight /></strong><p>A quick guide to the collection.</p></Link></aside>
      </div>
    </>
  );
}
