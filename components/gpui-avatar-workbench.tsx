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
    hint: "三种装载路径：内嵌照片、首字母 fallback、以及图片加载失败 → fallback。",
    code: `Avatar::new()
    .image(AvatarImage::Photo(0))
    .initials("CN")`,
  },
  {
    id: "badge",
    label: "Badge",
    hint: "右下角状态点：.badge(AvatarBadge::Dot)，带页面色描边。",
    code: `Avatar::new()
    .image(AvatarImage::Photo(1))
    .initials("CN")
    .badge(AvatarBadge::Dot)`,
  },
  {
    id: "badge-icon",
    label: "Badge icon",
    hint: "Badge 里也能放图标 —— .badge(AvatarBadge::Icon) 画一个 + 号。",
    code: `Avatar::new()
    .image(AvatarImage::Photo(2))
    .initials("PP")
    .badge(AvatarBadge::Icon)`,
  },
  {
    id: "group",
    label: "Group",
    hint: "负外边距叠压成一列，每个头像带页面色描边分隔，对应 shadcn 的 AvatarGroup。",
    code: `// -10px overlap + page-colored ring on each avatar
div().flex().items_center().children(
    self.members.iter().enumerate().map(|(i, m)| {
        let a = Avatar::new().image(m.image).initials(m.initials)
            .ring(self.surface);
        if i == 0 { div().child(a) } else { div().ml(px(-10.)).child(a) }
    }),
)`,
  },
  {
    id: "group-count",
    label: "Group count",
    hint: "组尾再叠一个 +N 计数圆，对应 AvatarGroupCount。",
    code: `// trailing +N circle shares the group's ring + overlap
div()
    .ml(px(-10.)).size(px(32.)).rounded(px(9999.))
    .border_2().border_color(self.surface)
    .child("+3")`,
  },
  {
    id: "group-icon",
    label: "Group icon",
    hint: "AvatarGroupCount 里也能放图标 —— 计数圆换成 + 号。",
    code: `// same slot, icon instead of "+3"
div()
    .ml(px(-10.)).size(px(32.)).rounded(px(9999.))
    .border_2().border_color(self.surface)
    .child(plus_icon(ink))`,
  },
  {
    id: "sizes",
    label: "Sizes",
    hint: "sm 24 / default 32 / lg 40 三档直径，badge 与字号同步缩放。",
    code: `Avatar::new()
    .size(AvatarSize::Lg)
    .image(AvatarImage::Photo(3))
    .initials("CN")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAvatarWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Avatar GPUI examples">
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
                src={`/gpui-command/index.html?demo=avatar&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} avatar — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="avatar.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
