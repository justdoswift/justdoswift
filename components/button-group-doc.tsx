import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiButtonGroupWorkbench } from "@/components/gpui-button-group-workbench";
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

const usage = `use justdo_command::button_group::{ButtonGroup, group_separator, group_text};
use justdo_command::button::{Button, ButtonJoin, ButtonVariant};

// ButtonGroup is a render-once container — children opt into the merged
// look with .join(Start|Middle|End) for horizontal and .vjoin(..) for
// vertical orientation.
ButtonGroup::new()
    .gap(px(0.))
    .child(
        Button::new("archive").label("Archive")
            .variant(ButtonVariant::Outline)
            .join(ButtonJoin::Start),
    )
    .child(
        Button::new("report").label("Report")
            .variant(ButtonVariant::Outline)
            .join(ButtonJoin::End),
    )`;

const composition = `ButtonGroup               container — flex row/col, role="group" semantics
├── Button children       .join(Start|Middle|End) merges borders + corners
│     .vjoin(..)          same treatment stacked top-to-bottom
├── group_separator()     1px divider — needed between same-surface buttons
├── group_text()          non-interactive label cell dressed like a button
└── nested ButtonGroup    groups inside groups keep the parent's gap`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --button-group

# Other demos:
cargo run --locked -- --button-group orientation
cargo run --locked -- --button-group sizes
cargo run --locked -- --button-group nested
cargo run --locked -- --button-group separator
cargo run --locked -- --button-group split
cargo run --locked -- --button-group input`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# button_group.rs composes button.rs — joined corners and merged
# borders live on Button via .join() / .vjoin().`;

const joinNote = `// join() controls which corners keep their radius; Middle/End
// drop the leading border (border-l-0) so the previous button's
// trailing border is the shared seam — same trick as shadcn's CSS.
Button::new("first").join(ButtonJoin::Start)   // left corners only
Button::new("mid").join(ButtonJoin::Middle)    // square, no left border
Button::new("last").join(ButtonJoin::End)      // right corners only`;

export async function ButtonGroupDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/button_group.rs"),
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
              <BreadcrumbPage>Button Group</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Button Group</h1>
            <p>A container that groups related buttons together with consistent styling.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiButtonGroupWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Button Group 是单个 Rust 文件（<code className="inline-code">src/button_group.rs</code>），容器、分隔线与文本格自绘，按钮复用 <code className="inline-code">button.rs</code> —— 合并圆角与描边通过 <code className="inline-code">.join()</code> / <code className="inline-code">.vjoin()</code> 声明。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">ButtonGroup</code> 是 render-once 容器，直接 <code className="inline-code">.child(..)</code> 装按钮。参与合并的子项用 <code className="inline-code">.join(Start|Middle|End)</code>（横向）或 <code className="inline-code">.vjoin(..)</code>（纵向）标记自己在组内的位置。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 ButtonGroup / ButtonGroupSeparator / ButtonGroupText。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={joinNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Join mechanics.</strong> 与 shadcn 的 CSS 方案一致：<code className="inline-code">.join(Middle)</code> / <code className="inline-code">.join(End)</code> 的按钮去掉左边框（<code className="inline-code">border-l-0</code>）并抹平左圆角，前一枚按钮的右边框即共享接缝 —— 不用负边距，也不会遮住分隔线。<code className="inline-code">.vjoin(..)</code> 在纵向组里对应去上边框 + 上下圆角取舍。</p>
          <p><strong>When to use a separator.</strong> Outline 按钮自带描边，合并后自然可见分界；Default/Secondary 等同色按钮之间要显式插 <code className="inline-code">group_separator(false, dark)</code>（1px、<code className="inline-code">self-stretch</code> 全高），split button 同理。</p>
          <p><strong>Group vs Toggle Group.</strong> Button Group 放的是相关动作；如果需要"选中态保持"（多选/单选切换），应该用 Toggle Group —— 语义不同。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Role &amp; focus</h3>
              <p>组容器对应 <code className="inline-code">role="group"</code> 语义；组内每个按钮仍是独立的 <code className="inline-code">gpui_base::Button</code> —— Tab 逐个聚焦、Enter/Space 激活、focus-visible 描边都保留。</p>
            </div>
            <div>
              <h3>Labeling</h3>
              <p>icon-only 成员（翻页、split 触发器）必须给 <code className="inline-code">.a11y_label(..)</code>；组本身在宿主侧应有一个可读的语境标签。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>容器与辅助元素的 API。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>ButtonGroup::new()</code></TableCell><TableCell><code>-&gt; ButtonGroup</code></TableCell><TableCell>Render-once flex container.</TableCell></TableRow>
              <TableRow><TableCell><code>.vertical(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Stack children top-to-bottom.</TableCell></TableRow>
              <TableRow><TableCell><code>.gap(..)</code></TableCell><TableCell><code>impl Into&lt;Pixels&gt;</code></TableCell><TableCell>Spacing between non-joined children / nested groups.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Theme for separator/text cells.</TableCell></TableRow>
              <TableRow><TableCell><code>group_separator(..)</code></TableCell><TableCell><code>(vertical, dark)</code></TableCell><TableCell>1px divider between same-surface buttons.</TableCell></TableRow>
              <TableRow><TableCell><code>group_text(..)</code></TableCell><TableCell><code>(text, dark)</code></TableCell><TableCell>Non-interactive label cell styled like a button.</TableCell></TableRow>
              <TableRow><TableCell><code>Button::join(..)</code></TableCell><TableCell><code>ButtonJoin</code></TableCell><TableCell>Solo | Start | Middle | End — horizontal merge.</TableCell></TableRow>
              <TableRow><TableCell><code>Button::vjoin(..)</code></TableCell><TableCell><code>ButtonJoin</code></TableCell><TableCell>Same, for vertical groups.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/button-group" target="_blank" rel="noreferrer">shadcn/ui Button Group (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
            <div><dt>Examples</dt><dd>7</dd></div>
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
