import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiComboboxWorkbench } from "@/components/gpui-combobox-workbench";
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

const usage = `use justdo_command::combobox::{Combobox, ComboboxItem, ComboboxChangeEvent};
use gpui_kit::base::input::InputState;

// The input entity is created with a Window, then the
// combobox entity wraps it and owns selection + filtering.
let input = cx.new(|cx| {
    InputState::new(window, cx).placeholder("Select framework…")
});

let combobox = cx.new(|cx| {
    Combobox::new(input, cx)
        .items(vec![
            ComboboxItem::new("Next.js"),
            ComboboxItem::new("SvelteKit"),
            ComboboxItem::new("Astro").disabled(true),
        ])
        .empty_text("No framework found.")
});

cx.subscribe(&combobox, |view, _, ev: &ComboboxChangeEvent, cx| {
    view.selection = ev.selected.clone();
    cx.notify();
});

div().child(combobox)`;

const composition = `Combobox                  entity — input + popup + selection
├── input                 Entity<InputState> — real text editing (gpui-base)
├── field                 InputBase — chips / ✕ clear / chevron trigger row
├── popup                 filtered rows — active highlight, ✓ on selected
│   ├── group headers     ComboboxItem::group(..)
│   ├── hint              ComboboxItem::hint(..) — trailing quiet text
│   └── empty             .empty_text(..) — ComboboxEmpty
├── single                .default_value(..) — label refills the input
├── multiple              .multiple(true) — chips + Backspace pops last
└── emits                 ComboboxChangeEvent { value, selected }`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --combobox

# Other demos:
cargo run --locked -- --combobox multiple
cargo run --locked -- --combobox groups
cargo run --locked -- --combobox disabled`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# combobox.rs builds on gpui-base primitives:
# InputState (editing) + Combobox root (role/ARIA
# + popup semantics) — no extra deps.`;

const keyboardNote = `// Keyboard semantics ride GPUI's action system, not raw
// key capture. A focused single-line input registers no
// up/down handlers, so input::MoveUp / input::MoveDown
// bubble out — the combobox listens for them and drives
// the active row. Enter propagates as input::Enter,
// Escape as input::Escape; both commit/close at the root.

// One exception needs to run *before* bindings: Backspace
// on an empty multi-select input is consumed by the editor,
// so a keystroke interceptor pops the newest chip.`;

const controlledNote = `// Controlled selection — read it or drive it.
combobox.update(cx, |c, cx| c.set_selected(&["Next.js"], cx));
combobox.update_in(cx, |c, window, cx| {
    c.set_value(Some("Astro"), window, cx);   // single select
    c.set_open(true, cx);
});

let current: Option<SharedString> = combobox.read(cx).selected_value();`;

