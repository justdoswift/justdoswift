import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAlertDialogWorkbench } from "@/components/gpui-alert-dialog-workbench";
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

const usage = `use justdo_command::alert_dialog::{AlertDialogSpec, alert_dialog};
use justdo_command::button::Button;
use gpui_kit::base::{AlertDialogTrigger, DialogHandle};

// Keep the handle on your host view.
struct Host { handle: DialogHandle }

// In render: the trigger opens the dialog; the host renders it.
// (any IntoElement works as the trigger — our Button is one)
AlertDialogTrigger::new(Button::new("open").label("Show Dialog"))
    .handle(self.handle.clone())

// Plus the composed dialog itself (backdrop + popup + parts):
alert_dialog(&spec, &self.handle, intro, |confirmed, _, _| {
    // true = confirm pressed, false = cancel / Escape
}, cx)`;

const composition = `AlertDialog                modal host · focus trap
├── AlertDialogTrigger     opens on press
└── AlertDialogPopup       centered card
    ├── AlertDialogMedia   optional icon box
    ├── AlertDialogTitle
    ├── AlertDialogDescription
    └── Footer
        ├── AlertDialogCancel   -> Cancel action
        └── AlertDialogAction   -> Confirm action`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --alert-dialog

# Other demos:
cargo run --locked -- --alert-dialog sm
cargo run --locked -- --alert-dialog media
cargo run --locked -- --alert-dialog sm-media
cargo run --locked -- --alert-dialog destructive`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# alert_dialog.rs builds on the modal primitives
# re-exported at gpui_kit::base:
#   AlertDialog, AlertDialogTrigger, AlertDialogBackdrop,
#   AlertDialogPopup, AlertDialogTitle, AlertDialogDescription,
#   AlertDialogCancel, AlertDialogAction, DialogHandle`;

const eventsSnippet = `// on_ok / on_cancel run when Action / Cancel / Enter / Escape resolve
// the dialog; returning true lets it close.
AlertDialog::new(cx)
    .handle(self.handle.clone())
    .on_ok(|_, _, _| true)
    .on_cancel(|_, _, _| true)
    .on_open_change(|open, reason, _, _| { /* reopen, backdrop, ... */ })`;

export async function AlertDialogDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/alert_dialog.rs"),
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
              <BreadcrumbPage>Alert Dialog</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Alert Dialog</h1>
            <p>A modal dialog that interrupts the user with important content and expects a response.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAlertDialogWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Alert Dialog 是单个 Rust 文件（<code className="inline-code">src/alert_dialog.rs</code>），构建在 gpui-base 的模态原语之上。复制到你的 crate，或下载完整示例直接运行。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p>宿主视图持有 <code className="inline-code">DialogHandle</code>，用 <code className="inline-code">AlertDialogSpec</code> 描述内容，渲染时触发器与对话框本体都通过 handle 关联。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件；点按 Show Dialog 打开，遮罩不可点击关闭。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构与 shadcn/ui 的 Base UI 版本一一对应。模态行为、焦点围栏与遮罩由 gpui-base 原语提供，样式由本模块统一提供。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <ul className="key-list">
                <li><kbd>Tab</kbd> 聚焦触发器，<kbd>Enter</kbd> <kbd>Space</kbd> 打开对话框</li>
                <li><kbd>Escape</kbd> 触发 Cancel 关闭对话框</li>
                <li><kbd>Enter</kbd> 在弹层内触发 Confirm 关闭</li>
                <li>打开时焦点移入弹层并被围栏，关闭后还给触发器；遮罩按下不会关闭</li>
              </ul>
            </div>
            <div>
              <h3>Semantics</h3>
              <p>弹层携带 <code className="inline-code">role=&#34;alertdialog&#34;</code>；Cancel 派发 Cancel、Action 派发 Confirm action。打开时背景淡出 + 卡片上浮入场，reduced-motion 下动画自动关闭。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>本模块暴露的配置与组合函数。模态原语由 <code className="inline-code">gpui_kit::base</code> 提供。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>AlertDialogSpec::new()</code></TableCell><TableCell><code>-&gt; AlertDialogSpec</code></TableCell><TableCell>Default size, no media, Cancel/Continue labels.</TableCell></TableRow>
              <TableRow><TableCell><code>.trigger/.title/.description(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Trigger label and popup copy.</TableCell></TableRow>
              <TableRow><TableCell><code>.cancel/.confirm(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Footer button labels.</TableCell></TableRow>
              <TableRow><TableCell><code>.size(..)</code></TableCell><TableCell><code>AlertDialogSize</code></TableCell><TableCell>Default | Sm — Sm narrows the card and stacks the footer.</TableCell></TableRow>
              <TableRow><TableCell><code>.media(..)</code></TableCell><TableCell><code>AlertDialogMedia</code></TableCell><TableCell>Plus | Bluetooth | Trash icon box above the title.</TableCell></TableRow>
              <TableRow><TableCell><code>.destructive(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Red confirm button and tinted media icon.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark color scheme for scrim, card and buttons.</TableCell></TableRow>
              <TableRow><TableCell><code>alert_dialog(..)</code></TableCell><TableCell><code>spec, handle, intro, on_done, cx</code></TableCell><TableCell>Composes backdrop + popup; on_done receives confirmed flag.</TableCell></TableRow>
              <TableRow><TableCell><code>DialogHandle</code></TableCell><TableCell><code>::new(open)</code></TableCell><TableCell>Open state shared by trigger, host and imperative close.</TableCell></TableRow>
            </TableBody>
          </Table>
          <CodeBlock code={eventsSnippet} label="Example.rs" />
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/alert-dialog" target="_blank" rel="noreferrer">shadcn/ui Alert Dialog <ArrowUpRight size={12} /></a>（Base UI 组合方式）。本页交互由 Rust / GPUI 绘制，模态原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-base <ArrowUpRight size={12} /></a>。</p>
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
