import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiBubbleWorkbench } from "@/components/gpui-bubble-workbench";
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

const usage = `use justdo_command::bubble::{Bubble, BubbleVariant, BubbleAlign};

// Bubble is an entity — keep it on your view, render as a
// child, subscribe for BubblePressEvent when pressable.
let bubble = cx.new(|_| {
    Bubble::new("Hey there! what's up?")
        .variant(BubbleVariant::Secondary)
        .align(BubbleAlign::Start)
        .reactions(vec!["👍".into()], BubbleAlign::Start)
});
div().child(bubble)`;

const composition = `Bubble                    entity — the framed surface
├── BubbleContent         .text — sizes to content, up to 80% row
│   └── "Show more"       .collapsible(preview) toggle
├── BubbleReactions       .reactions(chips, side) — edge-overlapping chips
└── pressable             .pressable(true) -> BubblePressEvent

BubbleGroup               column of same-sender bubbles, small gap`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --bubble

# Other demos:
cargo run --locked -- --bubble alignment
cargo run --locked -- --bubble group
cargo run --locked -- --bubble reactions
cargo run --locked -- --bubble buttons
cargo run --locked -- --bubble collapsible`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# bubble.rs is self-contained — reactions and frames
# are plain GPUI elements, no extra deps.`;

export async function BubbleDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/bubble.rs"),
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
              <BreadcrumbPage>Bubble</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Bubble</h1>
            <p>Displays conversational content in a message bubble. Supports variants, alignment, grouping, reactions, and collapsible content.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiBubbleWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Bubble 是单个 Rust 文件（<code className="inline-code">src/bubble.rs</code>），只依赖 gpui-kit —— 气泡、reaction chips、折叠开关都是普通 GPUI 元素。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Bubble</code> 是实体（<code className="inline-code">cx.new</code>），builder 配置文案、变体、对齐、reactions 与可点击性。气泡随内容自适应宽度，上限为行的 80%；<code className="inline-code">Ghost</code> 变体取消上限。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Bubble / BubbleContent / BubbleReactions / BubbleGroup。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Reactions.</strong> chips 用 <code className="inline-code">absolute + bottom(px(-20.))</code> 在独立叠层中跨过气泡下缘，以 3px 背景色圆环形成凹口；<code className="inline-code">side</code> 控制停靠左/右；气泡自动加底部间距防止压住下一行。</p>
          <p><strong>Pressable vs collapsible.</strong> <code className="inline-code">.pressable(true)</code> 让整个气泡变成按钮（focusable + <code className="inline-code">BubblePressEvent</code>）；<code className="inline-code">.collapsible(..)</code> 在气泡内部放独立的 Show more/less 切换，两者可以共存但语义不同。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Alignment = sender</h3>
              <p>start/end 对齐表达收发双方；pressable 气泡可 Tab 聚焦并有 focus-visible 描边，否则不参与焦点顺序。</p>
            </div>
            <div>
              <h3>Reactions</h3>
              <p>reaction chips 是纯展示叠加层；emoji 经系统字体渲染彩色字形，明暗主题均有描边分隔。</p>
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
              <TableRow><TableCell><code>Bubble::new(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Bubble text; Secondary variant, Start aligned.</TableCell></TableRow>
              <TableRow><TableCell><code>.variant(..)</code></TableCell><TableCell><code>BubbleVariant</code></TableCell><TableCell>Default | Secondary | Muted | Tinted | Outline | Ghost | Destructive.</TableCell></TableRow>
              <TableRow><TableCell><code>.align(..)</code></TableCell><TableCell><code>BubbleAlign</code></TableCell><TableCell>Start | End — which edge of the row the bubble hugs.</TableCell></TableRow>
              <TableRow><TableCell><code>.reactions(..)</code></TableCell><TableCell><code>Vec&lt;SharedString&gt;, BubbleAlign</code></TableCell><TableCell>Edge-overlapping emoji chips; side picks the anchor edge.</TableCell></TableRow>
              <TableRow><TableCell><code>.pressable(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Button-style bubble: focus + BubblePressEvent on press.</TableCell></TableRow>
              <TableRow><TableCell><code>.collapsible(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Collapsed preview text + Show more/less toggle.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark scheme for surfaces, ink and chips.</TableCell></TableRow>
              <TableRow><TableCell><code>BubblePressEvent</code></TableCell><TableCell><code>{`{ label }`}</code></TableCell><TableCell>Emitted by pressable bubbles; subscribe via cx.subscribe.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/bubble" target="_blank" rel="noreferrer">shadcn/ui Bubble <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
