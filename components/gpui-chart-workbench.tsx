"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState } from "react";
import { CodeBlock } from "@/components/code-block";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const variants = [
  {
    id: "bar",
    label: "Bar",
    height: 480,
    hint: "分组柱状 + CartesianGrid + X 轴 + 悬停 tooltip + 带 totals 的 legend（文档页同款交互）。",
    code: `Chart::new(ChartKind::Bar)
    .title("Bar Chart - Interactive", "…last 6 months")
    .config(vec![
        ChartSeries::new("Desktop", 0x2563eb, 0x3b82f6),
        ChartSeries::new("Mobile", 0x60a5fa, 0x93c5fd),
    ])
    .data(vec![
        ("Jan", vec![186., 80.]),
        ("Feb", vec![305., 200.]),
        // …
    ])`,
  },
  {
    id: "stacked",
    label: "Stacked",
    height: 480,
    hint: "ChartKind::Stacked —— 每类单列、按系列堆叠，y 轴按列总和归一。",
    code: `Chart::new(ChartKind::Stacked)
    .config(dm_series())
    .data(month_data())`,
  },
  {
    id: "line",
    label: "Line",
    height: 480,
    hint: "折线 + 顶点圆点，canvas 直接画 PathBuilder::stroke 折线。",
    code: `Chart::new(ChartKind::Line)
    .config(dm_series())
    .data(month_data())
    .indicator(IndicatorStyle::Line)`,
  },
  {
    id: "area",
    label: "Area",
    height: 480,
    hint: "折线下方渐变填充（linear_gradient 90° 从 40% 到 2% alpha）。",
    code: `Chart::new(ChartKind::Area)
    .config(dm_series())
    .data(month_data())`,
  },
  {
    id: "donut",
    label: "Donut",
    height: 480,
    hint: "PathBuilder::arc_to 画环形切片，中心总数；悬停按角度命中切片，其余变暗。",
    code: `Chart::new(ChartKind::Donut)
    .config(vec![
        ChartSeries::new("Chrome", 0xe76e50, 0x5b4bc4),
        // …Safari / Firefox / Edge / Other
    ])
    .data(vec![("Chrome", vec![275.]), /* … */])
    .center_label("925")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiChartWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("bar");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Chart GPUI examples">
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
                src={`/gpui-command/index.html?demo=chart&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} chart — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="chart.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
