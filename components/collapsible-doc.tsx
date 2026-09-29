import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCollapsibleWorkbench } from "@/components/gpui-collapsible-workbench";
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

const usage = `use justdo_command::collapsible::{Collapsible, CollapsibleChangeEvent};

// One entity owns the open state and the reveal animation.
let details = cx.new(|_| {
    Collapsible::new()
        .trigger("Product details")
        .content(|| {
            div()
                .child("This panel can be expanded or collapsed.")
                .into_any_element()
        })
});

cx.subscribe(&details, |view, _, ev: &CollapsibleChangeEvent, cx| {
    view.details_open = ev.open;
    cx.notify();
});

div().child(details)`;

const composition = `Collapsible               entity — trigger row + reveal panel
├── trigger               .trigger(..) — click row / Enter / Space toggles
├── hint                  .hint(..) — quiet text right of the label
├── chevron               ChevronStyle::Down (trailing, flips 180°)
│                         ChevronStyle::Right (leading, rotates 90°)
├── icon                  RowIcon::Folder | File — tree-row glyph
├── content               .content(|| element) — CollapsibleContent panel
│                         revealed with a MotionReveal height wipe
├── disabled              .disabled(true) — dimmed + inert
└── emits                 CollapsibleChangeEvent { open }`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --collapsible

# Other demos:
cargo run --locked -- --collapsible open
cargo run --locked -- --collapsible settings
cargo run --locked -- --collapsible file-tree`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# collapsible.rs reuses gpui-base MotionReveal for the
# height animation — no extra deps.`;

const controlledNote = `// Controlled state — mirror of isExpanded + onExpandedChange.
// The entity stores open state; drive it with set_open and
// read the change through CollapsibleChangeEvent.
collapsible.update(cx, |c, cx| c.set_open(true, cx));

cx.subscribe(&collapsible, |view, _, ev, cx| {
    view.open = ev.open;
    cx.notify();
});`;

const treeNote = `// Nested collapsibles build the file tree: each folder is an
// entity whose content factory returns child rows — nested
// folder entities clone cheaply (Entity<T> is a handle).
let ui = folder(cx, "ui", /* open */ true, move || {
    files.iter().map(|f| file_row(f)).collect()
});

let components = folder(cx, "components", true, move || {
    vec![div().child(ui.clone()).into_any_element(), /* … */]
});`;

export async function CollapsibleDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/collapsible.rs"),
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
              <BreadcrumbPage>Collapsible</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Collapsible</h1>
            <p>An interactive component which expands/collapses a panel.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCollapsibleWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Collapsible 是单个 Rust 文件（<code className="inline-code">src/collapsible.rs</code>）—— 展开动画复用 gpui-base 的 <code className="inline-code">MotionReveal</code>，chevron/文件夹图标用 PathBuilder 手绘。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Collapsible</code> 是实体：<code className="inline-code">.trigger(..)</code> 渲染标题行，<code className="inline-code">.content(..)</code> 接收一个<strong>工厂闭包</strong>（元素每帧重建，所以传构造器而不是元素本体）。点击或 Enter/Space 切换，发出 <code className="inline-code">CollapsibleChangeEvent</code>。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对应 <code className="inline-code">Collapsible → Trigger + Content</code>：trigger 是单行实体按钮，content 面板经 MotionReveal 做高度揭幕动画。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={controlledNote} label="Controlled.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>内容工厂。</strong> GPUI 元素是 render-once —— <code className="inline-code">.content(..)</code> 收 <code className="inline-code">Fn() -&gt; AnyElement</code>，每次渲染重建面板子树。嵌套 Collapsible（文件树）直接把孩子 entity 放进闭包里 clone。</p>
          <p><strong>展开动画。</strong> <code className="inline-code">transition</code> keyed-state 驱动 0↔1 进度，MotionReveal 负责高度擦除；收起时保持挂载直到动画播完。<code className="inline-code">cx.reduce_motion()</code> 下直接跳终态。</p>
          <CodeBlock code={treeNote} label="File tree" />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <p>Trigger 行进入 Tab 顺序，Enter / Space 切换；chevron 方向即状态提示（右指=收起、下指=展开），不只靠颜色。</p>
            </div>
            <div>
              <h3>Focus</h3>
              <p>点击聚焦后可用键盘操作；disabled 行半透明且完全不响应输入或聚焦。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>实体暴露的 builder 方法、受控方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Collapsible::new()</code></TableCell><TableCell><code>-&gt; Collapsible</code></TableCell><TableCell>Collapsed, chevron-down trigger.</TableCell></TableRow>
              <TableRow><TableCell><code>.trigger(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Trigger label; row click / Enter / Space toggles.</TableCell></TableRow>
              <TableRow><TableCell><code>.hint(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Quiet secondary text next to the label.</TableCell></TableRow>
              <TableRow><TableCell><code>.content(..)</code></TableCell><TableCell><code>Fn() -&gt; AnyElement</code></TableCell><TableCell>Panel factory — rebuilt each render (CollapsibleContent).</TableCell></TableRow>
              <TableRow><TableCell><code>.open(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Initial state (defaultExpanded).</TableCell></TableRow>
              <TableRow><TableCell><code>.chevron(..)</code></TableCell><TableCell><code>ChevronStyle</code></TableCell><TableCell>Down (trailing, flips) | Right (leading, rotates 90°).</TableCell></TableRow>
              <TableRow><TableCell><code>.icon(..)</code></TableCell><TableCell><code>RowIcon</code></TableCell><TableCell>Folder | File glyph before the label (tree rows).</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dimmed and inert.</TableCell></TableRow>
              <TableRow><TableCell><code>.set_open(..)</code></TableCell><TableCell><code>bool, &amp;mut Context</code></TableCell><TableCell>Controlled update (isExpanded / onExpandedChange).</TableCell></TableRow>
              <TableRow><TableCell><code>CollapsibleChangeEvent</code></TableCell><TableCell><code>{`{ open: bool }`}</code></TableCell><TableCell>Emitted on every expand/collapse.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/collapsible" target="_blank" rel="noreferrer">shadcn/ui Collapsible <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
