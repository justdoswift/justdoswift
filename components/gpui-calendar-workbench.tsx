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
    hint: "单月日历：‹ › 翻月、outside 日期灰显、today 与选中态用 primary 底色。",
    code: `let cal = cx.new(|_| {
    Calendar::new()
        .selected(Date::today())
});
// host renders cal.clone() — clicks emit CalendarSelectEvent`,
  },
  {
    id: "range",
    label: "Range",
    hint: "RangeCalendar：.mode(Range) + .months(2) 双月并排，首点定起点、次点定终点、第三点重开。",
    code: `Calendar::new()
    .mode(CalendarMode::Range)
    .months(2)
    .range(today.add_days(4), today.add_days(11))`,
  },
  {
    id: "dropdown",
    label: "Month/Year",
    hint: "captionLayout=\"dropdown\" 同款：caption 变成月份/年份下拉，点开后直接跳转。",
    code: `Calendar::new()
    .selected(Date::today())
    .dropdown(true)
// caption renders "September ⌄  2026 ⌄" — clicking opens
// a month (4×3) or year (±5) picker overlay`,
  },
  {
    id: "presets",
    label: "Presets",
    hint: "日历 + 预设快捷列组合：点预设直接选中相对日期并跳到对应月。",
    code: `// demo composes a ghost-button column next to the entity
div().flex_row().gap(px(16.))
    .child(calendar.clone())
    .child(preset_column(dark, weak))
// each preset: today.add_days(n) → selected + visible month`,
  },
  {
    id: "datetime",
    label: "Date & Time",
    hint: "Date and time picker 组合：日历选日期，右侧 Hours/Minutes 用 ▲▼ stepper 调整。",
    code: `// calendar entity + stepper column; steppers mutate
// host state (this.time) with wrap-around 0..24 / 0..60
div().flex_row().gap(px(16.))
    .child(calendar.clone())
    .child(time_column(dark, self.time, weak))`,
  },
  {
    id: "booked",
    label: "Booked",
    hint: "disabled_dates 列表内的日期不可选（半透明 + 拦截点击），booking 场景的已订日期。",
    code: `Calendar::new()
    .disabled_dates(vec![
        today.add_days(3),
        today.add_days(5),
        today.add_days(6),
        today.add_days(11),
        today.add_days(12),
    ])`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCalendarWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Calendar GPUI examples">
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
          <div className="workbench-preview" data-preview-theme={dark ? "dark" : "light"}>
            <div className="preview-stage">
              <iframe
                key={`${variant}-${dark}-${replay}`}
                className="gpui-action-frame"
                src={`/gpui-command/index.html?demo=calendar&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} calendar — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="calendar.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
