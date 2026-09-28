import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiCardWorkbench } from "@/components/gpui-card-workbench";
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

const usage = `use justdo_command::card::{Card, card_header, card_content, card_footer, CARD_SPACING};

Card::new().dark(dark)
    .child(card_header(CARD_SPACING, dark, |h| {
        h.title("Card Title")
            .description("Card Description")
            .action(action_button);   // CardAction — top right
    }))
    .child(card_content(CARD_SPACING, dark, |c| {
        c.child(body_content);
    }))
    .child(card_footer(CARD_SPACING, dark, |f| {
        f.child(footer_button);
    }))`;

const composition = `Card                    root — flex-col, gap/py = spacing, rounded-xl + border
├── card_cover(el, sp)  edge-to-edge media, mt(-sp) eats top padding
├── card_header         title + description column
│     .action(el)       top-right slot (CardAction)
│     .bordered(true)   border-b + pb — divided header
├── card_content        main body; .bleed() = -mx-spacing edge-to-edge
└── card_footer         action row; .bordered(true) = border-t + pt`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --card

# Other demos:
cargo run --locked -- --card sm
cargo run --locked -- --card spacing
cargo run --locked -- --card image`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# card.rs composes button.rs for actions — both ship in source.zip.`;

const spacingNote = `// --card-spacing equivalent: one number drives gap, py and
// every section's px. CARD_SPACING = 24, CARD_SPACING_SM = 16.
let sp = 20.;
Card::new().spacing(sp)
    .child(card_content(sp, dark, |c| {
        // bleed a hairline through the inset (shadcn's -mx-(--card-spacing))
        c.child(div().h(px(1.)).w_full().mx(px(-sp)).bg(edge));
    }))`;

export async function CardDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/card.rs"),
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
              <BreadcrumbPage>Card</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Card</h1>
            <p>Displays a card with header, content, and footer.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiCardWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Card 是单个 Rust 文件（<code className="inline-code">src/card.rs</code>）—— 容器与 section 辅助函数自绘，动作位复用 <code className="inline-code">button.rs</code>。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Card</code> 是 render-once 容器：按序 <code className="inline-code">.child(..)</code> 堆叠 <code className="inline-code">card_header</code> / <code className="inline-code">card_content</code> / <code className="inline-code">card_footer</code> / <code className="inline-code">card_cover</code>。每个 section 函数接收 <code className="inline-code">spacing</code>（内边距来源）和一个 spec 闭包。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Card / CardHeader / CardTitle / CardDescription / CardAction / CardContent / CardFooter。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Spacing scale.</strong> shadcn 的 <code className="inline-code">--card-spacing</code> 变量在 GPUI 里就是一个 <code className="inline-code">f32</code>：<code className="inline-code">.size(CardSize::Sm)</code> 切到 16px，<code className="inline-code">.spacing(..)</code> 任意覆盖 —— gap、py、section px 同步生效。Edge-to-edge 内容用 <code className="inline-code">mx(-sp)</code>（divider）或整段 <code className="inline-code">.bleed()</code>。</p>
          <CodeBlock code={spacingNote} label="Example.rs" />
          <p><strong>Text wrapping.</strong> GPUI 文本默认不换行 —— header 的 title/description 已启用 <code className="inline-code">whitespace_normal()</code> 且所在列 <code className="inline-code">flex_1 + min_w_0</code>（CSS 里 flex-1 + min-w-0 的同款约束），长描述在窄卡里正确折行。</p>
          <p><strong>RTL.</strong> shadcn 的 RTL 示例依赖阿拉伯文字形；Geist 不包含阿拉伯文，本移植不做该演示。</p>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>容器与 section helper 的 API。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Card::new()</code></TableCell><TableCell><code>-&gt; Card</code></TableCell><TableCell>Render-once root container (384px default width).</TableCell></TableRow>
              <TableRow><TableCell><code>.size(..)</code></TableCell><TableCell><code>CardSize</code></TableCell><TableCell>Default (24px) | Sm (16px) spacing scale.</TableCell></TableRow>
              <TableRow><TableCell><code>.spacing(..)</code></TableCell><TableCell><code>f32</code></TableCell><TableCell>Explicit --card-spacing override.</TableCell></TableRow>
              <TableRow><TableCell><code>.w(..)</code> / <code>.dark(..)</code></TableCell><TableCell><code>f32</code> / <code>bool</code></TableCell><TableCell>Card width / theme.</TableCell></TableRow>
              <TableRow><TableCell><code>.child(..)</code></TableCell><TableCell><code>impl IntoElement</code></TableCell><TableCell>Append section or free content, in order.</TableCell></TableRow>
              <TableRow><TableCell><code>card_header(sp, dark, |h|)</code></TableCell><TableCell><code>-&gt; AnyElement</code></TableCell><TableCell>h.title / .description / .action / .bordered / .child.</TableCell></TableRow>
              <TableRow><TableCell><code>card_content(sp, dark, |c|)</code></TableCell><TableCell><code>-&gt; AnyElement</code></TableCell><TableCell>c.child / .bleed (edge-to-edge).</TableCell></TableRow>
              <TableRow><TableCell><code>card_footer(sp, dark, |f|)</code></TableCell><TableCell><code>-&gt; AnyElement</code></TableCell><TableCell>f.child / .bordered / .column / .justify_end.</TableCell></TableRow>
              <TableRow><TableCell><code>card_cover(el, sp)</code></TableCell><TableCell><code>-&gt; AnyElement</code></TableCell><TableCell>Top-bleed media child (image, gradient, map).</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/card" target="_blank" rel="noreferrer">shadcn/ui Card (React Aria) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
