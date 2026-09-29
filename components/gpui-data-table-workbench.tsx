"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState } from "react";
import { CodeBlock } from "@/components/code-block";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const variants = [
  {
    id: "basic",
    label: "Basic",
    height: 560,
    hint: "纯表格：列定义 + 行数据，固定宽列与弹性列混排，超高自动滚动。",
    code: `let columns = vec![
    DataColumn::new("status", "Status").width(px(110.)),
    DataColumn::new("email", "Email"),             // flex column
    DataColumn::new("amount", "Amount").numeric(true).width(px(110.)),
];

let rows = vec![
    DataRow::new("728ed52f", vec!["success".into(), "ken99@example.com".into(), "$316.00".into()]),
    // ...
];

cx.new(|cx| DataTable::new(columns, rows, cx))`,
  },
  {
    id: "sorting",
    label: "Sorting",
    height: 560,
    hint: "sortable(true) 列头可点 —— 点击循环 升序 → 降序 → 原序；numeric(true) 按数值排序。",
    code: `DataColumn::new("email", "Email").sortable(true)
DataColumn::new("amount", "Amount")
    .numeric(true)     // "$316.00" → 316.0，按数值比较
    .sortable(true)

// 表头手绘 ↑/↓/↕ 指示当前方向`,
  },
  {
    id: "filtering",
    label: "Filtering",
    height: 600,
    hint: "filter_input(input, Some(col)) 接工具栏搜索框 —— 输入实时过滤指定列。",
    code: `let input = cx.new(|cx| {
    InputState::new(window, cx).placeholder("Filter emails…".to_string())
});

DataTable::new(columns, rows, cx)
    .filter_input(input, Some(1), cx)  // filter the Email column
// filter_input(input, None, cx)     —— 匹配所有列`,
  },
  {
    id: "pagination",
    label: "Pagination",
    height: 460,
    hint: "paginate(page_size) —— 页脚 Previous/Next + Page x of y，边界自动禁用。",
    code: `DataTable::new(columns, rows, cx)
    .paginate(5)   // 5 rows per page → 12 rows = 3 pages`,
  },
  {
    id: "selection",
    label: "Selection",
    height: 480,
    hint: "selection(true) —— 行首复选列 + 表头全选（部分选中时 indeterminate）+ 页脚计数。",
    code: `DataTable::new(columns, rows, cx)
    .selection(true)
    .paginate(5)

// 随时读取选中项：
table.read(cx).selected_ids()  // Vec<SharedString>`,
  },
  {
    id: "actions",
    label: "Row actions",
    height: 540,
    hint: "actions(&[..]) —— 行尾 ⋯ 列弹出操作菜单，发出 DataTableActionEvent。",
    code: `DataTable::new(columns, rows, cx)
    .actions(&["Copy payment ID", "View customer", "View payment details"])

cx.subscribe(&table, |view, _, ev: &DataTableActionEvent, cx| {
    // ev.action = 菜单项文本, ev.row = 行 id
    view.handle(ev.row.clone());
    cx.notify();
});`,
  },
  {
    id: "full",
    label: "Full featured",
    height: 600,
    hint: "shadcn 示例全特性：过滤 + 列可见性 + 排序 + 复选 + 分页 + 行操作。",
    code: `DataTable::new(columns, rows, cx)
    .filter_input(input, Some(1), cx)
    .selection(true)
    .actions(&["Copy payment ID", "View customer", "View payment details"])
    .column_visibility(true)   // "Columns ▾" dropdown
    .paginate(5)`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiDataTableWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("full");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Data table GPUI examples">
      <div className="example-heading"><h2>Overview</h2></div>
      <Tabs value={variant} onValueChange={(value) => setVariant(value as VariantId)}>
        <TabsList className="mb-2 h-auto w-full flex-wrap justify-start gap-1.5 rounded-none bg-transparent p-0">
          {variants.map((item) => (
            <TabsTrigger
              key={item.id}
              value={item.id}
              className="h-auto flex-none rounded-md px-3 py-1.5 text-xs data-[state=active]:border-border data-[state=active]:bg-secondary data-[state=active]:shadow-none"
            >
              {item.label}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>
      <p className="gpui-accordion-hint">{selected.hint}</p>
      <Tabs value={tab} onValueChange={(value) => setTab(value as ContentTab)}>
        <div className="workbench-toolbar">
          <TabsList className="rounded-full">
            <TabsTrigger value="preview" className="rounded-full px-4">Preview</TabsTrigger>
            <TabsTrigger value="code" className="rounded-full px-4">Code</TabsTrigger>
          </TabsList>
          <div className="workbench-actions">
            {tab === "preview" && (
              <>
                <Button variant="ghost" size="icon" onClick={() => setDark((value) => !value)} aria-label={dark ? "Light preview" : "Dark preview"} title="Toggle preview background">
                  {dark ? <Sun /> : <Moon />}
                </Button>
                <Button variant="ghost" size="icon" onClick={() => setReplay((value) => value + 1)} aria-label="Replay preview" title="Replay preview">
                  <RotateCcw />
                </Button>
              </>
            )}
            <Button variant="ghost" size="icon" asChild>
              <a href="/gpui-command/source.zip" download aria-label="Download runnable Rust example" title="Download runnable Rust example">
                <Download />
              </a>
            </Button>
          </div>
        </div>
        <TabsContent value="preview" className="workbench-body">
          <div className="workbench-preview" style={{ height: selected.height }} data-preview-theme={dark ? "dark" : "light"}>
            <div className="preview-stage">
              <iframe
                key={`${variant}-${dark}-${replay}`}
                className="gpui-action-frame"
                src={`/gpui-command/index.html?demo=data-table&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} data table — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="data_table.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
