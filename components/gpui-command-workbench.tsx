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
    height: 420,
    hint: "cmdk 风格命令面板：顶部搜索框 + 常驻过滤列表，↑/↓ 移动高亮，Enter/点击执行。",
    code: `let input = cx.new(|cx| {
    InputState::new(window, cx)
        .placeholder("Type a command or search…")
});

let command = cx.new(|cx| {
    Command::new(input, cx)
        .items(vec![
            CommandItem::new("Calendar").group("Suggestions"),
            CommandItem::new("Search Emoji").group("Suggestions"),
            CommandItem::new("Profile").group("Settings"),
            // …
        ])
});

div().child(command)`,
  },
  {
    id: "shortcuts",
    label: "Shortcuts",
    height: 420,
    hint: "CommandItem::shortcut(..) 在行尾渲染 ⌘P / ⌘B / ⌘S 样式的快捷键标签。",
    code: `CommandItem::new("Profile")
    .group("Settings")
    .shortcut("⌘P")

CommandItem::new("Billing")
    .group("Settings")
    .shortcut("⌘B")`,
  },
  {
    id: "groups",
    label: "Groups",
    height: 460,
    hint: "CommandItem::group(..) 自动插入组头 + 分隔线，组名也参与过滤。",
    code: `Command::new(input, cx).items(vec![
    CommandItem::new("New File").group("File"),
    CommandItem::new("Save").group("File"),
    CommandItem::new("Preferences").group("Preferences"),
    CommandItem::new("Color Theme").group("Preferences"),
    CommandItem::new("Toggle Terminal").group("View"),
    // …
])`,
  },
  {
    id: "scrollable",
    label: "Scrollable",
    height: 460,
    hint: "list_max_h(px(224.)) 限制列表高度，超高内容滚动，高亮行自动滚入视野。",
    code: `Command::new(input, cx)
    .items(all_commands())
    .list_max_h(px(224.))

// ↑/↓ moves the highlight; the row
// scrolls into view automatically.`,
  },
  {
    id: "dialog",
    label: "Dialog",
    height: 480,
    hint: "CommandDialog — 半透明遮罩 + 居中浮层，deferred 层保证画在所有内容之上。",
    code: `// Wrap Command in a scrim + deferred
// overlay to get CommandDialog.
deferred(
    div()
        .absolute().inset_0()
        .bg(scrim)
        .flex().items_center().justify_center()
        .child(
            div().w(px(380.))
                .rounded_lg().shadow_lg()
                .child(command)
        ),
)`,
  },
  {
    id: "empty",
    label: "Empty",
    height: 380,
    hint: "无匹配 / 空 collection 时显示 .empty_text(..)（CommandEmpty）。",
    code: `Command::new(input, cx)
    .items(vec![])
    .empty_text("Nothing here yet.")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCommandWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Command GPUI examples">
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
                src={`/gpui-command/index.html?demo=command&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} command palette — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="command.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
