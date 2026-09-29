import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCheckboxWorkbench } from "@/components/gpui-checkbox-workbench";
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

const usage = `use justdo_command::checkbox::{Checkbox, CheckboxState, CheckboxChangeEvent};

// Checkbox is an entity — click or Space toggles it; the
// host view keeps it and subscribes for the change event.
let terms = cx.new(|_| {
    Checkbox::new()
        .label("Accept terms and conditions")
        .description("By clicking this checkbox, you agree.")
});

cx.subscribe(&terms, |view, _, event: &CheckboxChangeEvent, cx| {
    view.accepted = event.checked;
    cx.notify();
});

div().child(terms)`;

const composition = `Checkbox                  entity — box + optional Field label/description
├── box                   16px rounded square; checked -> primary fill + ✓ mark
│                                               indeterminate -> fill + — dash
├── label                 .label(..) — clicks on it toggle the box
├── description           .description(..) — xs muted helper text
├── state                 Unchecked | Checked | Indeterminate
├── invalid               .invalid(true) — red border + label (isInvalid)
├── disabled              .disabled(true) — dimmed, unfocusable, inert
└── emits                 CheckboxChangeEvent { checked } on toggle`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --checkbox

# Other demos:
cargo run --locked -- --checkbox checked
cargo run --locked -- --checkbox invalid
cargo run --locked -- --checkbox disabled
cargo run --locked -- --checkbox group
cargo run --locked -- --checkbox table`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# checkbox.rs is self-contained — the check/dash marks are
# painted with PathBuilder, no icon fonts or assets.`;

const tableNote = `// Select-all pattern: rows emit CheckboxChangeEvent, the host
// recomputes the header checkbox — all Checked, none Unchecked,
// otherwise Indeterminate. Clicking the header drives every row.
let header = cx.new(|_| Checkbox::new());

for row in &rows {
    let header = header.clone();
    cx.subscribe(row, move |demo, _, _ev, cx| {
        let states: Vec<CheckboxState> =
            demo.rows.iter().map(|r| r.read(cx).state).collect();
        let st = if all_checked(&states) {
            CheckboxState::Checked
        } else if none_checked(&states) {
            CheckboxState::Unchecked
        } else {
            CheckboxState::Indeterminate
        };
        header.update(cx, |h, cx| h.set_state(st, cx));
    });
}

cx.subscribe(&header, move |demo, _, ev: &CheckboxChangeEvent, cx| {
    for row in &demo.rows {
        row.update(cx, |c, cx| c.set_state(
            if ev.checked { Checked } else { Unchecked }, cx));
    }
});`;

export async function CheckboxDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/checkbox.rs"),
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
              <BreadcrumbPage>Checkbox</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Checkbox</h1>
            <p>A control that allows the user to toggle between checked and not checked.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCheckboxWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Checkbox 是单个 Rust 文件（<code className="inline-code">src/checkbox.rs</code>），只依赖 gpui-kit —— 对勾与半选横杠用 <code className="inline-code">PathBuilder</code> 手绘，无资源文件。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Checkbox</code> 是实体（<code className="inline-code">cx.new</code>），内部自持三态；点击盒子、label 或按 Space 都会翻转并发出 <code className="inline-code">CheckboxChangeEvent</code>。label/description 即 shadcn 的 Field + FieldLabel + FieldDescription 组合。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Checkbox —— box / label / description / invalid / disabled / indeterminate 三态。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={tableNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Tri-state.</strong> 与 React Aria 一致：点击 Indeterminate 的盒子进入 Checked（不是 Unchecked）。表头 select-all 用这个语义实现全选。</p>
          <p><strong>Mark animation.</strong> ✓/— 标记的出现走 120ms opacity 过渡（<code className="inline-code">transition</code> keyed state，按实体区分），<code className="inline-code">cx.reduce_motion()</code> 时直接到位。</p>
          <p><strong>Focus.</strong> 整行可点，但只有盒子进入 Tab 顺序；focus-visible 时在盒子周围画 accent 光环 —— 与 shadcn 的 <code className="inline-code">ring</code> 样式对应。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <p>Tab 聚焦、Space 切换（prevent_default 阻止页面滚动）；disabled 的盒子完全移出焦点顺序且不响应输入。</p>
            </div>
            <div>
              <h3>Invalid</h3>
              <p>invalid 态不只改边框颜色 —— label 同步转红，双重信号表达校验失败，接近 <code className="inline-code">data-invalid</code> 的 Field 表现。</p>
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
              <TableRow><TableCell><code>Checkbox::new()</code></TableCell><TableCell><code>-&gt; Checkbox</code></TableCell><TableCell>Unchecked, no label, enabled.</TableCell></TableRow>
              <TableRow><TableCell><code>.label(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>FieldLabel text; clicking it toggles the box.</TableCell></TableRow>
              <TableRow><TableCell><code>.description(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>FieldDescription helper text under the label.</TableCell></TableRow>
              <TableRow><TableCell><code>.checked(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Initial state (defaultSelected equivalent).</TableCell></TableRow>
              <TableRow><TableCell><code>.indeterminate()</code></TableCell><TableCell><code>-</code></TableCell><TableCell>Partial-selection state (select-all header).</TableCell></TableRow>
              <TableRow><TableCell><code>.invalid(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Red border + label (isInvalid).</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dimmed, unfocusable, ignores input.</TableCell></TableRow>
              <TableRow><TableCell><code>.set_state(..)</code></TableCell><TableCell><code>CheckboxState, &amp;mut Context</code></TableCell><TableCell>Controlled update — e.g. select-all driving rows.</TableCell></TableRow>
              <TableRow><TableCell><code>CheckboxChangeEvent</code></TableCell><TableCell><code>{`{ checked: bool }`}</code></TableCell><TableCell>Emitted on click/Space toggle; subscribe via cx.subscribe.</TableCell></TableRow>
              <TableRow><TableCell><code>CheckboxState</code></TableCell><TableCell><code>Unchecked | Checked | Indeterminate</code></TableCell><TableCell>Tri-state value; clicking Indeterminate goes to Checked.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/checkbox" target="_blank" rel="noreferrer">shadcn/ui Checkbox <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
