import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiDataTableWorkbench } from "@/components/gpui-data-table-workbench";
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

const usage = `use justdo_command::data_table::{
    DataColumn, DataRow, DataTable, DataTableActionEvent,
};

let input = cx.new(|cx| {
    InputState::new(window, cx).placeholder("Filter emails…".to_string())
});

let table = cx.new(|cx| {
    DataTable::new(
        vec![
            DataColumn::new("status", "Status").width(px(110.)),
            DataColumn::new("email", "Email").sortable(true),
            DataColumn::new("amount", "Amount").numeric(true).width(px(110.)),
        ],
        vec![
            DataRow::new("728ed52f", vec![
                "success".into(),
                "ken99@example.com".into(),
                "$316.00".into(),
            ]),
            // …
        ],
        cx,
    )
    .filter_input(input, Some(1), cx)
    .selection(true)
    .column_visibility(true)
    .actions(&["Copy payment ID", "View customer"])
    .paginate(5)
});

cx.subscribe(&table, |view, _, ev: &DataTableActionEvent, cx| {
    view.handle_row_action(ev.row.clone(), ev.action.clone());
    cx.notify();
});

div().size_full().child(table)`;

const composition = `DataTable                    entity — columns, rows, every feature flag
├── DataTableToolbar      filter InputState + "Columns ▾" visibility menu
│                         .filter_input(input, Some(col), cx)
│                         .column_visibility(true)
├── TableHeader           .sortable(true) header → click cycles ↑/↓/原序
│   ├── head checkbox     .selection(true) — all/indeterminate over filtered
│   └── ⋯ column          shown when .actions(&[..]) is non-empty
├── TableBody             DataRow { id, cells } — hover + selected accent
│   ├── row checkbox      per-row toggle (stable by row index)
│   ├── cells             parallel to columns; .numeric(true) right-aligns
│   └── row menu          ⋯ → DataTableActionEvent { action, row }
└── TableFooter           "N of M selected" / "Page x of y" + Previous·Next
                          .paginate(page_size)`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --data-table

# Other demos:
cargo run --locked -- --data-table sorting
cargo run --locked -- --data-table filtering
cargo run --locked -- --data-table selection
cargo run --locked -- --data-table full`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"

# data_table.rs is self-contained: filter uses gpui-base's
# InputState, popups go through deferred(), all glyphs are
# PathBuilder — no extra deps.`;

const pipelineNote = `// Rows flow through three stages, each opt-in:
//
//   rows ──filter──▶ filtered ──sort──▶ sorted ──paginate──▶ page
//    12          query        column     page_size          5
//
// filtered_indices() returns stable indices into rows, so a
// checkbox keeps its identity no matter how the view reorders.
//
// Query changes reset to page 0; sort cycles none→asc→desc→none;
// numeric columns parse digits out of "$1,240.00" → 1240.0.`;

const anchorNote = `// The ⋯ row menu and the Columns checklist are popups drawn
// in the deferred pass, anchored against the table's own
// bounds (captured each prepaint):
//
//   row menu   x = table right − MENU_W, y = row bottom − scroll
//   cols menu  x = table right − COLS_W, y = toolbar bottom
//
// A full-size overlay swallows outside clicks → closes both.`;

