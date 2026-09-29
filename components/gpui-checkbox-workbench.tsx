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
    hint: "Field 组合：label + description，整行可点；Space 键切换。",
    code: `Checkbox::new()
    .label("Accept terms and conditions")
    .description("By clicking this checkbox, you agree to the terms.")`,
  },
  {
    id: "checked",
    label: "Checked",
    hint: ".checked(true) 设定初始勾选；运行时点击或 Space 翻转，发出 CheckboxChangeEvent。",
    code: `Checkbox::new()
    .label("Enable notifications")
    .description("You can enable or disable notifications at any time.")
    .checked(true)`,
  },
  {
    id: "invalid",
    label: "Invalid",
    hint: ".invalid(true) —— isInvalid 等价物：红边框 + 红标签。",
    code: `Checkbox::new()
    .label("Accept terms and conditions")
    .invalid(true)`,
  },
  {
    id: "disabled",
    label: "Disabled",
    hint: ".disabled(true) —— 半透明、不可聚焦、点击与 Space 都不响应。",
    code: `Checkbox::new()
    .label("Enable notifications")
    .disabled(true)

Checkbox::new()
    .label("Enabled and checked")
    .checked(true)
    .disabled(true)`,
  },
  {
    id: "group",
    label: "Group",
    hint: "多个 Field 组成 checkbox 列表，分隔线对齐 shadcn 的 FieldGroup。",
    code: `// One entity per item; the host view lays them out.
Checkbox::new().label("Hard disks")
Checkbox::new().label("External disks")
Checkbox::new().label("CDs, DVDs, and iPods")
Checkbox::new().label("Connected servers").checked(true)`,
  },
  {
    id: "table",
    label: "Table",
    hint: "表头 select-all：部分选中时呈 Indeterminate 横杠，点击切换全选/全不选。",
    code: `// Rows emit CheckboxChangeEvent; the host recomputes the
// header state (all / none / indeterminate) via set_state.
let header = cx.new(|_| Checkbox::new().indeterminate());
cx.subscribe(&row, move |demo, _, ev: &CheckboxChangeEvent, cx| {
    let st = /* all? Checked : none? Unchecked : Indeterminate */;
    header.update(cx, |h, cx| h.set_state(st, cx));
});`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCheckboxWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Checkbox GPUI examples">
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
                src={`/gpui-command/index.html?demo=checkbox&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} checkbox — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="checkbox.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
