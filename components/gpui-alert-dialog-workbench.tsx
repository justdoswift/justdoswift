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
    hint: "标题 + 描述 + Cancel/Continue 的基础确认框，点击 Show Dialog 打开。",
    code: `AlertDialogSpec::new()
    .trigger("Show Dialog")
    .title("Are you absolutely sure?")
    .description("This action cannot be undone.")
    .cancel("Cancel")
    .confirm("Continue")`,
  },
  {
    id: "sm",
    label: "Small",
    hint: "size(AlertDialogSize::Sm) 收窄弹层，按钮整行堆叠，对应 size=\"sm\"。",
    code: `AlertDialogSpec::new()
    .size(AlertDialogSize::Sm)
    .title("Allow app to access your location?")
    .description("This lets the app show nearby places.")`,
  },
  {
    id: "media",
    label: "Media",
    hint: "media(..) 在标题上方加图标块，对应 shadcn 的 AlertDialogMedia。",
    code: `AlertDialogSpec::new()
    .trigger("Share Project")
    .media(AlertDialogMedia::Plus)
    .title("Share this project?")
    .description("Anyone with the link can view this project.")`,
  },
  {
    id: "sm-media",
    label: "Small + Media",
    hint: "Sm 尺寸加图标块的组合变体，对应 Small with Media 示例。",
    code: `AlertDialogSpec::new()
    .trigger("Pair Device")
    .size(AlertDialogSize::Sm)
    .media(AlertDialogMedia::Bluetooth)
    .title("Pair with this device?")`,
  },
  {
    id: "destructive",
    label: "Destructive",
    hint: "destructive(true) 把确认按钮染红，配合 Trash 图标用于删除确认。",
    code: `AlertDialogSpec::new()
    .trigger("Delete Chat")
    .media(AlertDialogMedia::Trash)
    .destructive(true)
    .title("Delete this chat?")
    .confirm("Delete")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAlertDialogWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Alert Dialog GPUI examples">
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
                src={`/gpui-command/index.html?demo=alert-dialog&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} alert dialog — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>Click the trigger <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="alert_dialog.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
