import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiButtonWorkbench } from "@/components/gpui-button-workbench";
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

const usage = `use justdo_command::button::{Button, ButtonVariant, ButtonSize, ButtonIcon, IconPos};

// Button is a render-once element — build it inline inside your view's
// render(), it is not an entity and needs no cx.new.
div().child(
    Button::new("save")
        .label("Save")
        .icon(ButtonIcon::Plus, IconPos::Start)
        .variant(ButtonVariant::Default)
        .on_click(|_, _, cx| { /* activate */ }),
)`;

const composition = `Button                    element — gpui_base::Button owns focus/keys/disabled
├── label                 .label(..) — sm/xs medium, nowrap
├── icon                  .icon(icon, pos) — inline-start | inline-end
│     ArrowUp | ArrowUpRight | GitBranch | Plus | Spinner (animating arc)
├── variant               Default | Secondary | Destructive | Outline | Ghost | Link
├── size                  Xs | Sm | Default | Lg | IconXs | IconSm | Icon | IconLg
├── pill                  .pill(true) — rounded-full
├── join                  .join(Start|Middle|End) — ButtonGroup corners
├── disabled              .disabled(true) — inert, dimmed
└── on_click              pointer + Enter + Space activation`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --button

# Other demos:
cargo run --locked -- --button variants
cargo run --locked -- --button sizes
cargo run --locked -- --button icons
cargo run --locked -- --button rounded
cargo run --locked -- --button spinner
cargo run --locked -- --button group`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# button.rs builds on gpui_base::Button — focus, keyboard
# activation and disabled behavior come from the primitive.`;

const subscribeNote = `// on_click fires for pointer AND Enter/Space — the base
// primitive synthesizes a keyboard ClickEvent, so one handler
// covers both input paths.
Button::new("save")
    .label("Save")
    .on_click(|_, _, cx| save(cx));`;

export async function ButtonDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/button.rs"),
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
              <BreadcrumbPage>Button</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Button</h1>
            <p>Displays a button or a component that looks like a button.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiButtonWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Button 是单个 Rust 文件（<code className="inline-code">src/button.rs</code>），样式层自绘，交互语义交给 <code className="inline-code">gpui_base::Button</code> 原语 —— focus、Tab 顺序、Enter/Space 激活、disabled 惰性全部免费。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Button</code> 是 render-once 元素（<code className="inline-code">IntoElement</code>），在 <code className="inline-code">render()</code> 里内联构建即可，不需要实体。一个 <code className="inline-code">on_click</code> 同时覆盖鼠标与键盘。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Button —— variant / size / icon slot / pill / button group / spinner。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={subscribeNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Keyboard for free.</strong> <code className="inline-code">gpui_base::Button</code> 把 Enter/Space 合成 <code className="inline-code">ClickEvent::Keyboard</code> 派发进同一个 <code className="inline-code">on_click</code> —— 不需要手写 key handler，也不用担心重复触发。</p>
          <p><strong>Spinner clock.</strong> WASM 上 <code className="inline-code">std::time::Instant</code> 不可用 —— 起始时刻存在 <code className="inline-code">use_keyed_state</code> 里（key 取自 a11y label，因为按钮 id 已被底层原语的 FocusHandle 占用），渲染期用 <code className="inline-code">cx.background_executor().now()</code> 相减；reduced-motion 下静止且不再请求逐帧。</p>
          <p><strong>Icon-only.</strong> <code className="inline-code">Icon*</code> 尺寸没有可见文本，用 <code className="inline-code">.a11y_label(..)</code> 给无障碍树命名。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Role &amp; focus</h3>
              <p>底层原语标记 <code className="inline-code">role="button"</code>、管理 Tab 顺序与 focus-visible 描边；disabled 按钮既不可聚焦也不响应指针，父级点击也不会被穿透。</p>
            </div>
            <div>
              <h3>Contrast</h3>
              <p>六种变体在明暗主题下分别取色；Link 变体带下划线，不只靠颜色表达可点击；icon-only 按钮必须提供 <code className="inline-code">a11y_label</code>。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>元素暴露的 builder 方法。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Button::new(id)</code></TableCell><TableCell><code>-&gt; Button</code></TableCell><TableCell>Element id — used for focus state.</TableCell></TableRow>
              <TableRow><TableCell><code>.label(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Visible label; also the default a11y name.</TableCell></TableRow>
              <TableRow><TableCell><code>.a11y_label(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Required for icon-only buttons.</TableCell></TableRow>
              <TableRow><TableCell><code>.variant(..)</code></TableCell><TableCell><code>ButtonVariant</code></TableCell><TableCell>Default | Secondary | Destructive | Outline | Ghost | Link.</TableCell></TableRow>
              <TableRow><TableCell><code>.size(..)</code></TableCell><TableCell><code>ButtonSize</code></TableCell><TableCell>Xs | Sm | Default | Lg | IconXs | IconSm | Icon | IconLg.</TableCell></TableRow>
              <TableRow><TableCell><code>.icon(..)</code></TableCell><TableCell><code>ButtonIcon, IconPos</code></TableCell><TableCell>ArrowUp | ArrowUpRight | GitBranch | Plus | Spinner.</TableCell></TableRow>
              <TableRow><TableCell><code>.pill(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>rounded-full.</TableCell></TableRow>
              <TableRow><TableCell><code>.join(..)</code></TableCell><TableCell><code>ButtonJoin</code></TableCell><TableCell>Solo | Start | Middle | End — joined group corners.</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Inert: no pointer/keyboard activation, dimmed.</TableCell></TableRow>
              <TableRow><TableCell><code>.on_click(..)</code></TableCell><TableCell><code>Fn(&amp;ClickEvent, ..)</code></TableCell><TableCell>Pointer, Enter, and Space share one handler.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/button" target="_blank" rel="noreferrer">shadcn/ui Button <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
