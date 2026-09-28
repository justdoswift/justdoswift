import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCarouselWorkbench } from "@/components/gpui-carousel-workbench";
import { CopyCodeButton } from "@/components/copy-code-button";
import { CodeBlock } from "@/components/code-block";
import { SiteFooter } from "@/components/site-footer";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

const usage = `use justdo_command::carousel::{Carousel, CarouselSelectEvent};

let carousel = cx.new(|cx| {
    Carousel::new(cx)
        .dark(dark)
        .count(5)            // demo slides 1..=5
        .per_view(1)         // basis-full — one slide per viewport
        .gap(16.)
        .slide_px(336.)      // extent along the scroll axis
        .cross_px(200.)      // cross-axis extent
        .loop_(false)
});

// Current-slide API — emitted by nav buttons and arrow keys.
cx.subscribe(&carousel, |_, _, e: &CarouselSelectEvent, _| {
    println!("slide {} of {}", e.index + 1, e.count);
}).detach();`;

const composition = `Carousel              root — relative, focusable, arrow-key scroll
├── viewport          overflow-hidden clip box
│   └── track         flex-row (or flex-col) + gap, offset by -scroll px
│       ├── slide     flex_none, slide × cross, rounded card
│       └── …         rendered twice when loop_(true)
├── prev              circular outline button, -left-44px, disabled at 0
└── next              circular outline button, -right-44px, disabled at end`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --carousel

# Other demos:
cargo run --locked -- --carousel sizes
cargo run --locked -- --carousel spacing
cargo run --locked -- --carousel vertical
cargo run --locked -- --carousel loop`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# carousel.rs composes button.rs for the nav controls —
# both ship in source.zip.`;

const scrollNote = `// Scroll model: one animated px offset on the flex track
// (negative margin), eased cubic-out over 320ms like Embla.
// loop_(true) renders the slide list twice; a wrap-around
// animates into the duplicated lap then snaps the offset
// back by one lap — visually continuous, no blank space.
car.go(-1, cx);  // previous slide
car.go(1, cx);   // next slide (clamped or wrapped)`;

export async function CarouselDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/carousel.rs"),
    "utf8",
  );

  return (
    <>
      <main id="main-content" className="detail-main">
        <Breadcrumb className="mb-5">
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink asChild><Link href="/">Components</Link></BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbPage>Carousel</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Carousel</h1>
            <p>A carousel with motion and swipe built using Embla-style semantics.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCarouselWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Carousel 是单个 Rust 文件（<code className="inline-code">src/carousel.rs</code>）—— track/viewport/位移全自绘，导航按钮复用 <code className="inline-code">button.rs</code>。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Carousel</code> 是 entity：<code className="inline-code">cx.new(|cx| Carousel::new(cx))</code> 拿到 focus handle 后配置 slide 数量、每张占比（<code className="inline-code">per_view</code>）、间距与轴向。滚动状态由组件自持，选中变化通过 <code className="inline-code">CarouselSelectEvent</code> 订阅。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Carousel / CarouselContent / CarouselItem / CarouselPrevious / CarouselNext（Embla）。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Scroll &amp; loop.</strong> 位移是 track 上的单个动画 px offset（负 margin），ease-out cubic 320ms。<code className="inline-code">loop_(true)</code> 会把 slide 列表渲染两份：回绕时动画先滚进复制 lap，落地后 offset 按一圈取模归位 —— 视觉上连续无缝。</p>
          <CodeBlock code={scrollNote} label="Example.rs" />
          <p><strong>Keyboard.</strong> 根容器可聚焦（点击视口聚焦）：横排响应 ←/→，竖排（<code className="inline-code">.vertical(true)</code>）响应 ↑/↓，与 Embla 的键盘映射一致。</p>
          <p><strong>Boundaries.</strong> 非 loop 模式首尾按钮禁用（<code className="inline-code">can_prev / can_next</code>）；<code className="inline-code">per_view &gt; 1</code> 时末位自动停在最后完整视口（<code className="inline-code">count - per_view</code>）。</p>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>Carousel 实体与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Carousel::new(cx)</code></TableCell><TableCell><code>-&gt; Carousel</code></TableCell><TableCell>Entity; needs cx for the focus handle.</TableCell></TableRow>
              <TableRow><TableCell><code>.count(..)</code></TableCell><TableCell><code>usize</code></TableCell><TableCell>Slide count (demo slides 1..=n).</TableCell></TableRow>
              <TableRow><TableCell><code>.per_view(..)</code></TableCell><TableCell><code>usize</code></TableCell><TableCell>Slides per viewport — Embla basis-1/n.</TableCell></TableRow>
              <TableRow><TableCell><code>.gap(..)</code></TableCell><TableCell><code>f32</code></TableCell><TableCell>Slide gutter — Embla pl-*.</TableCell></TableRow>
              <TableRow><TableCell><code>.slide_px(..)</code> / <code>.cross_px(..)</code></TableCell><TableCell><code>f32</code></TableCell><TableCell>Slide extent on the scroll axis / cross axis.</TableCell></TableRow>
              <TableRow><TableCell><code>.vertical(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Vertical axis: column track + top/bottom buttons.</TableCell></TableRow>
              <TableRow><TableCell><code>.loop_(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Embla opts.loop — seamless wrap, buttons never disable.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark theme.</TableCell></TableRow>
              <TableRow><TableCell><code>CarouselSelectEvent</code></TableCell><TableCell><code>{"{ index, count }"}</code></TableCell><TableCell>Emitted on every slide change — the current-slide API.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与交互参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/carousel" target="_blank" rel="noreferrer">shadcn/ui Carousel (React Aria / Embla) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。Autoplay/插件体系与 RTL 未移植。</p>
        </section>
        <SiteFooter />
      </main>

      <aside className="resource-rail detail-rail" aria-label="On this page">
        <div className="on-this-page">
          <span>On this page</span>
          <a href="#preview">Overview</a>
          <a href="#installation">Installation</a>
          <a href="#usage">Usage</a>
          <a href="#composition">Composition</a>
          <a href="#notes">Notes</a>
          <a href="#api">API Reference</a>
        </div>
        <div className="component-facts">
          <h2>At a glance</h2>
          <dl>
            <div><dt>Framework</dt><dd>GPUI</dd></div>
            <div><dt>Platform</dt><dd>Desktop</dd></div>
            <div><dt>Examples</dt><dd>5</dd></div>
            <div><dt>Source</dt><dd>Rust</dd></div>
          </dl>
        </div>
        <Link className="guide-callout" href="/guide">
          <span>Getting started</span>
          <strong>Run it natively.<ArrowUpRight /></strong>
          <p>Download the example and build for your desktop.</p>
        </Link>
      </aside>
    </>
  );
}
