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
    hint: "相关动作并排：Archive / Report 合并成一组，Snooze 用间距分开。",
    code: `ButtonGroup::new()
    .gap(px(0.))
    .child(Button::new("archive").label("Archive")
        .variant(ButtonVariant::Outline).join(ButtonJoin::Start))
    .child(Button::new("report").label("Report")
        .variant(ButtonVariant::Outline).join(ButtonJoin::End))
    .child(div().pl(px(8.))
        .child(Button::new("snooze").label("Snooze")
            .variant(ButtonVariant::Outline)))`,
  },
  {
    id: "orientation",
    label: "Orientation",
    hint: ".vertical(true) 纵向堆叠 + .vjoin(..) 合并上下圆角与描边。",
    code: `ButtonGroup::new()
    .vertical(true)
    .gap(px(0.))
    .child(Button::new("inc").a11y_label("Increase")
        .variant(ButtonVariant::Outline).size(ButtonSize::Icon)
        .icon(ButtonIcon::Plus, IconPos::Start)
        .vjoin(ButtonJoin::Start))
    .child(Button::new("dec").a11y_label("Decrease")
        .variant(ButtonVariant::Outline).size(ButtonSize::Icon)
        .icon(ButtonIcon::Minus, IconPos::Start)
        .vjoin(ButtonJoin::End))`,
  },
  {
    id: "sizes",
    label: "Sizes",
    hint: "组内每个按钮自己声明尺寸 —— Sm / Default / Lg 与对应 icon 尺寸可混排。",
    code: `ButtonGroup::new().gap(px(0.))
    .child(Button::new("label").label("Button")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Sm)
        .join(ButtonJoin::Start))
    .child(Button::new("icon").a11y_label("Add")
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::IconSm)
        .icon(ButtonIcon::Plus, IconPos::Start)
        .join(ButtonJoin::End))`,
  },
  {
    id: "nested",
    label: "Nested",
    hint: "ButtonGroup 可以嵌套 —— 分页示例：1 2 3 一组，‹ › 翻页另一组。",
    code: `ButtonGroup::new().gap(px(16.))
    .child(
        ButtonGroup::new().gap(px(0.))
            .child(page_btn("1").join(ButtonJoin::Start))
            .child(page_btn("2").join(ButtonJoin::Middle))
            .child(page_btn("3").join(ButtonJoin::End)),
    )
    .child(
        ButtonGroup::new().gap(px(0.))
            .child(icon_btn("prev", ButtonIcon::ChevronLeft)
                .join(ButtonJoin::Start))
            .child(icon_btn("next", ButtonIcon::ChevronRight)
                .join(ButtonJoin::End)),
    )`,
  },
  {
    id: "separator",
    label: "Separator",
    hint: "同色按钮之间用 group_separator(false, ..) 画 1px 分隔线 —— outline 按钮自带描边则不需要。",
    code: `ButtonGroup::new().gap(px(0.))
    .child(Button::new("copy").label("Copy")
        .join(ButtonJoin::Start))
    .child(group_separator(false, dark))
    .child(Button::new("paste").label("Paste")
        .join(ButtonJoin::End))`,
  },
  {
    id: "split",
    label: "Split",
    hint: "Split button：主动作 + separator + icon-only 触发器（下拉菜单的挂载点）。",
    code: `ButtonGroup::new().gap(px(0.))
    .child(Button::new("publish").label("Publish")
        .variant(ButtonVariant::Secondary)
        .join(ButtonJoin::Start))
    .child(group_separator(false, dark))
    .child(Button::new("more").a11y_label("More options")
        .variant(ButtonVariant::Secondary)
        .size(ButtonSize::Icon)
        .icon(ButtonIcon::ChevronDown, IconPos::Start)
        .join(ButtonJoin::End))`,
  },
  {
    id: "input",
    label: "Input",
    hint: "Input 单元格抹右角 + 按钮抹左角，-1px 描边合并成搜索框。",
    code: `ButtonGroup::new().gap(px(0.))
    .child(fake_input("Search...", dark))
    .child(Button::new("search").label("Search")
        .variant(ButtonVariant::Outline)
        .join(ButtonJoin::End))`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiButtonGroupWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Button Group GPUI examples">
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
                src={`/gpui-command/index.html?demo=button-group&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} button group — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="button_group.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
