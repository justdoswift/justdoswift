import Link from "next/link";
import { ArrowUpRight, BookOpen, Grid2X2 } from "lucide-react";
import { categories, swiftComponents } from "@/lib/components";

export function LibraryNavigation({ active }: { active?: string }) {
  return (
    <nav className="library-navigation" aria-label="Component library">
      <div className="nav-group">
        <div className="nav-heading">Getting started</div>
        <Link href="/" className={active === "home" ? "nav-link active" : "nav-link"} aria-current={active === "home" ? "page" : undefined}><Grid2X2 /> All components <span className="nav-count">{swiftComponents.length}</span></Link>
        <Link href="/guide" className={active === "guide" ? "nav-link active" : "nav-link"} aria-current={active === "guide" ? "page" : undefined}><BookOpen /> Introduction</Link>
      </div>
      {categories.filter((category) => category !== "All").map((category) => (
        <div className="nav-group" key={category}>
          <div className="nav-heading">{category}<span>{swiftComponents.filter((item) => item.category === category).length}</span></div>
          {swiftComponents.filter((item) => item.category === category).map((item) => (
            <Link key={item.slug} href={`/components/${item.slug}`} className={active === item.slug ? "nav-link active" : "nav-link"} aria-current={active === item.slug ? "page" : undefined}>
              {item.title}{item.featured && <span className="new-dot" title="Featured" />}
            </Link>
          ))}
        </div>
      ))}
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
