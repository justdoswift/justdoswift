import Link from "next/link";
import { Github, Search } from "lucide-react";
import { BrandMark } from "@/components/brand-mark";

export function SiteHeader() {
  return (
    <header className="site-header">
      <div className="shell header-inner">
        <Link href="/" className="header-brand" aria-label="Just Do Swift home">
          <BrandMark />
        </Link>

        <nav className="desktop-nav" aria-label="Primary navigation">
          <Link href="/#library">Components</Link>
          <Link href="/#library">Motion</Link>
          <Link href="/#principles">Principles</Link>
        </nav>

        <div className="header-actions">
          <Link href="/#library" className="icon-button search-action" aria-label="Search components">
            <Search className="size-[18px]" />
            <span>Search</span>
            <kbd>⌘K</kbd>
          </Link>
          <a
            href="https://github.com/justdoswift/justdoswift"
            target="_blank"
            rel="noreferrer"
            className="icon-button github-action"
            aria-label="Open GitHub repository"
          >
            <Github className="size-[18px]" />
          </a>
        </div>
      </div>
    </header>
  );
}
