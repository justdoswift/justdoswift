import Link from "next/link";
import { ArrowRight, Check, Code2, Gauge, Layers3, Play, Sparkles } from "lucide-react";
import { ComponentGallery } from "@/components/component-gallery";
import { HeroShowcase } from "@/components/hero-showcase";
import { SiteFooter } from "@/components/site-footer";
import { SiteHeader } from "@/components/site-header";

export default function HomePage() {
  return (
    <main>
      <SiteHeader />
      <section className="hero-section">
        <div className="hero-grid-bg" />
        <div className="shell hero-layout">
          <div className="hero-copy">
            <div className="eyebrow"><span /> Curated SwiftUI library</div>
            <h1>Make SwiftUI<br /><em>feel alive.</em></h1>
            <p>
              精选 SwiftUI 组件与动效。看见效果，理解细节，复制源码，直接放进你的下一个 Apple 平台项目。
            </p>
            <div className="hero-actions">
              <Link href="/#library" className="primary-button">
                Explore components <ArrowRight />
              </Link>
              <Link href="/components/action-swap" className="secondary-button">
                <Play /> Watch preview
              </Link>
            </div>
            <div className="hero-metrics">
              <div><strong>06</strong><span>Components</span></div>
              <div><strong>17+</strong><span>Minimum iOS</span></div>
              <div><strong>0</strong><span>Dependencies</span></div>
            </div>
          </div>
          <HeroShowcase />
        </div>
      </section>

      <div className="shell library-intro-strip" aria-label="Library qualities">
        <span><Sparkles /> Native motion</span>
        <i />
        <span><Code2 /> Production code</span>
        <i />
        <span><Gauge /> Performance minded</span>
        <i />
        <span><Check /> Accessibility included</span>
      </div>

      <ComponentGallery />

      <section className="principles-section" id="principles">
        <div className="shell">
          <div className="principles-heading">
            <span className="section-kicker">The standard</span>
            <h2>不只是好看。<br />更应该好用。</h2>
            <p>每个实现都从原生体验、性能和可访问性出发，保留足够的解释，让你真正拥有代码。</p>
          </div>
          <div className="principles-grid">
            <article>
              <span className="principle-number">01</span>
              <div className="principle-icon"><Layers3 /></div>
              <h3>Native first</h3>
              <p>优先使用 SwiftUI 原生 API 与系统行为，不用脆弱的视觉障眼法。</p>
            </article>
            <article>
              <span className="principle-number">02</span>
              <div className="principle-icon"><Gauge /></div>
              <h3>Motion with purpose</h3>
              <p>动效用来解释状态变化、空间关系和操作结果，而不是制造等待。</p>
            </article>
            <article>
              <span className="principle-number">03</span>
              <div className="principle-icon"><Code2 /></div>
              <h3>Own the source</h3>
              <p>代码直接进入你的项目，可以阅读、修改和组合，不绑定专有运行时。</p>
            </article>
          </div>
        </div>
      </section>

      <section className="closing-section">
        <div className="shell closing-card">
          <div className="closing-orb" />
          <span className="section-kicker">Start building</span>
          <h2>One thoughtful interaction<br />can change the whole app.</h2>
          <p>从一个组件开始，把你的 SwiftUI 产品做得更清晰、更有触感。</p>
          <Link href="/components/action-swap" className="primary-button dark">
            Open Action Swap <ArrowRight />
          </Link>
        </div>
      </section>
      <SiteFooter />
    </main>
  );
}
