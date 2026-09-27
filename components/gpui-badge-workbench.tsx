"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState } from "react";
import { CodeBlock } from "@/components/code-block";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const variants = [
  {
    id: "variants",
    label: "Variants",
    hint: "Default / Secondary / Destructive / Outline / Ghost / Link 六种变体，一行排开。",
    code: `Badge::new().label("Default")
Badge::new().label("Secondary").variant(BadgeVariant::Secondary)
Badge::new().label("Destructive").variant(BadgeVariant::Destructive)
Badge::new().label("Outline").variant(BadgeVariant::Outline)
Badge::new().label("Ghost").variant(BadgeVariant::Ghost)
Badge::new().label("Link").variant(BadgeVariant::Link)`,
  },
  {
    id: "icons",
    label: "Icons",
    hint: "内联图标对应 data-icon=\"inline-start|end\" —— Check、Bookmark、尾部 Check。",
    code: `Badge::new()
    .label("Verified")
    .variant(BadgeVariant::Secondary)
    .icon(BadgeIcon::Check, BadgeIconPos::Start)

Badge::new()
    .label("Bookmark")
    .variant(BadgeVariant::Outline)
    .icon(BadgeIcon::Bookmark, BadgeIconPos::Start)`,
  },
  {
    id: "spinner",
    label: "Spinner",
    hint: "BadgeIcon::Spinner 画 240° 旋转弧线，reduced-motion 下静止。",
    code: `Badge::new()
    .label("Deleting")
    .variant(BadgeVariant::Secondary)
    .icon(BadgeIcon::Spinner, BadgeIconPos::Start)

Badge::new()
    .label("Generating")
    .icon(BadgeIcon::Spinner, BadgeIconPos::Start)`,
  },
  {
    id: "link",
    label: "Link",
    hint: ".clickable(true) 的 Link 变体：可聚焦、可点击，按下发出 BadgePressEvent。",
    code: `Badge::new()
    .label("Open Link")
    .variant(BadgeVariant::Link)
    .icon(BadgeIcon::ArrowUpRight, BadgeIconPos::End)
    .clickable(true)

// emits BadgePressEvent { label } on press`,
  },
  {
    id: "custom",
    label: "Custom colors",
    hint: ".tone(..) 覆盖 variant 配色：Blue / Green / Sky / Purple / Red 五套 tint。",
    code: `Badge::new()
    .label("Green")
    .variant(BadgeVariant::Secondary)
    .tone(BadgeTone::Green)

Badge::new()
    .label("Purple")
    .variant(BadgeVariant::Secondary)
    .tone(BadgeTone::Purple)`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiBadgeWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("variants");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Badge GPUI examples">
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
                src={`/gpui-command/index.html?demo=badge&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} badge — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="badge.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
