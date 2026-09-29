import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiContextMenuWorkbench } from "@/components/gpui-context-menu-workbench";
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

const usage = `use justdo_command::context_menu::{
    ContextMenu, ContextMenuEntry, ContextMenuItem, ContextMenuSelectEvent,
};

let menu = cx.new(|cx| {
    ContextMenu::new(
        // Trigger area — any renderable element factory.
        move || {
            div()
                .p_4()
                .border_1()
                .border_dashed()
                .child("Right click here")
                .into_any_element()
        },
        vec![
            ContextMenuEntry::Item(ContextMenuItem::new("Profile")),
            ContextMenuEntry::Item(ContextMenuItem::new("Billing")),
            ContextMenuEntry::Item(
                ContextMenuItem::new("Settings").shortcut("⌘S"),
            ),
            ContextMenuEntry::Separator,
            ContextMenuEntry::Item(
                ContextMenuItem::new("Delete").destructive(true),
            ),
        ],
        cx,
    )
});

cx.subscribe(&menu, |view, _, ev: &ContextMenuSelectEvent, cx| {
    view.handle(ev.value.clone());
    cx.notify();
});

div().size_full().child(menu)`;

const composition = `ContextMenu                entity — trigger + popup + nav state
├── ContextMenuTrigger    .trigger(factory) — right-click surface
│                         (secondary-button MouseDown → open at pointer)
├── ContextMenuContent    floating panel — deferred pass, viewport-clamped
│   ├── ContextMenuGroup  ContextMenuEntry::Label — muted heading
│   ├── ContextMenuItem   icon + label + shortcut + ▸
│   │                     .value(..) .icon(..) .shortcut(..)
│   │                     .disabled(..) .destructive(..)
│   ├── ContextMenuCheckboxItem   .checkbox(bool) — stays open on toggle
│   ├── ContextMenuRadioItem      .radio("group") — exclusive per group
│   ├── ContextMenuSeparator      ContextMenuEntry::Separator
│   └── ContextMenuSub    ContextMenuItem::submenu(vec![...])
│         └── SubContent  second panel at the trigger row
└── emits                 ContextMenuSelectEvent { value, checked, group }`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --context-menu

# Other demos:
cargo run --locked -- --context-menu submenu
cargo run --locked -- --context-menu selection
cargo run --locked -- --context-menu destructive`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# context_menu.rs is self-contained: FocusHandle for keyboard
# nav, deferred() for the popup pass, PathBuilder glyphs — no
# extra deps.`;

const positionNote = `// Pointer coordinates arrive in window space; the component
// captures its root bounds each prepaint and converts to local
// space, so the menu anchors correctly wherever the trigger
// sits. Panels clamp inside the root bounds (MENU_W = 208px).
//
// Right-clicking elsewhere while open re-anchors the menu —
// same as the OS menu.`;

const keyboardNote = `// When the menu opens, a FocusHandle on the overlay takes
// keyboard focus:
//
//   ↑ / ↓    move the active row (disabled rows are skipped)
//   →        open the submenu under the active row
//   ←        close the submenu (or the menu)
//   Enter    activate the active row
//   Escape   close
//
// Hovering a row also moves the highlight and opens its
// submenu, mirroring OS menus.`;

export async function ContextMenuDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/context_menu.rs"),
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
              <BreadcrumbPage>Context Menu</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Context Menu</h1>
            <p>Displays a menu of actions triggered by a right click.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiContextMenuWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Context Menu 是单个 Rust 文件（<code className="inline-code">src/context_menu.rs</code>）—— 浮层走 <code className="inline-code">deferred()</code> overlay pass，键盘导航用 <code className="inline-code">FocusHandle</code> + <code className="inline-code">on_key_down</code>，图标和 ⌘/⇧/⌥/⌃ 修饰键全部 PathBuilder 手绘（Geist 没有这些码位）。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">ContextMenu</code> 是实体：<code className="inline-code">.trigger(..)</code> 接收一个元素工厂作为右键区域，菜单项用 <code className="inline-code">ContextMenuEntry</code> 枚举声明（Item / Label / Separator）。右键在指针处弹出，点击行发出 <code className="inline-code">ContextMenuSelectEvent</code>。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对应 <code className="inline-code">ContextMenuTrigger + ContextMenuContent → Item / Label / Separator / Sub / CheckboxItem / RadioItem / Shortcut</code>。子菜单用 <code className="inline-code">.submenu(vec![...])</code> 内嵌，复选与单选行提交后菜单保持打开。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={positionNote} label="Positioning.rs" />
        </section>

        <section className="doc-section" id="keyboard">
          <h2>Keyboard</h2>
          <p>菜单打开时焦点移到浮层：↑/↓ 移动高亮（跳过禁用行），→ 打开子菜单，← 收起子菜单，Enter 执行，Escape 关闭。鼠标 hover 同样驱动高亮与子菜单开合。</p>
          <CodeBlock code={keyboardNote} label="Key handling.rs" />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Semantics</h3>
              <p>禁用行半透明且不进入高亮序列；destructive 项红色双重标识；checkbox 行渲染 ✓、radio 行渲染圆点，状态可从实体读取（<code className="inline-code">checked(..)</code> / <code className="inline-code">radio_value(..)</code>）。</p>
            </div>
            <div>
              <h3>Dismissal</h3>
              <p>点击外部 / Escape / 选择普通项关闭；右键点击别处把菜单重新锚定到新的指针位置，与系统菜单一致。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>实体暴露的构造器、builder 方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>ContextMenu::new(..)</code></TableCell><TableCell><code>trigger factory, Vec&lt;ContextMenuEntry&gt;</code></TableCell><TableCell>Trigger renders via a factory closure.</TableCell></TableRow>
              <TableRow><TableCell><code>ContextMenuItem::new(..)</code></TableCell><TableCell><code>&amp;str label</code></TableCell><TableCell><code>.value(..)</code> <code>.icon(..)</code> <code>.shortcut(..)</code> <code>.disabled(..)</code> <code>.destructive(..)</code></TableCell></TableRow>
              <TableRow><TableCell><code>.checkbox(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Toggle row; menu stays open.</TableCell></TableRow>
              <TableRow><TableCell><code>.radio(..)</code></TableCell><TableCell><code>&amp;str group</code></TableCell><TableCell>Exclusive within the group.</TableCell></TableRow>
              <TableRow><TableCell><code>.submenu(..)</code></TableCell><TableCell><code>Vec&lt;ContextMenuEntry&gt;</code></TableCell><TableCell>Nested panel (one level deep).</TableCell></TableRow>
              <TableRow><TableCell><code>.set_open(..)</code></TableCell><TableCell><code>bool, Window, cx</code></TableCell><TableCell>Controlled open at trigger center.</TableCell></TableRow>
              <TableRow><TableCell><code>.checked(..)</code></TableCell><TableCell><code>-&gt; Option&lt;bool&gt;</code></TableCell><TableCell>Checkbox state by value.</TableCell></TableRow>
              <TableRow><TableCell><code>.radio_value(..)</code></TableCell><TableCell><code>-&gt; Option&lt;SharedString&gt;</code></TableCell><TableCell>Current value of a radio group.</TableCell></TableRow>
              <TableRow><TableCell><code>ContextMenuSelectEvent</code></TableCell><TableCell><code>{`{ value, checked, group }`}</code></TableCell><TableCell>Emitted on every activation.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与交互参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/context-menu" target="_blank" rel="noreferrer">shadcn/ui Context Menu (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
