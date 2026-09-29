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
    hint: "按钮触发器 + Popover 内嵌 Calendar：「Pick a date」占位，选中后回显长格式日期并关闭弹层。",
    code: `let calendar = cx.new(|_| Calendar::new());
let picker = cx.new(|cx| DatePicker::new(Some(calendar), window, cx));
cx.subscribe(&picker, |_, _, ev: &DatePickerEvent, cx| {
    // ev.selection — Option<DatePick>
    cx.notify();
})
.detach();`,
  },
  {
    id: "range",
    label: "Range",
    hint: "mode(Range) + months(2)：首点定起点、次点定终点（自动交换）并关闭弹层，回显「start – end」。",
    code: `let calendar = cx.new(|_| {
    Calendar::new()
        .mode(CalendarMode::Range)
        .months(2)
});
DatePicker::new(Some(calendar), window, cx)
    .placeholder("Pick a date range")
// mid-range the popover stays open; end pick closes it`,
  },
  {
    id: "dob",
    label: "Date of Birth",
    hint: "生日场景：label + dropdown caption（月/年直跳），初始视图锚定 1990 年。",
    code: `let calendar = cx.new(|_| {
    Calendar::new()
        .dropdown(true)
        .visible_month(1990, 1)
});
DatePicker::new(Some(calendar), window, cx)
    .label("Date of birth")
    .placeholder("Select date")`,
  },
  {
    id: "input",
    label: "With Input",
    hint: "真实输入框：键入 YYYY-MM-DD 或 M/D/YYYY 即时解析并同步选中态；日历图标单独开关弹层。",
    code: `let input = cx.new(|cx| {
    InputState::new(window, cx)
        .placeholder("Select date".to_string())
});
DatePicker::new(Some(calendar), window, cx)
    .label("Subscription Date")
    .date_input(input, cx)
// typing 2026-09-30 highlights the day; picking
// a day writes the ISO date back into the field`,
  },
  {
    id: "time",
    label: "Date & Time",
    hint: "弹层里日历 + Time 行：日期与时间各自解析，DatePickerEvent 同时携带两者。",
    code: `let time = cx.new(|cx| {
    InputState::new(window, cx).placeholder("9:00".to_string())
});
DatePicker::new(Some(calendar), window, cx)
    .label("Date")
    .time_input(time, cx)
// DatePickerEvent { selection, time: Option<(h, m)> }`,
  },
  {
    id: "natural",
    label: "Natural Language",
    hint: "自然语言输入：Enter 解析 tomorrow / in 3 days / next Friday / 2026-09-30 并回显一句话确认。",
    code: `let input = cx.new(|cx| {
    InputState::new(window, cx).placeholder("E.g. tomorrow".to_string())
});
DatePicker::new(None, window, cx)   // input-only, no calendar
    .label("Schedule Date")
    .natural_input(input, cx)
// "in 2 weeks" → today + 14d →
// "Your post will be published on October 12, 2026."`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiDatePickerWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Date picker GPUI examples">
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
          <div className="workbench-preview is-date-picker" data-preview-theme={dark ? "dark" : "light"}>
            <div className="preview-stage">
              <iframe
                key={`${variant}-${dark}-${replay}`}
                className="gpui-action-frame"
                src={`/gpui-command/index.html?demo=date-picker&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} date picker — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="date_picker.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
