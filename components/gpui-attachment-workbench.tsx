"use client";

import { Download, Moon, RotateCcw, Sun } from "lucide-react";
import { useState } from "react";
import { CodeBlock } from "@/components/code-block";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const variants = [
  {
    id: "default",
    label: "Basic",
    hint: "横向布局：图标 media + 标题/描述 + 移除按钮，对应 shadcn 的默认 Attachment 组合。点 × 发出 AttachmentRemoveEvent。",
    code: `Attachment::new()
    .media(AttachmentMedia::Image)
    .title("workspace.png")
    .description("PNG · 820 KB")`,
  },
  {
    id: "image",
    label: "Image",
    hint: "纵向布局 + 图片 media：.vertical() 把 media 堆到内容上方，适合缩略图类附件。",
    code: `Attachment::new()
    .vertical()
    .media(AttachmentMedia::Photo)
    .title("workspace.png")
    .description("PNG · 820 KB")`,
  },
  {
    id: "states",
    label: "States",
    hint: "五种上传状态：idle / uploading（进度条）/ processing（标题置灰）/ error（红色告警，文案说明失败原因）/ done。",
    code: `Attachment::new()
    .state(AttachmentState::Uploading { progress: 0.64 })
    .title("design-system.zip")
    .description("Uploading · 64%")`,
  },
  {
    id: "sizes",
    label: "Sizes",
    hint: "default / sm / xs 三档密度；xs 只保留 media + 标题。",
    code: `Attachment::new()
    .size(AttachmentSize::Sm)
    .title("Small attachment")
    .description("PDF · 2.4 MB")`,
  },
  {
    id: "group",
    label: "Group",
    hint: "横向滚动组：容器 overflow_x_scroll + ScrollHandle，滚轮/触控板可滚动，对应 shadcn 的 AttachmentGroup。",
    code: `div()
    .id("attachment-group")
    .flex()
    .gap_3()
    .overflow_x_scroll()
    .track_scroll(&self.scroll)
    .children(self.attachments.iter().cloned())`,
  },
  {
    id: "trigger",
    label: "Trigger",
    hint: "整卡触发：.openable(true) 铺一层点击遮罩发出 AttachmentOpenEvent；遮罩避开移除列，× 仍可独立点击。",
    code: `Attachment::new()
    .title("research-summary.pdf")
    .description("PDF · 1.4 MB")
    .openable(true)`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAttachmentWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Attachment GPUI examples">
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
                src={`/gpui-command/index.html?demo=attachment&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} attachment — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="attachment.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
