import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAspectRatioWorkbench } from "@/components/gpui-aspect-ratio-workbench";
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

const usage = `use justdo_command::aspect_ratio::AspectRatio;

// Width comes from the parent; height = width / ratio.
div().w(px(460.)).child(
    AspectRatio::new(16. / 9.)
        .rounded_lg()
        .overflow_hidden()
        .child(img("photo.png"))
)`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --aspect-ratio

# Other demos:
cargo run --locked -- --aspect-ratio square
cargo run --locked -- --aspect-ratio portrait`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# aspect_ratio.rs is a thin styled wrapper over
# gpui's layout-level aspect_ratio, no primitives
# needed beyond div + canvas.`;

export async function AspectRatioDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/aspect_ratio.rs"),
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
              <BreadcrumbPage>Aspect Ratio</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Aspect Ratio</h1>
            <p>Displays content within a desired ratio.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAspectRatioWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Aspect Ratio 是单个 Rust 文件（<code className="inline-code">src/aspect_ratio.rs</code>），底层就是 GPUI 布局系统的 <code className="inline-code">aspect_ratio</code> 样式。复制到你的 crate，或下载完整示例直接运行。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p>容器取父级全宽，高度按 <code className="inline-code">ratio</code>（宽 ÷ 高）推导。媒体内容作为 child 放进去，配合 <code className="inline-code">overflow_hidden</code> 裁圆角。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件；演示中的风景占位图由 canvas + PathBuilder 绘制。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Notes</h2>
          <div className="implementation-notes">
            <div>
              <h3>Sizing</h3>
              <p>比例约束作用在「宽已知、高推导」的方向上——外层只给定宽度。放进固定高度容器时请改给子元素定宽，或调整 <code className="inline-code">w_full</code> 为具体宽度。</p>
            </div>
            <div>
              <h3>Media</h3>
              <p>内部媒体元素建议 <code className="inline-code">size_full</code> 撑满比例框；canvas 没有固有尺寸，必须显式充满父级才能拿到真实的绘制 bounds。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>组件是无状态的布局元素，样式方法（<code className="inline-code">rounded_lg</code>、<code className="inline-code">overflow_hidden</code> 等）直接由 <code className="inline-code">Styled</code> 提供。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>AspectRatio::new(..)</code></TableCell><TableCell><code>ratio: f32</code></TableCell><TableCell>Width ÷ height, e.g. 16./9., 1., 9./16.</TableCell></TableRow>
              <TableRow><TableCell><code>.child(..)</code></TableCell><TableCell><code>impl IntoElement</code></TableCell><TableCell>Content rendered inside the ratio box.</TableCell></TableRow>
              <TableRow><TableCell><code>Styled methods</code></TableCell><TableCell><code>rounded_*, overflow_*</code></TableCell><TableCell>Standard tailwind-style modifiers apply directly.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/aspect-ratio" target="_blank" rel="noreferrer">shadcn/ui Aspect Ratio <ArrowUpRight size={12} /></a>。本页预览由 Rust / GPUI 绘制，比例布局来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui <ArrowUpRight size={12} /></a> 的 taffy 引擎。</p>
        </section>
        <SiteFooter />
      </main>

      <aside className="resource-rail detail-rail" aria-label="On this page">
        <div className="on-this-page">
          <span>On this page</span>
          <a href="#preview">Overview</a>
          <a href="#installation">Installation</a>
          <a href="#usage">Usage</a>
          <a href="#composition">Notes</a>
          <a href="#api">API Reference</a>
        </div>
        <div className="component-facts">
          <h2>At a glance</h2>
          <dl>
            <div><dt>Framework</dt><dd>GPUI</dd></div>
            <div><dt>Platform</dt><dd>Desktop</dd></div>
            <div><dt>Examples</dt><dd>3</dd></div>
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
