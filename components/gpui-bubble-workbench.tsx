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
    hint: "七种视觉变体：default / secondary / muted / tinted / outline / destructive / ghost（无框全宽）。",
    code: `Bubble::new("This is the default primary bubble.")
    .variant(BubbleVariant::Default)

Bubble::new("This one is tinted with primary color.")
    .variant(BubbleVariant::Tinted)

Bubble::new("Ghost bubbles work for assistant text.")
    .variant(BubbleVariant::Ghost)`,
  },
  {
    id: "alignment",
    label: "Alignment",
    hint: ".align(BubbleAlign::End) 把气泡贴到右侧 —— 用户消息；Start 为默认接收方。",
    code: `Bubble::new("Aligned to the end — user messages.")
    .variant(BubbleVariant::Default)
    .align(BubbleAlign::End)`,
  },
  {
    id: "group",
    label: "Bubble group",
    hint: "同一发送方的连续消息用小间距堆叠，对应 BubbleGroup。",
    code: `// column with a small gap = BubbleGroup
div().flex().flex_col().items_start().gap_2().children(
    thread.iter().map(|text| bubble(text)),
)

Bubble::new("Find the bug and fix it.")
    .reactions(vec!["👀".into()], BubbleAlign::Start)`,
  },
  {
    id: "reactions",
    label: "Reactions",
    hint: ".reactions(..) 在气泡下缘叠加 emoji chip 条，side 控制停靠边。",
    code: `Bubble::new("Bold. I'll add some tests.")
    .variant(BubbleVariant::Default)
    .align(BubbleAlign::End)
    .reactions(vec!["👀".into(), "🚀".into(), "+2".into()],
               BubbleAlign::End)`,
  },
  {
    id: "buttons",
    label: "Buttons",
    hint: ".pressable(true) 让气泡变成可聚焦按钮，点击发出 BubblePressEvent —— 对应 render={<button/>}。",
    code: `Bubble::new("I forgot my password")
    .variant(BubbleVariant::Outline)
    .pressable(true)

// host view:
cx.subscribe(&bubble, |demo, _, event: &BubblePressEvent, cx| {
    demo.answer(event.label.clone());
    cx.notify();
});`,
  },
  {
    id: "collapsible",
    label: "Collapsible",
    hint: "长文本先显示 preview + Show more，点击就地展开为全文 / Show less。",
    code: `Bubble::new(long_text)
    .variant(BubbleVariant::Secondary)
    .collapsible("The accessibility review found two
        focus states that were visually too subtle…")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiBubbleWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("variants");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Bubble GPUI examples">
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
                src={`/gpui-command/index.html?demo=bubble&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} bubble — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="bubble.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
