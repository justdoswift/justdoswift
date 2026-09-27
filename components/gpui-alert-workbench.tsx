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
    hint: "图标 + 标题 + 描述的基础提示块，Overview 连续展示两条。",
    code: `Alert::new()
    .icon(AlertIcon::Check)
    .title("Payment successful")
    .description("Your payment of $29.99 has been processed.")`,
  },
  {
    id: "destructive",
    label: "Destructive",
    hint: "variant(AlertVariant::Destructive) 把边框、文字与图标整体染成警示色。",
    code: `Alert::new()
    .variant(AlertVariant::Destructive)
    .icon(AlertIcon::Error)
    .title("Payment failed")
    .description("Your payment could not be processed.")`,
  },
  {
    id: "action",
    label: "Action",
    hint: "action(label) 在提示块右侧加按钮，点击发出 AlertActionEvent。",
    code: `Alert::new()
    .icon(AlertIcon::Info)
    .title("Dark mode is now available")
    .action("Enable")

// cx.subscribe(&self.alert, |_, _, event, _| {
//     println!("pressed: {}", event.label);
// });`,
  },
  {
    id: "custom",
    label: "Custom colors",
    hint: "surface / edge / tint 覆盖默认配色，对应 shadcn 的自定义色示例。",
    code: `Alert::new()
    .icon(AlertIcon::Warning)
    .surface(rgb(0xfffbec))
    .edge(rgb(0xf3d889))
    .tint(rgb(0xa15c07))
    .title("Your subscription will expire in 3 days.")`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAlertWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Alert GPUI examples">
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
          <div className="workbench-preview gpui-alert-preview" data-preview-theme={dark ? "dark" : "light"}>
            <div className="preview-stage">
              <iframe
                key={`${variant}-${dark}-${replay}`}
                className="gpui-action-frame"
                src={`/gpui-command/index.html?demo=alert&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} alert — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>Try it out <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="alert.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
