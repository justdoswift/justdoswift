import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiBreadcrumbWorkbench } from "@/components/gpui-breadcrumb-workbench";
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

const usage = `use justdo_command::breadcrumb::{Breadcrumb, CrumbItem, CrumbSeparator};

// Breadcrumb is an entity — create it on your view, render as
// a child, subscribe for BreadcrumbNavigateEvent.
let breadcrumb = cx.new(|_| {
    Breadcrumb::new()
        .items(vec![
            CrumbItem::link("Home"),
            CrumbItem::link("Components"),
            CrumbItem::page("Breadcrumb"),
        ])
        .separator(CrumbSeparator::Chevron)
});
div().child(breadcrumb)`;

const composition = `Breadcrumb                entity — renders the trail, emits BreadcrumbNavigateEvent
└── BreadcrumbList        flex row of items + separators
    ├── BreadcrumbItem    CrumbItem::Link(..) — clickable ancestor link
    ├── BreadcrumbSeparator  Chevron | Slash | Dot (mirrored in RTL)
    ├── BreadcrumbItem    CrumbItem::Ellipsis — collapsed "More"
    └── BreadcrumbItem    CrumbItem::Page(..) — current page, plain text`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --breadcrumb

# Other demos:
cargo run --locked -- --breadcrumb separator
cargo run --locked -- --breadcrumb collapsed
cargo run --locked -- --breadcrumb link
cargo run --locked -- --breadcrumb rtl`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# breadcrumb.rs is self-contained — separators are
# painted with PathBuilder, no extra deps.`;

const eventNote = `// Links and the ellipsis emit BreadcrumbNavigateEvent;
// the trailing Page crumb is text only (aria-current="page").
cx.subscribe(&breadcrumb, |demo, _, event, cx| {
    demo.navigate(event.label.clone());
    cx.notify();
});`;

export async function BreadcrumbDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/breadcrumb.rs"),
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
              <BreadcrumbPage>Breadcrumb</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Breadcrumb</h1>
            <p>Displays the path to the current resource using a hierarchy of links.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiBreadcrumbWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Breadcrumb 是单个 Rust 文件（<code className="inline-code">src/breadcrumb.rs</code>），只依赖 gpui-kit —— chevron / slash / dot 分隔符全部 <code className="inline-code">PathBuilder</code> 手绘。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Breadcrumb</code> 是实体（<code className="inline-code">cx.new</code>），<code className="inline-code">CrumbItem</code> 描述每条：Link 可点、Page 是纯文本、Ellipsis 表示折叠的中间层级。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Breadcrumb / BreadcrumbList / Item / Link / Separator / Page / Ellipsis。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={eventNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Hit testing.</strong> 每条 link / ellipsis 是带 <code className="inline-code">.id()</code> 的独立元素 —— hover 变色、Tab 聚焦、focus-visible 描边都由 GPUI 命中测试驱动，不用自己算区域。</p>
          <p><strong>RTL.</strong> <code className="inline-code">.rtl(true)</code> 反转条目顺序并把 chevron 画成左向 —— 分隔符镜像逻辑在 <code className="inline-code">chevron(color, mirrored)</code> 里。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Current page</h3>
              <p>末级 crumb 渲染为纯文本（对应 <code className="inline-code">BreadcrumbPage</code> / <code className="inline-code">aria-current="page"</code>），不进入 Tab 顺序，点击无效果。</p>
            </div>
            <div>
              <h3>Keyboard</h3>
              <p>每条 link 与 ellipsis 可通过 Tab 聚焦，focus-visible 描边反馈；链接 hover 加深，不只依赖颜色表达可交互。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>实体暴露的 builder 方法、条目类型与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Breadcrumb::new()</code></TableCell><TableCell><code>-&gt; Breadcrumb</code></TableCell><TableCell>Default Home / Components / Breadcrumb trail.</TableCell></TableRow>
              <TableRow><TableCell><code>.items(..)</code></TableCell><TableCell><code>Vec&lt;CrumbItem&gt;</code></TableCell><TableCell>Ordered crumbs; last should usually be Page.</TableCell></TableRow>
              <TableRow><TableCell><code>CrumbItem::link(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Clickable ancestor — BreadcrumbLink.</TableCell></TableRow>
              <TableRow><TableCell><code>CrumbItem::page(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Current page text — BreadcrumbPage.</TableCell></TableRow>
              <TableRow><TableCell><code>CrumbItem::Ellipsis</code></TableCell><TableCell><code>-</code></TableCell><TableCell>Collapsed "…" marker — BreadcrumbEllipsis, pressable.</TableCell></TableRow>
              <TableRow><TableCell><code>.separator(..)</code></TableCell><TableCell><code>CrumbSeparator</code></TableCell><TableCell>Chevron | Slash | Dot between crumbs.</TableCell></TableRow>
              <TableRow><TableCell><code>.rtl(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Reverse order + mirrored chevrons for RTL locales.</TableCell></TableRow>
              <TableRow><TableCell><code>BreadcrumbNavigateEvent</code></TableCell><TableCell><code>{`{ label }`}</code></TableCell><TableCell>Emitted by link/ellipsis press; subscribe via cx.subscribe.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/breadcrumb" target="_blank" rel="noreferrer">shadcn/ui Breadcrumb <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
