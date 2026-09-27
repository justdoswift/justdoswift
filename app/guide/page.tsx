import type { Metadata } from "next";
import Link from "next/link";
import { ArrowRight, BookOpen, Code2, Download, MousePointer2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import { CodeBlock } from "@/components/code-block";
import { SiteHeader } from "@/components/site-header";
import { SiteFooter } from "@/components/site-footer";
import { LibrarySidebar } from "@/components/library-sidebar";
import { LibraryResources } from "@/components/library-resources";

export const metadata: Metadata = {
  title: "Introduction",
  description: "从网页预览到桌面应用：下载、运行并集成 Rust / GPUI Accordion 组件。",
  alternates: { canonical: "/guide" },
};

const commands = `# From the extracted example directory:
cargo run --locked -- --accordion

# Or explore another demo:
cargo run --locked -- --accordion card`;

export default function GuidePage() {
  return (
    <>
      <SiteHeader />
      <div className="docs-shell">
        <LibrarySidebar active="guide" />
        <main id="main-content" className="guide-main gpui-guide">
          <Breadcrumb className="mb-5">
            <BreadcrumbList>
              <BreadcrumbItem><span>Getting started</span></BreadcrumbItem>
              <BreadcrumbSeparator>/</BreadcrumbSeparator>
              <BreadcrumbItem><BreadcrumbPage>Introduction</BreadcrumbPage></BreadcrumbItem>
            </BreadcrumbList>
          </Breadcrumb>
          <header className="page-heading">
            <div className="heading-eyebrow"><span /> Rust &amp; GPUI</div>
            <h1>Make it your own.</h1>
            <p>为桌面应用设计的 GPUI 组件，从 Accordion 开始。<br />从预览到源码，把细腻的交互带进你的项目。</p>
          </header>
          <div className="guide-steps">
            <section>
              <span><MousePointer2 /></span>
              <div>
                <h2>01. Explore the interaction</h2>
                <p>打开组件页面，在 Preview 中体验展开、收起和键盘导航，试试不同模式与明暗主题。网页通过 WASM 运行 GPUI，与桌面端共用同一份 Rust 组件源码。</p>
              </div>
            </section>
            <section>
              <span><Code2 /></span>
              <div>
                <h2>02. Run it on your desktop</h2>
                <p>下载完整示例并解压。安装 Rust 后，在示例目录运行下方命令；rustup 会按照项目中的 rust-toolchain.toml 使用指定工具链。首次构建会下载依赖。</p>
                <Button className="gpui-guide-download rounded-full" size="sm" asChild>
                  <a href="/gpui-command/source.zip" download><Download /> Download Rust examples</a>
                </Button>
                <CodeBlock code={commands} label="Terminal" />
                <p className="gpui-guide-command-note">Accordion 还提供 multiple、disabled、card 三个演示。替换命令最后的名称即可体验。</p>
              </div>
            </section>
            <section>
              <span><BookOpen /></span>
              <div>
                <h2>03. Make the details yours</h2>
                <p>组件页面的 Code 展示实际 Rust 实现，Usage 给出接入方式。保留组件实体，将点击回调或选中状态连接到你的应用，再按需要调整文案、颜色与布局。完整示例包含所需字体和图形资源，方便一起迁入项目。</p>
              </div>
            </section>
          </div>
          <div className="guide-next">
            <div><h2>Start with a small interaction.</h2><p>从 Accordion 的第一段展开开始。</p></div>
            <Button className="rounded-full" size="sm" asChild>
              <Link href="/gpui/accordion">Explore Accordion <ArrowRight /></Link>
            </Button>
          </div>
          <SiteFooter />
        </main>
        <LibraryResources />
      </div>
    </>
  );
}
