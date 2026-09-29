import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiDatePickerWorkbench } from "@/components/gpui-date-picker-workbench";
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

const usage = `use justdo_command::calendar::{Calendar, CalendarMode};
use justdo_command::date_picker::{DatePick, DatePicker, DatePickerEvent};

// DatePicker wraps a Calendar entity in a popover behind a trigger.
let calendar = cx.new(|_| Calendar::new());
let picker = cx.new(|cx| {
    DatePicker::new(Some(calendar), window, cx)
        .placeholder("Pick a date")
});
cx.subscribe(&picker, |_, _, ev: &DatePickerEvent, cx| {
    // ev.selection — Option<DatePick::Single(Date) | DatePick::Range{..}>
    // ev.time      — Option<(hour, minute)> when .time_input(..) is wired
    cx.notify();
})
.detach();
// in render(): .child(picker.clone())`;

const composition = `DatePicker                entity — owns open state + echoed selection
├── trigger               button row: calendar icon + label + chevron ⌄
│     label               "Pick a date" → "September 12, 2025"
│     variant             text span, typed InputState, or natural input
└── deferred popover      absolute, anchored under the trigger
      ├── Calendar        entity — same one used standalone
      └── Time row        .time_input(..) adds a "Time  [9:00]" footer

DatePick                  Single(Date) | Range { start, end }
DatePickerEvent           { selection, time } — emitted on every change`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --date-picker

# Other demos:
cargo run --locked -- --date-picker range
cargo run --locked -- --date-picker dob
cargo run --locked -- --date-picker input
cargo run --locked -- --date-picker time
cargo run --locked -- --date-picker natural`;

const parsing = `// Three parsers ship in date_picker.rs — no external crate:

parse_date("2026-09-30")  // ISO — also YYYY/M/D
parse_date("9/30/2026")   // US — also M-D-YYYY

parse_time("9:00")        // → (9, 0)
parse_time("9:30 pm")     // → (21, 30); "12 am" → (0, 0)

parse_natural("tomorrow")       // today + 1
parse_natural("in 3 days")      // today + 3 (days/weeks/months)
parse_natural("next Friday")    // nearest future Friday
parse_natural("2026-09-30")     // falls through to parse_date`;

export async function DatePickerDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/date_picker.rs"),
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
              <BreadcrumbPage>Date Picker</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Date Picker</h1>
            <p>A date picker combines a button, a popover and a calendar so users can select a date or a range of dates.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiDatePickerWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Date Picker 是单个 Rust 文件（<code className="inline-code">src/date_picker.rs</code>），复用 <code className="inline-code">calendar.rs</code> 的日期数学 —— 不依赖 chrono。弹层定位用 <code className="inline-code">deferred(..)</code> overlay pass + <code className="inline-code">on_children_prepainted</code> 捕获触发器 bounds，嵌在任何布局里都能锚定。</p>
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">DatePicker</code> 是一个 entity：把（可选的）<code className="inline-code">Calendar</code> entity 交给它托管，订阅 <code className="inline-code">DatePickerEvent</code> 拿选中态。Calendar 自身的能力（Range / dropdown caption / disabled dates / months(2)）原样透出。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对齐 shadcn/ui 的 <code className="inline-code">PopoverTrigger → Button + Popover → Calendar</code>。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="parsing">
          <h2>Input &amp; parsing</h2>
          <p>三个变体共用一套手写 parser（零依赖）：ISO/US 日期、12/24 小时时间、自然语言短语。</p>
          <CodeBlock code={parsing} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Open/close semantics.</strong> 单选模式下点日期即关弹层（Radix 同款）；Range 模式点起点后弹层保持，点终点才关。点击弹层外的遮罩也关闭。</p>
          <p><strong>Typed input.</strong> <code className="inline-code">.date_input(input, cx)</code> 把触发器的文案换成真实单行编辑器 —— 解析成功即高亮日历对应日期；反向从日历选中会把 ISO 日期写回输入框（subscribe 回调拿不到 Window，回写在 render 里应用 pending 值）。</p>
          <p><strong>Natural language.</strong> <code className="inline-code">.natural_input(..)</code> 不挂日历 —— Enter 解析 &quot;tomorrow&quot; / &quot;in 3 days&quot; / &quot;next Friday&quot; / ISO 日期，并在下方回显 &quot;Your post will be published on …&quot; 式确认语。</p>
          <p><strong>Anchoring.</strong> 弹层宽度按 mode 取 296/560px，水平方向钳在组件根 bounds 内，永远贴着触发器下沿 +6px。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Pointer semantics</h3>
              <p>触发器整行可点；输入变体里只有图标格触发开关，输入框点击不会冒泡成 toggle。弹层外有全幅遮罩承接 &quot;点击外部关闭&quot;。</p>
            </div>
            <div>
              <h3>Calendar internals</h3>
              <p>弹层内的日历复用 <code className="inline-code">Calendar</code> entity —— 每天是 <code className="inline-code">gpui_base::Button</code>（Tab/Enter/Space 完备），翻月按钮带 a11y label。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>DatePicker entity 的 builder API、读取方法与事件。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>DatePicker::new(..)</code></TableCell><TableCell><code>Option&lt;Entity&lt;Calendar&gt;&gt;, window, cx</code></TableCell><TableCell>Hosts the calendar; None = input-only.</TableCell></TableRow>
              <TableRow><TableCell><code>.placeholder(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Trigger text before a pick.</TableCell></TableRow>
              <TableRow><TableCell><code>.label(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Field caption above the trigger.</TableCell></TableRow>
              <TableRow><TableCell><code>.date_input(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;, cx</code></TableCell><TableCell>Typed ISO/US date field.</TableCell></TableRow>
              <TableRow><TableCell><code>.natural_input(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;, cx</code></TableCell><TableCell>Natural-language parse on Enter.</TableCell></TableRow>
              <TableRow><TableCell><code>.time_input(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;, cx</code></TableCell><TableCell>Time row inside the popover.</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Muted + click-inert trigger.</TableCell></TableRow>
              <TableRow><TableCell><code>set_open(..)</code></TableCell><TableCell><code>bool, cx</code></TableCell><TableCell>Programmatic open/close.</TableCell></TableRow>
              <TableRow><TableCell><code>date() / range()</code></TableCell><TableCell><code>Option&lt;..&gt;</code></TableCell><TableCell>Read the echoed selection.</TableCell></TableRow>
              <TableRow><TableCell><code>DatePickerEvent</code></TableCell><TableCell><code>{"{"} selection, time {"}"}</code></TableCell><TableCell>DatePick::Single | ::Range; Option&lt;(h, m)&gt;.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/date-picker" target="_blank" rel="noreferrer">shadcn/ui Date Picker (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
          <a href="#parsing">Input &amp; parsing</a>
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
