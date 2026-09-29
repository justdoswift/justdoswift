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
    height: 380,
    hint: "卡片 trigger + 尾部 chevron，点击展开面板；MotionReveal 高度揭幕动画。",
    code: `Collapsible::new()
    .trigger("Product details")
    .content(|| div()
        .child("This panel can be expanded or collapsed…")
        .into_any_element())`,
  },
  {
    id: "open",
    label: "Open",
    height: 380,
    hint: ".open(true) 默认展开（defaultExpanded）；运行时用 set_open 受控。",
    code: `Collapsible::new()
    .trigger("Can I use this in my project?")
    .open(true)
    .content(|| div()
        .child("Yes. Free to use for personal and…")
        .into_any_element())

// controlled: coll.set_open(true, cx)`,
  },
  {
    id: "settings",
    label: "Settings",
    height: 380,
    hint: ".hint(..) 在 label 旁放一行说明文字，面板里放设置行。",
    code: `Collapsible::new()
    .trigger("Radius")
    .hint("Set the corner radius of the element.")
    .content(|| settings_rows())`,
  },
  {
    id: "file-tree",
    label: "File tree",
    height: 720,
    hint: "嵌套 Collapsible 组成文件树：chevron-right 旋转 90° + 文件夹/文件图标缩进。",
    code: `// Each folder is a Collapsible entity; its content is a
// factory returning child rows (nested folders + files).
Collapsible::new()
    .trigger("components")
    .chevron(ChevronStyle::Right)
    .icon(RowIcon::Folder)
    .open(true)
    .content(move || vec![
        div().child(ui.clone()).into_any_element(),
        file_row("login-form.tsx", dark),
    ])`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCollapsibleWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Collapsible GPUI examples">
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
                src={`/gpui-command/index.html?demo=collapsible&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} collapsible — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="collapsible.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
