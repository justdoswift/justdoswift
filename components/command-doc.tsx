import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCommandWorkbench } from "@/components/gpui-command-workbench";
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

const usage = `use justdo_command::command::{Command, CommandItem, CommandSelectEvent};
use gpui_kit::base::input::InputState;

// The input entity is created with a Window, then the
// command entity wraps it and owns filtering + highlight.
let input = cx.new(|cx| {
    InputState::new(window, cx).placeholder("Type a command or search…")
});

let command = cx.new(|cx| {
    Command::new(input, cx)
        .items(vec![
            CommandItem::new("Calendar").group("Suggestions"),
            CommandItem::new("Profile")
                .group("Settings")
                .shortcut("⌘P")
                .icon(CommandIcon::User),
        ])
        .empty_text("No results found.")
});

cx.subscribe(&command, |view, _, ev: &CommandSelectEvent, cx| {
    view.run_command(&ev.value);
    cx.notify();
});

div().child(command)`;

const composition = `Command                   entity — input + list + active row
├── CommandInput          Entity<InputState> — real text editing (gpui-base)
│                         magnifier glyph, 44px row, bottom border
├── CommandList           filtered rows — scrollable (list_max_h)
│   ├── CommandGroup      heading per CommandItem::group(..)
│   ├── CommandSeparator  auto line between different groups
│   ├── CommandItem       icon + label + optional shortcut
│   │                     .value(..) .keywords(..) .disabled(..)
│   ├── CommandShortcut   trailing muted ⌘P / ⌘B / ⌘S
│   └── CommandEmpty      .empty_text(..) — no-match state
└── emits                 CommandSelectEvent { value }`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --command

# Other demos:
cargo run --locked -- --command shortcuts
cargo run --locked -- --command groups
cargo run --locked -- --command scrollable
cargo run --locked -- --command dialog`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# command.rs builds on gpui-base primitives:
# InputState (editing) + ScrollHandle — no extra deps.`;

const keyboardNote = `// Keyboard semantics ride GPUI's action system, not raw
// key capture. A focused single-line input registers no
// up/down handlers, so input::MoveUp / input::MoveDown
// bubble out — the command listens for them and moves the
// highlighted row (skipping disabled items).
//
// Enter propagates as input::Enter and commits the active
// row. Escape clears the query first; on an empty query it
// propagates so a CommandDialog host can close.`;

const filteredNote = `// Filtering is substring match over label, value, group
// and hidden keywords — items can expose extra match-only
// terms that never render:
CommandItem::new("Command Palette")
    .keywords(&["palette", "commands"])
    .shortcut("⇧⌘P")`;

export async function CommandDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/command.rs"),
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
              <BreadcrumbPage>Command</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Command</h1>
            <p>Fast, composable, unstyled command menu for GPUI.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCommandWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Command 是单个 Rust 文件（<code className="inline-code">src/command.rs</code>）—— 文本编辑复用 gpui-base 的 <code className="inline-code">InputState</code>，行图标用 PathBuilder 手绘，列表滚动走 <code className="inline-code">ScrollHandle</code>，dialog 变体用 <code className="inline-code">deferred()</code> 把面板画到 overlay pass。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Command</code> 是实体：输入框是一个真实的 <code className="inline-code">InputState</code>（先 new 出来再传进构造器），实体自持过滤结果与高亮行。输入即过滤（label/value/group/keywords 子串匹配），Enter 或点击发出 <code className="inline-code">CommandSelectEvent</code>。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <CodeBlock code={filteredNote} label="Keywords.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对应 <code className="inline-code">Command → CommandInput + CommandList → CommandGroup + CommandItem + CommandSeparator + CommandShortcut</code>：组头与分隔线由 <code className="inline-code">CommandItem::group(..)</code> 在渲染时自动推导，过滤后空组自动消失。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="keyboard">
          <h2>Keyboard</h2>
          <p>单行输入框不注册上下方向 handler —— 方向键以 <code className="inline-code">input::MoveDown/MoveUp</code> action 冒泡出来，组件在外层接住并移动高亮行（跳过 disabled）。Enter 走 <code className="inline-code">input::Enter</code> 提交；Escape 先清空查询，空查询时再按则向上冒泡，方便 CommandDialog 宿主关闭。</p>
          <CodeBlock code={keyboardNote} label="Key handling.rs" />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Semantics</h3>
              <p>禁用行渲染为半透明且不进入高亮序列；空结果有明确的 <code className="inline-code">empty_text</code> 状态而非空白；列表高度受限时高亮行自动滚入视野。</p>
            </div>
            <div>
              <h3>Keyboard</h3>
              <p>输入保持焦点：↓/↑ 循环移动高亮、Enter 执行、Escape 清空/关闭；鼠标 hover 同步高亮，点击直接提交。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>实体暴露的 builder 方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Command::new(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;</code></TableCell><TableCell>Wraps a caller-created input entity.</TableCell></TableRow>
              <TableRow><TableCell><code>.items(..)</code></TableCell><TableCell><code>Vec&lt;CommandItem&gt;</code></TableCell><TableCell>The collection; filtered as you type.</TableCell></TableRow>
              <TableRow><TableCell><code>CommandItem::new(..)</code></TableCell><TableCell><code>&amp;str label</code></TableCell><TableCell><code>.value(..)</code> <code>.group(..)</code> <code>.shortcut(..)</code> <code>.icon(..)</code> <code>.keywords(..)</code> <code>.disabled(..)</code></TableCell></TableRow>
              <TableRow><TableCell><code>.empty_text(..)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Shown when the query matches nothing.</TableCell></TableRow>
              <TableRow><TableCell><code>.list_max_h(..)</code></TableCell><TableCell><code>Pixels</code></TableCell><TableCell>List viewport cap before scrolling.</TableCell></TableRow>
              <TableRow><TableCell><code>.query()</code></TableCell><TableCell><code>-&gt; String</code></TableCell><TableCell>Current filter text.</TableCell></TableRow>
              <TableRow><TableCell><code>CommandSelectEvent</code></TableCell><TableCell><code>{`{ value }`}</code></TableCell><TableCell>Emitted on Enter / click commit.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与交互参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/command" target="_blank" rel="noreferrer">shadcn/ui Command (React Aria / cmdk) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