export async function ComboboxDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/combobox.rs"),
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
              <BreadcrumbPage>Combobox</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Combobox</h1>
            <p>Autocomplete input and command palette with a list of suggestions.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiComboboxWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Combobox 是单个 Rust 文件（<code className="inline-code">src/combobox.rs</code>）—— 文本编辑复用 gpui-base 的 <code className="inline-code">InputState</code>，弹出层语义（role/ARIA/焦点移交）来自 <code className="inline-code">gpui-base::Combobox</code> 原语，勾选与 chevron 用 PathBuilder 手绘。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Combobox</code> 是实体：输入框是一个真实的 <code className="inline-code">InputState</code>（先 new 出来再传进构造器），实体自持过滤列表、active 行和选中值。输入即过滤（label/value/group 子串匹配），Enter 提交高亮行，选项回填 label 后再次输入即可重新搜索。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对应 <code className="inline-code">Combobox → Input + Listbox</code>：field 行承载 chips/清空/开合，popup 列表渲染分组、勾选与禁用态。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={controlledNote} label="Controlled.rs" />
        </section>

        <section className="doc-section" id="keyboard">
          <h2>Keyboard</h2>
          <p>单行输入框不注册上下方向 handler —— 方向键以 <code className="inline-code">input::MoveDown/MoveUp</code> action 冒泡出来，组件在外层接住并驱动高亮行；Enter 走 <code className="inline-code">input::Enter</code>，Escape 走 <code className="inline-code">input::Escape</code>。多选下「空查询按 Backspace 删最后一个 chip」发生在绑定派发之前，由 keystroke interceptor 完成。</p>
          <CodeBlock code={keyboardNote} label="Key handling.rs" />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Semantics</h3>
              <p>根元素带 <code className="inline-code">role=combobox</code> + <code className="inline-code">aria-expanded</code>；禁用项与禁用组件不响应指针与键盘；invalid 用红边 + 焦点环双重标识，不只靠颜色。</p>
            </div>
            <div>
              <h3>Keyboard</h3>
              <p>输入保持焦点：↓/↑ 移动高亮、Enter 提交、Escape 还原并关闭、Backspace 空查询时删 chip；列表超长时可滚动且高亮行自动滚入视野。</p>
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
              <TableRow><TableCell><code>Combobox::new(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;</code></TableCell><TableCell>Wraps a caller-created input entity.</TableCell></TableRow>
              <TableRow><TableCell><code>.items(..)</code></TableCell><TableCell><code>Vec&lt;ComboboxItem&gt;</code></TableCell><TableCell>The collection; filtered as you type.</TableCell></TableRow>
              <TableRow><TableCell><code>ComboboxItem::new(..)</code></TableCell><TableCell><code>&amp;str label</code></TableCell><TableCell><code>.value(..)</code> <code>.group(..)</code> <code>.hint(..)</code> <code>.disabled(..)</code></TableCell></TableRow>
              <TableRow><TableCell><code>.multiple(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Chip selection; Enter/click toggles.</TableCell></TableRow>
              <TableRow><TableCell><code>.default_value(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Initial single selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.selected_values(..)</code></TableCell><TableCell><code>&amp;[&amp;str]</code></TableCell><TableCell>Initial multi selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.clearable(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>✕ button when a value is present.</TableCell></TableRow>
              <TableRow><TableCell><code>.empty_text(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Shown when the query matches nothing.</TableCell></TableRow>
              <TableRow><TableCell><code>.invalid(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Red border + focus ring.</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dimmed and inert.</TableCell></TableRow>
              <TableRow><TableCell><code>.set_value(..)</code></TableCell><TableCell><code>Option&lt;&amp;str&gt;, Window, cx</code></TableCell><TableCell>Controlled single selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.set_selected(..)</code></TableCell><TableCell><code>&amp;[&amp;str], cx</code></TableCell><TableCell>Controlled multi selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.set_open(..)</code></TableCell><TableCell><code>bool, cx</code></TableCell><TableCell>Controlled popup open state.</TableCell></TableRow>
              <TableRow><TableCell><code>.selected_value()</code></TableCell><TableCell><code>-&gt; Option&lt;SharedString&gt;</code></TableCell><TableCell>Current single selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.selected_values_vec()</code></TableCell><TableCell><code>-&gt; Vec&lt;SharedString&gt;</code></TableCell><TableCell>Current multi selection.</TableCell></TableRow>
              <TableRow><TableCell><code>ComboboxChangeEvent</code></TableCell><TableCell><code>{`{ value, selected }`}</code></TableCell><TableCell>Emitted on every commit/toggle/clear.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与交互参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/combobox" target="_blank" rel="noreferrer">shadcn/ui Combobox (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
          <a href="#keyboard">Keyboard</a>
          <a href="#accessibility">Accessibility</a>
          <a href="#api">API Reference</a>
        </div>
        <div className="component-facts">
          <h2>At a glance</h2>
          <dl>
            <div><dt>Framework</dt><dd>GPUI</dd></div>
            <div><dt>Platform</dt><dd>Desktop</dd></div>
            <div><dt>Examples</dt><dd>8</dd></div>
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
