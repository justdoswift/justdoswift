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
    height: 560,
    hint: "登录卡：header 标题+描述+右上角 Sign Up，content 表单字段，footer 两个全宽按钮。",
    code: `Card::new().dark(dark)
    .child(card_header(CARD_SPACING, dark, |h| {
        h.title("Login to your account")
            .description("Enter your email below…")
            .action(signup_button);
    }))
    .child(card_content(CARD_SPACING, dark, |c| {
        c.child(email_field).child(password_field);
    }))
    .child(card_footer(CARD_SPACING, dark, |f| {
        f.column(true)
            .child(login_button)
            .child(google_button);
    }))`,
  },
  {
    id: "sm",
    label: "Size: sm",
    height: 340,
    hint: ".size(CardSize::Sm) 把 --card-spacing 从 24px 收到 16px —— 所有 section 一起收紧。",
    code: `Card::new()
    .size(CardSize::Sm)
    .child(card_header(CARD_SPACING_SM, dark, |h| {
        h.title("Scheduled reports")
            .description("Weekly snapshots…")
            .action(chevron_button);
    }))
    .child(card_content(CARD_SPACING_SM, dark, |c| {
        c.child(bullet("Choose a schedule…"))
         // …
        ;
    }))`,
  },
  {
    id: "spacing",
    label: "Spacing",
    height: 560,
    hint: ".spacing(20) 覆盖默认 24px；divider 用 mx(-sp) 贯穿卡片内边距（shadcn 的 -mx-(--card-spacing)），footer bordered + 右对齐按钮。",
    code: `let sp = 20.;
Card::new().spacing(sp)
    .child(card_content(sp, dark, |c| {
        c.child(para("These terms govern…"))
            // edge-to-edge hairline divider
            .child(div().h(px(1.)).w_full()
                .mx(px(-sp)).bg(edge))
            .child(para("You are responsible…"));
    }))
    .child(card_footer(sp, dark, |f| {
        f.bordered(true).justify_end(true)
            .child(decline_button)
            .child(accept_button);
    }))`,
  },
  {
    id: "image",
    label: "Image",
    height: 520,
    hint: "card_cover(el, sp) 作为第一个 child —— mt(-sp) 吃掉卡片顶部 padding，媒体区贴齐圆角边缘。",
    code: `Card::new()
    .child(card_cover(gradient_cover, CARD_SPACING))
    .child(card_header(CARD_SPACING, dark, |h| {
        h.title("Design systems meetup")
            .description("A practical talk…");
    }))
    .child(card_footer(CARD_SPACING, dark, |f| {
        f.justify_end(true).child(view_event_button);
    }))`,
  },
] as const;

type VariantId = (typeof variants)[number]["id"];
type ContentTab = "preview" | "code";

export function GpuiCardWorkbench({ source }: { source: string }) {
  const [variant, setVariant] = useState<VariantId>("basic");
  const [tab, setTab] = useState<ContentTab>("preview");
  const [dark, setDark] = useState(false);
  const [replay, setReplay] = useState(0);
  const selected = variants.find((item) => item.id === variant) ?? variants[0];

  return (
    <section className="workbench gpui-accordion-workbench" id="preview" aria-label="Card GPUI examples">
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
                src={`/gpui-command/index.html?demo=card&variant=${variant}&theme=${dark ? "dark" : "light"}`}
                title={`${selected.label} card — native GPUI preview`}
              />
            </div>
            <div className="preview-caption"><span>GPUI · Live preview</span><span>GPUI WASM <span aria-hidden="true">↗</span></span></div>
          </div>
        </TabsContent>
        <TabsContent value="code" className="workbench-body">
          <div className="workbench-code">
            <CodeBlock code={selected.code} label="Example.rs" />
            <CodeBlock code={source} label="card.rs" />
          </div>
        </TabsContent>
      </Tabs>
    </section>
  );
}
