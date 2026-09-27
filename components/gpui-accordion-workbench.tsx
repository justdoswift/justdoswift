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
    hint: "默认一次只展开一个条目，首项默认打开。点击标题收起或切换，也可以用 ↑ ↓ 键在条目间移动。",
    code: `Accordion::new()
    .entry(AccordionEntry::new("shipping", "What are your shipping options?")
        .content("We offer standard (5-7 days) and express shipping."))
    .entry(AccordionEntry::new("returns", "What is your return policy?")
        .content("Returns are accepted within 30 days of purchase."))
    .entry(AccordionEntry::new("support", "How can I contact support?")
        .content("Reach us through live chat or email."))
    .open("shipping")`,
  },
  {
    id: "multiple",
    label: "Multiple",
    hint: "multiple(true) 允许多个条目同时展开。",
    code: `Accordion::new()
    .multiple(true)
    .entry(AccordionEntry::new("notifications", "Notification Settings")
        .content("Manage how you receive notifications."))
    .entry(AccordionEntry::new("privacy", "Privacy & Security")
        .content("Control who can see your activity."))
    .entry(AccordionEntry::new("billing", "Billing & Subscription")
        .content("Update your payment method."))
    .open("notifications")`,
  },
  {
    id: "disabled",
    label: "Disabled",
    hint: "disabled(true) 的条目不响应交互，键盘导航也会跳过它。",
    code: `Accordion::new()
    .entry(AccordionEntry::new("history", "Can I access my account history?")
        .content("Open Settings → History to browse sign-ins."))
    .entry(AccordionEntry::new("premium", "Premium feature information")
        .content("Available on the Professional and Enterprise plans.")
        .disabled(true))
    .entry(AccordionEntry::new("email", "How do I update my email address?")
        .content("Open Settings → Profile and edit the email field."))`,
  },
  {
    id: "card",
    label: "Card",
    hint: "把 Accordion 放进带标题与描述的卡片容器，适合账户设置这类自成一节的场景。",
    code: `// The host view wraps the entity in a bordered container.
div()
    .rounded_xl()
    .border_1()
    .border_color(rgb(0xe8e8e8))
    .px_6()
    .py_5()
    .shadow_lg()
    .child(/* card heading and description */)
    .child(self.accordion.clone())`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiAccordionWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("default");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Accordion GPUI examples">
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
                src={`/gpui-command/index.html?demo=accordion&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} accordion — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>Try it out <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="accordion.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
