import Link from "next/link";
import { ArrowLeft } from "lucide-react";
import { SiteHeader } from "@/components/site-header";
import { Button } from "@/components/ui/button";

export default function NotFound() {
  return (
    <main id="main-content">
      <SiteHeader />
      <section className="not-found">
        <span>404</span>
        <h1>This component slipped away.</h1>
        <p>这个页面已移除或不存在。当前组件库提供 Accordion。</p>
        <Button className="rounded-full" size="lg" asChild>
          <Link href="/"><ArrowLeft /> Back to library</Link>
        </Button>
      </section>
    </main>
  );
}
