import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiChartWorkbench } from "@/components/gpui-chart-workbench";
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

const usage = `use justdo_command::chart::{Chart, ChartKind, ChartSeries};

let chart = cx.new(|_| {
    Chart::new(ChartKind::Bar)
        .title("Bar Chart - Interactive", "…last 6 months")
        .config(vec![
            ChartSeries::new("Desktop", 0x2563eb, 0x3b82f6),
            ChartSeries::new("Mobile", 0x60a5fa, 0x93c5fd),
        ])
        .data(vec![
            ("Jan", vec![186., 80.]),
            ("Feb", vec![305., 200.]),
            ("Mar", vec![237., 120.]),
            // …
        ])
        .dark(dark)
});`;

const composition = `Chart                 card-styled container — title + caption + plot + legend
├── plot              relative box, 220px tall
│   ├── canvas        grid lines / line+area paths / donut slices
│   ├── bars          flex bands (Bar | Stacked)
│   ├── hover bands   one transparent flex_1 column per x category
│   └── tooltip       absolute, follows the cursor
├── x axis            per-category flex_1 labels
└── legend            dot + label + series total`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --chart

# Other demos:
cargo run --locked -- --chart stacked
cargo run --locked -- --chart line
cargo run --locked -- --chart area
cargo run --locked -- --chart donut`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# chart.rs is self-contained — no chart library dependency.`;

const configNote = `// ChartSeries is the ChartConfig equivalent — label plus
// light/dark colors, decoupled from the data rows.
const DESKTOP: (u32, u32) = (0x2563eb, 0x3b82f6); // light, dark
ChartSeries::new("Desktop", DESKTOP.0, DESKTOP.1)`;

export async function ChartDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/chart.rs"),
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
              <BreadcrumbPage>Chart</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Chart</h1>
            <p>Beautiful charts drawn directly in GPUI — no chart library.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiChartWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Chart 是单个 Rust 文件（<code className="inline-code">src/chart.rs</code>）—— 无图表库依赖：网格、柱、线、面积、环形全用 GPUI 的 div 与 <code className="inline-code">canvas</code>/<code className="inline-code">PathBuilder</code> 自绘。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Chart::new(kind)</code> 建实体，<code className="inline-code">.config(..)</code> 传 <code className="inline-code">ChartSeries</code>（ChartConfig 等价物：label + light/dark 色），<code className="inline-code">.data(..)</code> 传 (类目, 各系列值) 行。悬停即出 tooltip，无需接线。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 ChartContainer / ChartTooltip / ChartLegend（Recharts）。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Chart config.</strong> <code className="inline-code">ChartSeries</code> 解耦 label 与 light/dark 色值 —— 与 shadcn 的 <code className="inline-code">ChartConfig</code> 对应，数据行只带数值。</p>
          <CodeBlock code={configNote} label="Example.rs" />
          <p><strong>Tooltip.</strong> 悬停命中按 x 分带（cartesian）或按极角+半径（donut）判定；<code className="inline-code">.indicator(IndicatorStyle::Line)</code> 换线性指示条，默认圆点。donut 悬停会把其余切片降到 35% 透明。</p>
          <p><strong>Scale.</strong> y 轴归一取单列最大值（<code className="inline-code">Stacked</code> 取列总和）。网格 4 条水平线、无纵向网格（对齐 <code className="inline-code">CartesianGrid vertical=&#123;false&#125;</code>）。</p>
          <p><strong>Not ported.</strong> Recharts 的动画入场、刷选缩放、雷达/径向图未实现；RTL 文字依赖阿拉伯字形（Geist 不含）。</p>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>Chart 实体与配置。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Chart::new(kind)</code></TableCell><TableCell><code>ChartKind</code></TableCell><TableCell>Bar | Stacked | Line | Area | Donut.</TableCell></TableRow>
              <TableRow><TableCell><code>.title(.., ..)</code></TableCell><TableCell><code>&str, &str</code></TableCell><TableCell>Card-style heading + caption above the plot.</TableCell></TableRow>
              <TableRow><TableCell><code>.config(..)</code></TableCell><TableCell><code>Vec&lt;ChartSeries&gt;</code></TableCell><TableCell>Series (or donut slice) labels + light/dark colors.</TableCell></TableRow>
              <TableRow><TableCell><code>.data(..)</code></TableCell><TableCell><code>Vec&lt;(&str, Vec&lt;f32&gt;)&gt;</code></TableCell><TableCell>(category, per-series values); donut rows are slices.</TableCell></TableRow>
              <TableRow><TableCell><code>.indicator(..)</code></TableCell><TableCell><code>IndicatorStyle</code></TableCell><TableCell>Tooltip indicator: Dot (default) | Line.</TableCell></TableRow>
              <TableRow><TableCell><code>.legend(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Legend row with series totals.</TableCell></TableRow>
              <TableRow><TableCell><code>.center_label(..)</code></TableCell><TableCell><code>&str</code></TableCell><TableCell>Donut hole label (e.g. total).</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark theme (colors read from series' dark slots).</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/chart" target="_blank" rel="noreferrer">shadcn/ui Chart (Recharts) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
            <div><dt>Examples</dt><dd>5</dd></div>
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
