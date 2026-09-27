import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiBadgeWorkbench } from "@/components/gpui-badge-workbench";
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

const usage = `use justdo_command::badge::{Badge, BadgeVariant, BadgeIcon, BadgeIconPos};

// Badge is a tiny entity — keep it on your view, render it
// as a child, subscribe for BadgePressEvent when clickable.
let badge = cx.new(|_| {
    Badge::new()
        .label("Verified")
        .variant(BadgeVariant::Secondary)
        .icon(BadgeIcon::Check, BadgeIconPos::Start)
});
div().child(badge)`;

const composition = `Badge                     entity — renders the pill, emits BadgePressEvent
├── label                 .label(..) — text-xs medium
├── icon                  .icon(icon, pos) — inline-start | inline-end
│     Check | Bookmark | ArrowUpRight | Spinner (animating arc)
├── variant               Default | Secondary | Destructive | Outline | Ghost | Link
├── tone                  .tone(..) — custom tinted surface + ink
└── clickable             .clickable(true) — focus, click -> BadgePressEvent`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --badge

# Other demos:
cargo run --locked -- --badge variants
cargo run --locked -- --badge icons
cargo run --locked -- --badge spinner
cargo run --locked -- --badge link
cargo run --locked -- --badge custom`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# badge.rs is self-contained — icons and the spinner
# are painted with PathBuilder, no extra deps.`;

const linkNote = `// Clickable badges are real entities: focusable via Tab,
// press via click — the host view subscribes for the event.
cx.subscribe(&badge, |demo, _, event: &BadgePressEvent, cx| {
    demo.open(event.label.clone());
    cx.notify();
});`;

export async function BadgeDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/badge.rs"),
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
              <BreadcrumbPage>Badge</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Badge</h1>
            <p>Displays a badge or a component that looks like a badge.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiBadgeWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Badge 是单个 Rust 文件（<code className="inline-code">src/badge.rs</code>），只依赖 gpui-kit —— 图标与 spinner 全部用 <code className="inline-code">PathBuilder</code> 手绘，无资源文件。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Badge</code> 是实体（<code className="inline-code">cx.new</code>），builder 配置文案、变体、内联图标与可点击性；host view 持有 entity 并渲染为 child。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Badge —— variant / icon slot / inline-start|end / spinner / link。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={linkNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Spinner clock.</strong> WASM 上 <code className="inline-code">std::time::Instant</code> 不可用 —— badge.rs 用 <code className="inline-code">cx.background_executor().now()</code> 做旋转时钟，跨平台安全。只在存在 Spinner 图标时逐帧请求重绘（<code className="inline-code">request_animation_frame</code>），reduced-motion 下静止。</p>
          <p><strong>Entity vs element.</strong> Badge 做成 entity 而非 render-once 元素：link 变体需要持久 <code className="inline-code">FocusHandle</code>（<code className="inline-code">use_keyed_state</code>）与点击事件 —— 纯展示徽章不付额外成本，只有 <code className="inline-code">.clickable(true)</code> 才启用交互。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Contrast</h3>
              <p>每个 variant 与 tone 的 ink/surface 配对在明暗主题下分别取色，保证文字对比度；Link 变体带下划线，不只靠颜色表达可点击。</p>
            </div>
            <div>
              <h3>Focus</h3>
              <p>clickable badge 可通过 Tab 聚焦，focus-visible 描边反馈；纯展示徽章不参与焦点顺序，避免干扰键盘导航。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>实体暴露的 builder 方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Badge::new()</code></TableCell><TableCell><code>-&gt; Badge</code></TableCell><TableCell>Default variant, no icon, label "Badge".</TableCell></TableRow>
              <TableRow><TableCell><code>.label(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>xs / medium label text.</TableCell></TableRow>
              <TableRow><TableCell><code>.variant(..)</code></TableCell><TableCell><code>BadgeVariant</code></TableCell><TableCell>Default | Secondary | Destructive | Outline | Ghost | Link.</TableCell></TableRow>
              <TableRow><TableCell><code>.icon(..)</code></TableCell><TableCell><code>BadgeIcon, BadgeIconPos</code></TableCell><TableCell>Check | Bookmark | ArrowUpRight | Spinner; Start | End.</TableCell></TableRow>
              <TableRow><TableCell><code>.tone(..)</code></TableCell><TableCell><code>BadgeTone</code></TableCell><TableCell>Blue | Green | Sky | Purple | Red tinted colors.</TableCell></TableRow>
              <TableRow><TableCell><code>.clickable(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Focusable + emits BadgePressEvent on press.</TableCell></TableRow>
              <TableRow><TableCell><code>BadgePressEvent</code></TableCell><TableCell><code>{`{ label }`}</code></TableCell><TableCell>Emitted by clickable badges; subscribe via cx.subscribe.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/badge" target="_blank" rel="noreferrer">shadcn/ui Badge <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
          <a href="#accessibility">Accessibility</a>
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
