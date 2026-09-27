import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAccordionWorkbench } from "@/components/gpui-accordion-workbench";
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

const usage = `use justdo_command::accordion::{Accordion, AccordionEntry};

// Keep the entity on your host view.
let accordion = cx.new(|_| {
    Accordion::new()
        .entry(AccordionEntry::new("shipping", "What are your shipping options?")
            .content("We offer standard (5-7 days) and express shipping."))
        .entry(AccordionEntry::new("returns", "What is your return policy?")
            .content("Returns are accepted within 30 days of purchase."))
        .entry(AccordionEntry::new("support", "How can I contact support?")
            .content("Reach us through live chat or email."))
        .open("shipping")
});

// In render: div().child(self.accordion.clone())`;

const composition = `Accordion
├── AccordionItem
│   ├── AccordionHeader
│   │   └── AccordionTrigger     button · aria-expanded
│   └── AccordionPanel           region
└── AccordionItem
    ├── AccordionHeader
    │   └── AccordionTrigger
    └── AccordionPanel`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --accordion

# Other demos:
cargo run --locked -- --accordion multiple
cargo run --locked -- --accordion disabled
cargo run --locked -- --accordion card`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# accordion.rs builds on the unstyled primitives
# re-exported at gpui_kit::base:
#   Accordion, AccordionItem, AccordionHeader,
#   AccordionTrigger, AccordionPanel, MotionReveal`;

const eventsSnippet = `// AccordionChange { value, open } is emitted on every toggle.
cx.subscribe(&self.accordion, |view, _accordion, event, cx| {
    println!("{} -> {}", event.value, event.open);
    cx.notify();
});`;

export async function AccordionDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/accordion.rs"),
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
              <BreadcrumbPage>Accordion</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Accordion</h1>
            <p>A vertically stacked set of interactive headings that each reveal a section of content.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAccordionWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Accordion 是单个 Rust 文件（<code className="inline-code">src/accordion.rs</code>），构建在 gpui-base 的无样式原语之上。复制到你的 crate，或下载完整示例直接运行。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p>用 <code className="inline-code">AccordionEntry</code> 描述每一段内容，把实体保留在宿主视图上，渲染时放进任意容器。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">每个条目的 value 是稳定的标识符，open(value) 指定初始展开项。网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构与 shadcn/ui 的 Base UI 版本一一对应。样式由组件实体统一提供，可访问性语义来自 gpui-base 原语。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Keyboard</h3>
              <ul className="key-list">
                <li><kbd>↑</kbd> <kbd>↓</kbd> 在条目间循环移动焦点</li>
                <li><kbd>Home</kbd> <kbd>End</kbd> 跳到第一个 / 最后一个条目</li>
                <li><kbd>Enter</kbd> <kbd>Space</kbd> 展开或收起当前条目</li>
                <li><kbd>Tab</kbd> 沿页面 tab 序进入或离开组件</li>
              </ul>
            </div>
            <div>
              <h3>Semantics</h3>
              <p>标题携带 heading 语义（level 3），触发器是带 aria-expanded 的 button，面板是 region。焦点可见性、disabled 状态与 reduced-motion 由 GPUI 与浏览器偏好共同决定。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>组件实体暴露的方法与事件。结构原语由 <code className="inline-code">gpui_kit::base</code> 提供。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Accordion::new()</code></TableCell><TableCell><code>-&gt; Accordion</code></TableCell><TableCell>Creates the entity content. Single-open and collapsible by default.</TableCell></TableRow>
              <TableRow><TableCell><code>.entry(..)</code></TableCell><TableCell><code>AccordionEntry</code></TableCell><TableCell>Appends one item; call order is presentation order.</TableCell></TableRow>
              <TableRow><TableCell><code>.open(value)</code></TableCell><TableCell><code>&amp;str</code></TableCell><TableCell>Marks an entry expanded initially. Repeatable with multiple.</TableCell></TableRow>
              <TableRow><TableCell><code>.multiple(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Allows several entries to stay open at once.</TableCell></TableRow>
              <TableRow><TableCell><code>.collapsible(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Allows closing the last open entry (single mode).</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark color scheme for surfaces, text and borders.</TableCell></TableRow>
              <TableRow><TableCell><code>AccordionEntry::new(..)</code></TableCell><TableCell><code>value, title</code></TableCell><TableCell>One row: stable value plus its heading.</TableCell></TableRow>
              <TableRow><TableCell><code>.content(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Panel copy revealed by the trigger.</TableCell></TableRow>
              <TableRow><TableCell><code>.disabled(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Inert item: no clicks, skipped by arrow-key navigation.</TableCell></TableRow>
              <TableRow><TableCell><code>AccordionChange</code></TableCell><TableCell><code>{"{ value, open }"}</code></TableCell><TableCell>Emitted on every toggle; subscribe with cx.subscribe.</TableCell></TableRow>
            </TableBody>
          </Table>
          <CodeBlock code={eventsSnippet} label="Example.rs" />
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/accordion" target="_blank" rel="noreferrer">shadcn/ui Accordion <ArrowUpRight size={12} /></a>（Base UI 组合方式）。本页交互由 Rust / GPUI 绘制，无样式原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-base <ArrowUpRight size={12} /></a>。</p>
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
