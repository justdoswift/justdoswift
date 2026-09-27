import Link from "next/link";
import { ArrowUpRight } from "lucide-react";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <span>© 2026 Just Do Swift</span>
      <div><Link href="/guide">Guide</Link><a href="https://github.com/justdoswift/justdoswift" target="_blank" rel="noreferrer">GitHub <ArrowUpRight /></a><a href="https://ui.shadcn.com" target="_blank" rel="noreferrer">UI inspired by shadcn/ui <ArrowUpRight /></a></div>
    </footer>
  );
}
