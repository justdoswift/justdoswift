import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCalendarWorkbench } from "@/components/gpui-calendar-workbench";
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

const usage = `use justdo_command::calendar::{Calendar, CalendarMode, CalendarSelectEvent, Date};

// Calendar is an entity — host it in a view and subscribe for selections.
let calendar = cx.new(|_| {
    Calendar::new()
        .selected(Date::today())
        .dropdown(true)
});
cx.subscribe(&calendar, |_, _, event: &CalendarSelectEvent, cx| {
    // CalendarSelection::Single(date) | ::Range { start, end }
    cx.notify();
})
.detach();
// in render(): .child(calendar.clone())`;

const composition = `Calendar                  entity — owns visible month + selection state
├── header                ‹ prev · caption · next ›
│     caption             text, or "September ⌄  2026 ⌄" with .dropdown(true)
│     popup               4×3 month grid / ±5-year grid overlay
├── weekday row           S M T W T F S
└── day grid              32px cells — outside days dimmed
    Date                  proleptic-Gregorian math, no external crate
    CalendarSelectEvent   emitted on every pick (Single / Range)`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --calendar

# Other demos:
cargo run --locked -- --calendar range
cargo run --locked -- --calendar dropdown
cargo run --locked -- --calendar presets
cargo run --locked -- --calendar datetime
cargo run --locked -- --calendar booked`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

[target.'cfg(target_family = "wasm")'.dependencies]
js-sys = "=0.3.98"   # browser "today" — SystemTime::now panics on wasm`;

const statesNote = `// Cell states are computed per day in day_cell():
is_today     → accent background
is_selected  → primary bg + primary-foreground ink
range start/end → primary, outer rounded corner
range middle    → accent bg, square corners
outside         → muted ink
disabled_dates  → muted + 45% opacity, clicks intercepted`;

export async function CalendarDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/calendar.rs"),
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
              <BreadcrumbPage>Calendar</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Calendar</h1>
            <p>A date field component that allows users to enter and edit a single date or a range of dates.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCalendarWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Calendar 是单个 Rust 文件（<code className="inline-code">src/calendar.rs</code>），日期数学用 Howard Hinnant 的 civil-from-days 算法自实现 —— 不依赖 chrono。浏览器里 &quot;today&quot; 走 <code className="inline-code">js_sys::Date</code>，因为 <code className="inline-code">SystemTime::now()</code> 在 wasm32 上会 panic。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Calendar</code> 是一个 entity（持 visible month、selection、disabled 集合），宿主在 <code className="inline-code">cx.new(..)</code> 里构造并 <code className="inline-code">cx.subscribe</code> 收 <code className="inline-code">CalendarSelectEvent</code>。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对齐 shadcn/ui 的 Calendar / RangeCalendar。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={statesNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Range picking.</strong> <code className="inline-code">.mode(CalendarMode::Range)</code> 下首点定 start、次点定 end（点在 start 之前会自动交换），第三次点击重开新 range。<code className="inline-code">.months(2)</code> 并排渲染相邻两月，‹ › 只出现在首/末格。</p>
          <p><strong>Caption dropdown.</strong> <code className="inline-code">.dropdown(true)</code> 把 caption 换成 &quot;September ⌄ / 2026 ⌄&quot; 两个 ghost 按钮，点击展开 4×3 月格或前后 ±5 年网格；弹层用 <code className="inline-code">block_mouse_except_scroll()</code> 拦截点击，不会穿透到下层日期格。</p>
          <p><strong>Outside days.</strong> 上/下月补位的日期灰显但可点 —— 点了之后视图跳到该日期所在月（shadcn 同款行为）。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard &amp; focus</h3>
              <p>每一天是 <code className="inline-code">gpui_base::Button</code> —— Tab 聚焦、Enter/Space 激活、focus-visible 描边都由原语提供；icon-only 翻月按钮带 <code className="inline-code">a11y_label(&quot;Previous month&quot;)</code>。</p>
            </div>
            <div>
              <h3>Disabled dates</h3>
              <p><code className="inline-code">.disabled_dates(..)</code> 内的日期被原语标记为 disabled —— 半透明渲染且点击拦截，配合预设表示已订/不可用。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>Calendar entity 的 builder API 与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Calendar::new()</code></TableCell><TableCell><code>-&gt; Calendar</code></TableCell><TableCell>Entity; visible month defaults to today.</TableCell></TableRow>
              <TableRow><TableCell><code>.mode(..)</code></TableCell><TableCell><code>CalendarMode</code></TableCell><TableCell>Single | Range picking.</TableCell></TableRow>
              <TableRow><TableCell><code>.months(..)</code></TableCell><TableCell><code>usize</code></TableCell><TableCell>1 or 2 month columns side by side.</TableCell></TableRow>
              <TableRow><TableCell><code>.visible_month(..)</code></TableCell><TableCell><code>(year, month)</code></TableCell><TableCell>First visible month.</TableCell></TableRow>
              <TableRow><TableCell><code>.selected(..)</code></TableCell><TableCell><code>Date</code></TableCell><TableCell>Initial single selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.range(..)</code></TableCell><TableCell><code>(start, end)</code></TableCell><TableCell>Initial range selection.</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled_dates(..)</code></TableCell><TableCell><code>Vec&lt;Date&gt;</code></TableCell><TableCell>Unpickable dates (booked).</TableCell></TableRow>
              <TableRow><TableCell><code>.dropdown(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Caption → month/year select buttons.</TableCell></TableRow>
              <TableRow><TableCell><code>CalendarSelectEvent</code></TableCell><TableCell><code>selection: CalendarSelection</code></TableCell><TableCell>Single(Date) | Range {"{"} start, end {"}"}.</TableCell></TableRow>
              <TableRow><TableCell><code>Date</code></TableCell><TableCell><code>{"{"} year, month, day {"}"}</code></TableCell><TableCell>today(), add_days(..); no external crate.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/calendar" target="_blank" rel="noreferrer">shadcn/ui Calendar (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
