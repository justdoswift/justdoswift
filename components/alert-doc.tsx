import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAlertWorkbench } from "@/components/gpui-alert-workbench";
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

const usage = `use justdo_command::alert::{Alert, AlertIcon};

// Keep the entity on your host view.
let alert = cx.new(|_| {
    Alert::new()
        .icon(AlertIcon::Check)
        .title("Payment successful")
        .description("Your payment of $29.99 has been processed.")
});

// In render: div().child(self.alert.clone())`;

const composition = `Alert                      status callout
├── Icon                   info · check · error · warning
├── AlertTitle             .title(..)
├── AlertDescription       .description(..)
└── AlertAction            .action(..) -> AlertActionEvent`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --alert

# Other demos:
cargo run --locked -- --alert destructive
cargo run --locked -- --alert action
cargo run --locked -- --alert custom`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# alert.rs is a single styled component —
# it uses gpui's div / canvas / PathBuilder
# directly, no extra primitives needed.`;

const eventsSnippet = `// AlertActionEvent { label } is emitted when the action is pressed.
cx.subscribe(&self.alert, |view, _alert, event, cx| {
    println!("pressed: {}", event.label);
    cx.notify();
});`;

export async function AlertDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/alert.rs"),
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
              <BreadcrumbPage>Alert</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Alert</h1>
            <p>Displays a callout for user attention.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAlertWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Alert 是单个 Rust 文件（<code className="inline-code">src/alert.rs</code>），直接用 gpui 的元素与 <code className="inline-code">PathBuilder</code> 图标绘制。复制到你的 crate，或下载完整示例直接运行。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p>builder 链式配置图标、标题、描述与操作按钮；把实体保留在宿主视图上，渲染时放进任意容器。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Alert：容器 + 图标 + 标题 + 描述 + 可选操作。操作按钮独立可聚焦。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <ul className="key-list">
                <li><kbd>Tab</kbd> 沿页面 tab 序聚焦操作按钮</li>
                <li><kbd>Enter</kbd> <kbd>Space</kbd> 触发 AlertActionEvent</li>
              </ul>
            </div>
            <div>
              <h3>Semantics</h3>
              <p>Alert 是静态状态提示（对应 role=&#34;alert&#34; 的展示语义），action 变体的按钮可聚焦、带 focus-visible 高亮。reduced-motion 下入场动画自动关闭。</p>
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
              <TableRow><TableCell><code>Alert::new()</code></TableCell><TableCell><code>-&gt; Alert</code></TableCell><TableCell>Creates the entity content. Default variant, no icon.</TableCell></TableRow>
              <TableRow><TableCell><code>.variant(..)</code></TableCell><TableCell><code>AlertVariant</code></TableCell><TableCell>Default | Destructive — flips surface, border and ink colors.</TableCell></TableRow>
              <TableRow><TableCell><code>.icon(..)</code></TableCell><TableCell><code>AlertIcon</code></TableCell><TableCell>Info | Check | Error | Warning, drawn with PathBuilder.</TableCell></TableRow>
              <TableRow><TableCell><code>.title(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Bold heading line inside the callout.</TableCell></TableRow>
              <TableRow><TableCell><code>.description(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Secondary copy under the title.</TableCell></TableRow>
              <TableRow><TableCell><code>.action(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Right-side button; pressing it emits AlertActionEvent.</TableCell></TableRow>
              <TableRow><TableCell><code>.surface/.edge/.tint(..)</code></TableCell><TableCell><code>impl Into&lt;Hsla&gt;</code></TableCell><TableCell>Per-alert color overrides for custom-color variants.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark color scheme for surfaces, text and borders.</TableCell></TableRow>
              <TableRow><TableCell><code>AlertActionEvent</code></TableCell><TableCell><code>{"{ label }"}</code></TableCell><TableCell>Emitted on action press; subscribe with cx.subscribe.</TableCell></TableRow>
            </TableBody>
          </Table>
          <CodeBlock code={eventsSnippet} label="Example.rs" />
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/alert" target="_blank" rel="noreferrer">shadcn/ui Alert <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
            <div><dt>Examples</dt><dd>4</dd></div>
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
