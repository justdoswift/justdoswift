"use client";

import Link from "next/link";
import { usePathname, useRouter } from "next/navigation";
import { ArrowRight, Github, Menu, Moon, Search, Sun, X } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { BrandMark } from "@/components/brand-mark";
import { LibraryNavigation } from "@/components/library-sidebar";
import { Button } from "@/components/ui/button";
import { gpuiComponents } from "@/lib/gpui-components";

export function SiteHeader() {
  const pathname = usePathname();
  const router = useRouter();
  const dialog = useRef<HTMLDialogElement>(null);
  const searchInput = useRef<HTMLInputElement>(null);
  const trigger = useRef<HTMLButtonElement>(null);
  const [menuOpen, setMenuOpen] = useState(false);
  const [searchOpen, setSearchOpen] = useState(false);
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  const [dark, setDark] = useState(false);
  const results = gpuiComponents.filter((item) => `${item.title} ${item.description} ${item.category}`.toLowerCase().includes(query.toLowerCase()));

  useEffect(() => {
    try {
      const preference = localStorage.getItem("justdoswift-theme") === "dark";
      setDark(preference);
      document.documentElement.dataset.theme = preference ? "dark" : "light";
    } catch { /* The default light theme works without storage. */ }
    function shortcut(event: KeyboardEvent) {
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        setSearchOpen((open) => !open);
      }
      if (event.key === "Escape") setMenuOpen(false);
    }
    window.addEventListener("keydown", shortcut);
    return () => window.removeEventListener("keydown", shortcut);
  }, []);

  useEffect(() => {
    setMenuOpen(false);
    setSearchOpen(false);
    // The sticky documentation shell should not hide a new page's heading.
    if (!window.location.hash) window.scrollTo({ top: 0, behavior: "instant" });
  }, [pathname]);
  useEffect(() => {
    if (searchOpen) {
      dialog.current?.showModal();
      searchInput.current?.focus();
    } else {
      dialog.current?.close();
    }
  }, [searchOpen]);

  function toggleTheme() {
    const next = !dark;
    setDark(next);
    document.documentElement.dataset.theme = next ? "dark" : "light";
    try { localStorage.setItem("justdoswift-theme", next ? "dark" : "light"); } catch { /* Optional persistence. */ }
  }

  function choose(slug: string) {
    setSearchOpen(false);
    setQuery("");
    setSelected(0);
    const component = gpuiComponents.find((item) => item.slug === slug);
    if (component) router.push(component.href);
  }

  return (
    <>
      <a className="skip-link" href="#main-content">Skip to content</a>
      <header className="site-header">
        <Link href="/" className="header-brand" aria-label="Just Do Swift home"><BrandMark /></Link>
        <nav className="desktop-nav" aria-label="Primary navigation">
          <Link href="/" className={pathname === "/" || pathname.startsWith("/gpui") ? "current" : ""}>Components</Link>
          <Link href="/guide" className={pathname === "/guide" ? "current" : ""}>Guide</Link>
        </nav>
        <div className="header-actions">
          <Button ref={trigger} variant="outline" className="search-trigger h-9 w-[170px] justify-start gap-2 rounded-full px-3 text-xs font-normal text-muted-foreground shadow-none max-[800px]:w-9 max-[800px]:justify-center max-[800px]:px-0" onClick={() => { setSearchOpen(true); setSelected(0); }} aria-label="Search components"><Search className="size-3.5" /><span className="max-[800px]:hidden">Search</span><kbd className="ml-auto max-[800px]:hidden">⌘ K</kbd></Button>
          <Button variant="outline" size="icon" className="size-9 rounded-full" onClick={toggleTheme} aria-label={dark ? "Switch to light mode" : "Switch to dark mode"}>{dark ? <Sun /> : <Moon />}</Button>
          <Button variant="outline" size="icon" className="github-action size-9 rounded-full" asChild>
            <a href="https://github.com/justdoswift/justdoswift" target="_blank" rel="noreferrer" aria-label="GitHub repository"><Github /></a>
          </Button>
          <Button className="header-cta rounded-full" size="sm" asChild>
            <Link href="/guide">Get started <ArrowRight /></Link>
          </Button>
          <Button variant="outline" size="icon" className="menu-trigger size-9 rounded-full" onClick={() => setMenuOpen((open) => !open)} aria-expanded={menuOpen} aria-controls="mobile-navigation" aria-label={menuOpen ? "Close navigation" : "Open navigation"}>{menuOpen ? <X /> : <Menu />}</Button>
        </div>
      </header>
      {menuOpen && <div className="mobile-navigation" id="mobile-navigation" onClick={(event) => { if ((event.target as HTMLElement).closest("a")) setMenuOpen(false); }}><LibraryNavigation active={pathname === "/" ? "home" : pathname.startsWith("/gpui/") ? `gpui-${pathname.split("/").pop()}` : pathname.split("/").pop()} /></div>}
      <dialog className="search-dialog" ref={dialog} aria-label="Search components" onCancel={() => setSearchOpen(false)} onClose={() => { setSearchOpen(false); trigger.current?.focus(); }} onClick={(event) => { if (event.target === event.currentTarget) setSearchOpen(false); }}>
        <div className="search-dialog-inner">
          <div className="search-dialog-input"><Search /><input ref={searchInput} placeholder="Find a component…" aria-label="Find a component" role="combobox" aria-expanded="true" aria-controls="search-results" aria-activedescendant={results[selected] ? `result-${results[selected].slug}` : undefined} value={query} onChange={(event) => { setQuery(event.target.value); setSelected(0); }} onKeyDown={(event) => {
            if (event.key === "ArrowDown" || event.key === "ArrowUp") {
              event.preventDefault();
              setSelected((index) => Math.max(0, Math.min(results.length - 1, index + (event.key === "ArrowDown" ? 1 : -1))));
            }
            if (event.key === "Enter" && results[selected]) { event.preventDefault(); choose(results[selected].slug); }
          }} /><button type="button" onClick={() => setSearchOpen(false)} aria-label="Close search"><kbd>esc</kbd></button></div>
          <div className="search-results" id="search-results" role="listbox" aria-label="Matching components">
            <span className="search-caption">Components</span>
            {results.map((item, index) => <button type="button" id={`result-${item.slug}`} role="option" aria-selected={selected === index} className={selected === index ? "selected" : ""} key={item.slug} onMouseEnter={() => setSelected(index)} onClick={() => choose(item.slug)}><span><strong>{item.title}</strong><small>GPUI · {item.platform}</small></span><ArrowRight /></button>)}
            {!results.length && <p className="search-empty">没有找到组件，换个关键词试试。</p>}
          </div>
          <div className="search-dialog-footer"><span>↑ ↓ to navigate</span><span>↵ to open</span><span>esc to close</span></div>
        </div>
      </dialog>
    </>
  );
}
