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
    hint: "Default / Secondary / Destructive / Outline / Ghost / Link 六种变体，hover/active/focus-visible 都有反馈。",
    code: `Button::new("id").label("Default")
Button::new("id").label("Secondary").variant(ButtonVariant::Secondary)
Button::new("id").label("Destructive").variant(ButtonVariant::Destructive)
Button::new("id").label("Outline").variant(ButtonVariant::Outline)
Button::new("id").label("Ghost").variant(ButtonVariant::Ghost)
Button::new("id").label("Link").variant(ButtonVariant::Link)`,
  },
  {
    id: "sizes",
    label: "Sizes",
    hint: "Xs / Sm / Default / Lg 四种文字尺寸；icon 尺寸在 Icons 示例里。",
    code: `Button::new("id").label("Extra Small").size(ButtonSize::Xs)
Button::new("id").label("Small").size(ButtonSize::Sm)
Button::new("id").label("Default")
Button::new("id").label("Large").size(ButtonSize::Lg)`,
  },
  {
    id: "icons",
    label: "Icons",
    hint: ".icon(icon, pos) 对应 data-icon=\"inline-start|end\"；icon-only 尺寸带 a11y_label。",
    code: `Button::new("id")
    .label("New Branch")
    .icon(ButtonIcon::GitBranch, IconPos::Start)

Button::new("id")
    .size(ButtonSize::Icon)
    .variant(ButtonVariant::Outline)
    .icon(ButtonIcon::ArrowUp, IconPos::Start)
    .a11y_label("Move up")`,
  },
  {
    id: "rounded",
    label: "Rounded",
    hint: ".pill(true) 对应 rounded-full —— 文字按钮与圆形图标按钮。",
    code: `Button::new("id")
    .label("Get Started")
    .pill(true)
    .icon(ButtonIcon::ArrowUpRight, IconPos::End)

Button::new("id")
    .pill(true)
    .size(ButtonSize::Icon)
    .icon(ButtonIcon::Plus, IconPos::Start)`,
  },
  {
    id: "spinner",
    label: "Spinner",
    hint: "ButtonIcon::Spinner 画 240° 旋转弧线；disabled 时按钮不响应输入。",
    code: `Button::new("id")
    .label("Generating")
    .icon(ButtonIcon::Spinner, IconPos::Start)
    .disabled(true)

Button::new("id")
    .label("Downloading")
    .variant(ButtonVariant::Secondary)
    .icon(ButtonIcon::Spinner, IconPos::Start)
    .disabled(true)`,
  },
  {
    id: "group",
    label: "Button Group",
    hint: ".join(Start|Middle|End) 抹平内侧圆角并 -1px 合并描边，三个按钮连成一段。",
    code: `Button::new("id").label("Archive")
    .variant(ButtonVariant::Outline).join(ButtonJoin::Start)
Button::new("id").label("Report")
    .variant(ButtonVariant::Outline).join(ButtonJoin::Middle)
Button::new("id").label("Snooze")
    .variant(ButtonVariant::Outline).join(ButtonJoin::End)`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiButtonWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("variants");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Button GPUI examples">
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
                src={`/gpui-command/index.html?demo=button&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} button — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="button.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
