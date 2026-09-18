import { Suspense } from "react";
import { ComponentGallery } from "@/components/component-gallery";
import { SiteHeader } from "@/components/site-header";
import { SiteFooter } from "@/components/site-footer";
import { LibrarySidebar } from "@/components/library-sidebar";
import { LibraryResources } from "@/components/library-resources";

export default function HomePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="home" />
        <main id="main-content" className="catalog-main">
          <div className="breadcrumb"><span>Library</span><span>/</span><span>Components</span></div>
          <header className="page-heading">
            <div className="heading-eyebrow"><span /> Built with SwiftUI</div>
            <h1>Small components.<br />Delightful interactions.</h1>
            <p>精选 SwiftUI 组件与动效。发现一个细节，看看它如何运作，<br className="desktop-break" />再把源码带进你的下一个项目。</p>
          </header>
          <Suspense fallback={<div className="catalog-loading">Loading components…</div>}><ComponentGallery /></Suspense>
          <div className="collection-end"><span className="tiny-spark">✳</span><span>A small collection, always growing.</span><a href="https://github.com/justdoswift/justdoswift" target="_blank" rel="noreferrer">Follow on GitHub ↗</a></div>
          <SiteFooter />
        </main>
        <LibraryResources />
      </div>
    </>
  );
}
