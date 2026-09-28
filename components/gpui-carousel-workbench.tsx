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
    height: 340,
    hint: "视口 overflow-hidden + flex track 平移，两侧圆形 outline 按钮（起始位 prev 禁用），点按发射 Slide X of N。",
    code: `let carousel = cx.new(|cx| {
    Carousel::new(cx)
        .dark(dark)
        .count(5)          // 5 auto slides, 336×200 each
        .slide_px(336.)
        .cross_px(200.)
});
// subscribe for the current-slide API
cx.subscribe(&carousel, |_, _, e: &CarouselSelectEvent, _| {
    log(e.index, e.count);
}).detach();`,
  },
  {
    id: "sizes",
    label: "Sizes",
    height: 340,
    hint: ".per_view(3) —— shadcn 的 basis-1/3：视口装三张，翻到末尾自动停。",
    code: `Carousel::new(cx)
    .count(5)
    .per_view(3)         // three slides per viewport
    .slide_px(104.)
    .gap(16.)`,
  },
  {
    id: "spacing",
    label: "Spacing",
    height: 340,
    hint: ".gap(32) —— shadcn 的 pl-* 间距版：每张 slide 之间留出可见 gutter。",
    code: `Carousel::new(cx)
    .count(5)
    .per_view(2)
    .slide_px(160.)
    .gap(32.)            // Embla's pl-8 equivalent`,
  },
  {
    id: "vertical",
    label: "Orientation",
    height: 380,
    hint: ".vertical(true) —— track 变纵列，导航按钮移到上下两端，键盘换成 ↑/↓。",
    code: `Carousel::new(cx)
    .vertical(true)      // flex-col track + top/bottom buttons
    .slide_px(160.)      // slide height along the axis
    .cross_px(320.)      // viewport width`,
  },
  {
    id: "loop",
    label: "Loop",
    height: 340,
    hint: ".loop_(true) —— Embla 的 opts.loop：过端点无缝回绕，按钮永不禁用（track 渲染两份实现连续滚动）。",
    code: `Carousel::new(cx)
    .count(5)
    .loop_(true)         // wraps past both ends seamlessly`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCarouselWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Carousel GPUI examples">
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
                src={`/gpui-command/index.html?demo=carousel&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} carousel — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="carousel.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
