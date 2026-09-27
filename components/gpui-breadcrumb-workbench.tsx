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
    hint: "Home / Components / Breadcrumb —— link 可点击，末级 Page 是纯文本。",
    code: `Breadcrumb::new().items(vec![
    CrumbItem::link("Home"),
    CrumbItem::link("Components"),
    CrumbItem::page("Breadcrumb"),
])`,
  },
  {
    id: "separator",
    label: "Custom separator",
    hint: "对应 BreadcrumbSeparator 的自定义 children —— .separator(CrumbSeparator::Dot)。",
    code: `Breadcrumb::new()
    .separator(CrumbSeparator::Dot)
// Chevron | Slash | Dot — all drawn with PathBuilder`,
  },
  {
    id: "collapsed",
    label: "Collapsed",
    hint: "CrumbItem::Ellipsis 渲染 BreadcrumbEllipsis 风格的 … 标记，可点击发出导航事件。",
    code: `Breadcrumb::new().items(vec![
    CrumbItem::link("Home"),
    CrumbItem::Ellipsis,
    CrumbItem::link("Components"),
    CrumbItem::page("Breadcrumb"),
])`,
  },
  {
    id: "link",
    label: "Link",
    hint: "每条 link 可 Tab 聚焦、hover 加深、点击发出 BreadcrumbNavigateEvent。",
    code: `cx.subscribe(&breadcrumb, |demo, _, event: &BreadcrumbNavigateEvent, cx| {
    demo.navigate(event.label.clone());
    cx.notify();
});
// emits on click -> "Navigate to Home"`,
  },
  {
    id: "rtl",
    label: "RTL",
    hint: ".rtl(true) 反转条目顺序并把 chevron 指向左侧，对应阿拉伯语排版。",
    code: `Breadcrumb::new().rtl(true)
// items render reversed; chevrons mirror to point left`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiBreadcrumbWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Breadcrumb GPUI examples">
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
                src={`/gpui-command/index.html?demo=breadcrumb&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} breadcrumb — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="breadcrumb.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
