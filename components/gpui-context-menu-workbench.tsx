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
    hint: "在虚线框里右键 —— 菜单在指针处弹出，点击外部或 Escape 关闭。",
    code: `ContextMenu::new(
    || trigger_element(),   // any renderable area
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Profile")),
        ContextMenuEntry::Item(ContextMenuItem::new("Billing")),
        ContextMenuEntry::Item(ContextMenuItem::new("Team")),
        ContextMenuEntry::Item(ContextMenuItem::new("Subscription")),
    ],
    cx,
)`,
  },
  {
    id: "submenu",
    label: "Submenu",
    height: 480,
    hint: "ContextMenuItem::submenu(..) 挂二级菜单 —— hover 或 → 打开，← 收起。",
    code: `ContextMenuItem::new("More Tools").submenu(vec![
    ContextMenuEntry::Item(
        ContextMenuItem::new("Save Page As…").shortcut("⌘S"),
    ),
    ContextMenuEntry::Item(
        ContextMenuItem::new("Create Shortcut…"),
    ),
    ContextMenuEntry::Separator,
    ContextMenuEntry::Item(
        ContextMenuItem::new("Developer Tools"),
    ),
])`,
  },
  {
    id: "shortcuts",
    label: "Shortcuts",
    height: 420,
    hint: "ContextMenuItem::shortcut(..) 在行尾渲染 ⌘X / ⇧⌘Z 等快捷键提示（修饰键手绘）。",
    code: `ContextMenuItem::new("Undo").shortcut("⌘Z")
ContextMenuItem::new("Redo").shortcut("⇧⌘Z")
ContextMenuItem::new("Cut").shortcut("⌘X")
ContextMenuItem::new("Copy").shortcut("⌘C")
ContextMenuItem::new("Paste").shortcut("⌘V")`,
  },
  {
    id: "groups",
    label: "Groups",
    height: 460,
    hint: "ContextMenuEntry::Label + Separator 组成分组菜单。",
    code: `vec![
    ContextMenuEntry::Label("File".into()),
    ContextMenuEntry::Item(ContextMenuItem::new("New File")),
    ContextMenuEntry::Item(ContextMenuItem::new("Duplicate")),
    ContextMenuEntry::Separator,
    ContextMenuEntry::Label("Edit".into()),
    ContextMenuEntry::Item(ContextMenuItem::new("Rename")),
    ContextMenuEntry::Item(ContextMenuItem::new("Move to Trash")),
]`,
  },
  {
    id: "icons",
    label: "Icons",
    height: 420,
    hint: "ContextMenuItem::icon(..) 行首图标，PathBuilder 手绘。",
    code: `ContextMenuItem::new("Back").icon(MenuIcon::ArrowLeft)
ContextMenuItem::new("Reload").icon(MenuIcon::Rotate)
ContextMenuItem::new("Copy Link").icon(MenuIcon::Share)
ContextMenuItem::new("Settings").icon(MenuIcon::Gear)`,
  },
  {
    id: "selection",
    label: "Selection",
    height: 460,
    hint: "checkbox(true) 多选开关 + radio(group) 单选互斥 —— 提交后菜单保持打开。",
    code: `ContextMenuItem::new("Show Sidebar").checkbox(true)
ContextMenuItem::new("Show Minimap").checkbox(true)

// radio group "theme" — exclusive
ContextMenuItem::new("System").radio("theme")
ContextMenuItem::new("Light").radio("theme")
ContextMenuItem::new("Dark").radio("theme")

menu.read(cx).checked("Show Sidebar");
menu.read(cx).radio_value("theme");`,
  },
  {
    id: "destructive",
    label: "Destructive",
    height: 420,
    hint: "destructive(true) — 红色文案 + 红色 hover，用于 Delete 等危险操作。",
    code: `ContextMenuItem::new("Delete")
    .icon(MenuIcon::Trash)
    .shortcut("⌘⌫")
    .destructive(true)`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiContextMenuWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Context menu GPUI examples">
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
                src={`/gpui-command/index.html?demo=context-menu&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} context menu — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="context_menu.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
