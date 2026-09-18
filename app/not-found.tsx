import Link from "next/link";
import { ArrowLeft } from "lucide-react";
import { SiteHeader } from "@/components/site-header";

export default function NotFound() {
  return (
    <main>
      <SiteHeader />
      <section className="not-found">
        <span>404</span>
        <h1>This component slipped away.</h1>
        <p>它可能还在构建中，或者已经移动到了新的位置。</p>
        <Link href="/#library" className="primary-button"><ArrowLeft /> Back to library</Link>
      </section>
    </main>
  );
}