export async function DataTableDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/data_table.rs"),
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
              <BreadcrumbPage>Data Table</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Data Table</h1>
            <p>Powerful table and datagrids — sorting, filtering, pagination, row selection, column visibility and row actions.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiDataTableWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Data Table 是单个 Rust 文件（<code className="inline-code">src/data_table.rs</code>）—— 过滤框复用 gpui-base 的 <code className="inline-code">InputState</code>，弹层走 <code className="inline-code">deferred()</code> overlay pass，排序箭头/复选标记/⋯ 全部 PathBuilder 手绘。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">DataTable</code> 是实体：<code className="inline-code">DataColumn</code> 定义列（固定宽或 flex、可排序、数值列右对齐），<code className="inline-code">DataRow</code> 提供 <code className="inline-code">id + cells</code>。所有特性都是 builder 开关 —— 与 TanStack 的 features 思路一致：不启用就不渲染。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>结构对应 <code className="inline-code">Table → TableHeader + TableBody + TableFooter</code>，外加工具栏（过滤框 + 列可见性菜单）。行渲染只画可见列；表头复选框对过滤结果做 全选/半选/未选 三态。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={pipelineNote} label="Pipeline.rs" />
        </section>

        <section className="doc-section" id="interactions">
          <h2>Interactions</h2>
          <div className="implementation-notes">
            <div>
              <h3>Sorting</h3>
              <p>点击 <code className="inline-code">sortable</code> 列头循环 升序 → 降序 → 原序，箭头颜色区分激活态。数值列（<code className="inline-code">numeric(true)</code>）从文本里提取数字比较 —— <code className="inline-code">$1,240.00 &gt; $316.00</code>。</p>
            </div>
            <div>
              <h3>Popups</h3>
              <p>行操作菜单与 Columns 清单都锚定在表格自身 bounds（每帧 prepaint 捕获），滚动时位置实时校正；点击外部或再点 ⋯ 关闭。</p>
            </div>
          </div>
          <CodeBlock code={anchorNote} label="Anchoring.rs" />
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>States</h3>
              <p>表头复选框三态（unchecked / indeterminate / checked）反映过滤后的选中比例；分页按钮在边界页半透明并禁用；空结果渲染 "No results." 占位行。</p>
            </div>
            <div>
              <h3>Honest visuals</h3>
              <p>排序方向、选中行、禁用按钮、开关状态全部由绘制态驱动 —— 不依赖 DOM 语义，状态也可从实体读取（<code className="inline-code">selected_ids()</code>）。</p>
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
              <TableRow><TableCell><code>DataTable::new(..)</code></TableCell><TableCell><code>Vec&lt;DataColumn&gt;, Vec&lt;DataRow&gt;</code></TableCell><TableCell>Plain table — everything else is opt-in.</TableCell></TableRow>
              <TableRow><TableCell><code>DataColumn::new(..)</code></TableCell><TableCell><code>id, header</code></TableCell><TableCell><code>.sortable(..)</code> <code>.numeric(..)</code> <code>.width(..)</code></TableCell></TableRow>
              <TableRow><TableCell><code>DataRow::new(..)</code></TableCell><TableCell><code>id, Vec&lt;SharedString&gt;</code></TableCell><TableCell><code>cells</code> parallel to columns.</TableCell></TableRow>
              <TableRow><TableCell><code>.filter_input(..)</code></TableCell><TableCell><code>Entity&lt;InputState&gt;, Option&lt;col&gt;</code></TableCell><TableCell>Toolbar search box; live substring filter.</TableCell></TableRow>
              <TableRow><TableCell><code>.selection(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Checkbox column + footer count.</TableCell></TableRow>
              <TableRow><TableCell><code>.selected_ids()</code></TableCell><TableCell><code>-&gt; Vec&lt;SharedString&gt;</code></TableCell><TableCell>Stable row ids, view-independent.</TableCell></TableRow>
              <TableRow><TableCell><code>.paginate(..)</code></TableCell><TableCell><code>page_size</code></TableCell><TableCell>Footer pager + page slice.</TableCell></TableRow>
              <TableRow><TableCell><code>.column_visibility(..)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>"Columns ▾" dropdown checklist.</TableCell></TableRow>
              <TableRow><TableCell><code>.actions(..)</code></TableCell><TableCell><code>&amp;[&amp;str]</code></TableCell><TableCell>⋯ column + row menu items.</TableCell></TableRow>
              <TableRow><TableCell><code>DataTableActionEvent</code></TableCell><TableCell><code>{`{ action, row }`}</code></TableCell><TableCell>Menu item label + row id.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>特性组合参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/aria/data-table" target="_blank" rel="noreferrer">shadcn/ui Data Table (TanStack Table) <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础原语来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
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
          <a href="#interactions">Interactions</a>
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
