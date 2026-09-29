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
    height: 440,
    hint: "输入过滤 + 点击/Enter 选中；选中后 label 回填并可继续输入重新搜索。",
    code: `let input = cx.new(|cx| {
    InputState::new(window, cx)
        .placeholder("Select framework…")
});

let combobox = cx.new(|cx| {
    Combobox::new(input, cx)
        .items(frameworks())
        .empty_text("No framework found.")
});

div().child(combobox)`,
  },
  {
    id: "multiple",
    label: "Multiple",
    height: 460,
    hint: "多选 chips：Enter/点击 toggle，空输入时 Backspace 删掉最后一个 chip。",
    code: `Combobox::new(input, cx)
    .items(frameworks())
    .multiple(true)
    .selected_values(&["Next.js"])
    .empty_text("No framework found.")

// Backspace on the empty query pops the
// newest chip — handled via a keystroke
// interceptor before input dispatch.`,
  },
  {
    id: "clearable",
    label: "Clearable",
    height: 440,
    hint: ".clearable(true) 在有值时显示 ✕ 清空按钮。",
    code: `Combobox::new(input, cx)
    .items(frameworks())
    .clearable(true)

// Pick a value, then the ✕ button resets
// the query and the selection.`,
  },
  {
    id: "groups",
    label: "Groups",
    height: 460,
    hint: "ComboboxItem::group(..) 分组渲染，组名也参与过滤。",
    code: `Combobox::new(input, cx)
    .items(vec![
        ComboboxItem::new("Debounce").group("Utils"),
        ComboboxItem::new("Throttle").group("Utils"),
        ComboboxItem::new("Fetch").group("Network"),
        // …
    ])`,
  },
  {
    id: "custom",
    label: "Hint rows",
    height: 460,
    hint: "ComboboxItem::hint(..) 在每行右侧放次要说明（如角色）。",
    code: `Combobox::new(input, cx)
    .items(vec![
        ComboboxItem::new("Ada Lovelace").hint("Owner"),
        ComboboxItem::new("Alan Turing").hint("Reviewer"),
        ComboboxItem::new("Grace Hopper").hint("Member"),
        // …
    ])`,
  },
  {
    id: "invalid",
    label: "Invalid",
    height: 440,
    hint: ".invalid(true) — 红色边框与焦点环，用于表单校验失败态。",
    code: `Combobox::new(input, cx)
    .items(frameworks())
    .invalid(true)`,
  },
  {
    id: "disabled",
    label: "Disabled",
    height: 440,
    hint: ".disabled(true) + InputState::set_disabled — 半透明且完全不响应。",
    code: `input.update(cx, |input, cx| {
    input.set_disabled(true, cx);
});

Combobox::new(input, cx)
    .items(frameworks())
    .default_value("SvelteKit")
    .disabled(true)`,
  },
  {
    id: "empty",
    label: "Empty",
    height: 440,
    hint: "空 collection / 无匹配时都显示 .empty_text(..)（ComboboxEmpty）。",
    code: `Combobox::new(input, cx)
    .items(vec![])
    .empty_text("This collection is empty.")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiComboboxWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Combobox GPUI examples">
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
                src={`/gpui-command/index.html?demo=combobox&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} combobox — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="combobox.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
