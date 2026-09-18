import Link from "next/link";
import { ArrowUpRight } from "lucide-react";
import { BrandMark } from "@/components/brand-mark";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <div className="shell footer-grid">
        <div>
          <Link href="/" aria-label="Just Do Swift home">
            <BrandMark />
          </Link>
          <p className="footer-note">Crafted SwiftUI components, motion recipes and implementation notes.</p>
        </div>
        <div className="footer-links">
          <div>
            <span>Explore</span>
            <Link href="/#library">Components</Link>
            <Link href="/#principles">Principles</Link>
          </div>
          <div>
            <span>Connect</span>
            <a href="https://github.com/justdoswift/justdoswift" target="_blank" rel="noreferrer">
              GitHub <ArrowUpRight className="size-3.5" />
            </a>
            <a href="https://x.com/justdoswift" target="_blank" rel="noreferrer">
              X <ArrowUpRight className="size-3.5" />
            </a>
          </div>
        </div>
      </div>
      <div className="shell footer-bottom">
        <span>© 2026 Just Do Swift</span>
        <span>Built for the Apple ecosystem.</span>
      </div>
    </footer>
  );
}
