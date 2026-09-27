import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAttachmentWorkbench } from "@/components/gpui-attachment-workbench";
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

const usage = `use justdo_command::attachment::{Attachment, AttachmentMedia, AttachmentState};

// Keep the entity on your host view.
let attachment = cx.new(|_| {
    Attachment::new()
        .media(AttachmentMedia::FileText)
        .title("sales-dashboard.pdf")
        .description("PDF · 2.4 MB")
        .state(AttachmentState::Uploading { progress: 0.64 })
});

// In render: div().child(self.attachment.clone())`;

const composition = `Attachment                    card container
├── AttachmentMedia           icon box or image thumbnail
├── AttachmentContent         title + metadata column
│   ├── AttachmentTitle       .title(..)
│   └── AttachmentDescription .description(..)
├── AttachmentActions         remove button -> AttachmentRemoveEvent
└── AttachmentTrigger         .openable(true) -> AttachmentOpenEvent

AttachmentGroup               horizontal scroll row
  .overflow_x_scroll() + .track_scroll(&handle)`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --attachment

# Other demos:
cargo run --locked -- --attachment image
cargo run --locked -- --attachment states
cargo run --locked -- --attachment sizes
cargo run --locked -- --attachment group
cargo run --locked -- --attachment trigger`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# attachment.rs is a single styled component —
# it uses gpui's div / canvas / PathBuilder
# directly, no extra primitives needed.`;

const eventsSnippet = `// Both events carry the attachment title.
cx.subscribe(&self.attachment, |view, _a, event: &AttachmentRemoveEvent, cx| {
    view.files.retain(|f| *f != event.name);
    cx.notify();
});
cx.subscribe(&self.attachment, |view, _a, event: &AttachmentOpenEvent, cx| {
    view.preview = Some(event.name.clone());
    cx.notify();
});`;

export async function AttachmentDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/attachment.rs"),
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
              <BreadcrumbPage>Attachment</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Attachment</h1>
            <p>Displays a file or image attachment with media, metadata, upload state, and actions.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAttachmentWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Attachment 是单个 Rust 文件（<code className="inline-code">src/attachment.rs</code>），用 gpui 的 <code className="inline-code">div</code> / <code className="inline-code">canvas</code> / <code className="inline-code">PathBuilder</code> 自绘，不依赖额外原语。复制到你的 crate，或下载完整示例直接运行。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p>builder 链式配置 media、标题、描述、上传状态与交互；把实体保留在宿主视图上，渲染时放进任意容器，事件用 <code className="inline-code">cx.subscribe</code> 订阅。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Attachment：Media / Content / Actions / Trigger，外加横向滚动的 Group。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <ul className="key-list">
                <li><kbd>Tab</kbd> 沿页面 tab 序聚焦移除按钮，focus-visible 描边</li>
                <li><kbd>Enter</kbd> <kbd>Space</kbd> 触发 AttachmentRemoveEvent</li>
                <li>Group 行用滚轮 / 触控板横向滚动，每张卡仍可通过 Tab 到达</li>
              </ul>
            </div>
            <div>
              <h3>Semantics</h3>
              <p>Error 状态除红色描边外始终带文字失败原因，不单靠颜色；trigger 遮罩避开操作列，移除按钮保持独立可点；processing 状态同步降标题对比度。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>组件实体暴露的方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Attachment::new()</code></TableCell><TableCell><code>-&gt; Attachment</code></TableCell><TableCell>Creates the entity content; state Done, horizontal layout.</TableCell></TableRow>
              <TableRow><TableCell><code>.media(..)</code></TableCell><TableCell><code>AttachmentMedia</code></TableCell><TableCell>FileText | FileCode | Image line icons, or Photo thumbnail.</TableCell></TableRow>
              <TableRow><TableCell><code>.title(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>File name heading.</TableCell></TableRow>
              <TableRow><TableCell><code>.description(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Metadata line: type · size, or a failure reason on error.</TableCell></TableRow>
              <TableRow><TableCell><code>.state(..)</code></TableCell><TableCell><code>AttachmentState</code></TableCell><TableCell>Idle | Uploading {"{ progress }"} | Processing | Error | Done.</TableCell></TableRow>
              <TableRow><TableCell><code>.size(..)</code></TableCell><TableCell><code>AttachmentSize</code></TableCell><TableCell>Default | Sm | Xs — Xs renders media + title only.</TableCell></TableRow>
              <TableRow><TableCell><code>.vertical()</code></TableCell><TableCell><code>-&gt; Self</code></TableCell><TableCell>Stacks media above content; action pinned top-right.</TableCell></TableRow>
              <TableRow><TableCell><code>.removable(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Shows the focusable × action (default true).</TableCell></TableRow>
              <TableRow><TableCell><code>.openable(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Adds a card-wide trigger emitting AttachmentOpenEvent.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark color scheme for surface, text and borders.</TableCell></TableRow>
              <TableRow><TableCell><code>AttachmentRemoveEvent</code></TableCell><TableCell><code>{"{ name }"}</code></TableCell><TableCell>Emitted on × press; subscribe with cx.subscribe.</TableCell></TableRow>
              <TableRow><TableCell><code>AttachmentOpenEvent</code></TableCell><TableCell><code>{"{ name }"}</code></TableCell><TableCell>Emitted on card press when openable.</TableCell></TableRow>
            </TableBody>
          </Table>
          <CodeBlock code={eventsSnippet} label="Example.rs" />
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/attachment" target="_blank" rel="noreferrer">shadcn/ui Attachment <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
          <a href="#accessibility">Accessibility</a>
          <a href="#api">API Reference</a>
        </div>
        <div className="component-facts">
          <h2>At a glance</h2>
          <dl>
            <div><dt>Framework</dt><dd>GPUI</dd></div>
            <div><dt>Platform</dt><dd>Desktop</dd></div>
            <div><dt>Examples</dt><dd>6</dd></div>
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
