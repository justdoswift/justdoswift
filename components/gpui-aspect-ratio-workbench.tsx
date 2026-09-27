"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState } from "react";
import { CodeBlock } from "@/components/code-block";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const variants = [
  {
    id: "default",
    label: "16 : 9",
    hint: "横屏内容的默认比例，对应 shadcn 的 ratio={16 / 9} 示例。",
    code: `AspectRatio::new(16. / 9.)
    .rounded_lg()
    .overflow_hidden()
    .child(img("photo.png"))`,
  },
  {
    id: "square",
    label: "1 : 1",
    hint: "正方形画幅，对应 ratio={1 / 1}，常用于头像与封面。",
    code: `AspectRatio::new(1.)
    .rounded_lg()
    .overflow_hidden()
    .child(img("avatar.png"))`,
  },
  {
    id: "portrait",
    label: "9 : 16",
    hint: "竖版画幅，对应 ratio={9 / 16}，适合短视频与竖向媒体。",
    code: `AspectRatio::new(9. / 16.)
    .rounded_lg()
    .overflow_hidden()
    .child(img("poster.png"))`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAspectRatioWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Aspect Ratio GPUI examples">
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
                src={`/gpui-command/index.html?demo=aspect-ratio&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} aspect ratio — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="aspect_ratio.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
