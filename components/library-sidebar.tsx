import Link from "next/link";
import { ArrowUpRight, BookOpen, Grid2X2 } from "lucide-react";
import { gpuiComponents } from "@/lib/gpui-components";

export function LibraryNavigation({ active }: { active?: string }) {
  return (
    <nav className="library-navigation" aria-label="Component library">
      <div className="nav-group">
        <div className="nav-heading">GPUI<span>{gpuiComponents.length}</span></div>
        {gpuiComponents.map((item) => (
          <Link key={item.slug} href={item.href} className={active === `gpui-${item.slug}` ? "nav-link active" : "nav-link"} aria-current={active === `gpui-${item.slug}` ? "page" : undefined}>{item.title}<span className="new-dot" /></Link>
        ))}
      </div>
      <div className="nav-group">
        <div className="nav-heading">Getting started</div>
        <Link href="/" className={active === "home" ? "nav-link active" : "nav-link"} aria-current={active === "home" ? "page" : undefined}><Grid2X2 /> All components <span className="nav-count">{gpuiComponents.length}</span></Link>
        <Link href="/guide" className={active === "guide" ? "nav-link active" : "nav-link"} aria-current={active === "guide" ? "page" : undefined}><BookOpen /> Introduction</Link>
      </div>
    </nav>
  );
}

export function LibrarySidebar({ active }: { active?: string }) {
  return (
    <aside className="library-sidebar">
      <LibraryNavigation active={active} />
      <a className="sidebar-bottom" href="https://github.com/justdoswift/justdoswift" target="_blank" rel="noreferrer">Built in the open <ArrowUpRight /></a>
    </aside>
  );
}
