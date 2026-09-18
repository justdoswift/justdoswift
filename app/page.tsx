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
            <p>看动效，读源码。把喜欢的细节带进你的 SwiftUI 项目。</p>
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
