import Link from "next/link";
import { readFile } from "node:fs/promises";
import path from "node:path";
import { ArrowUpRight } from "lucide-react";
import { GpuiAvatarWorkbench } from "@/components/gpui-avatar-workbench";
import { CopyCodeButton } from "@/components/copy-code-button";
import { CodeBlock } from "@/components/code-block";
import { SiteFooter } from "@/components/site-footer";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

const usage = `use justdo_command::avatar::{Avatar, AvatarImage, AvatarSize};

// A stack of overlapping avatars or a standalone one —
// Avatar is a render-once element, drop it into any container.
div().child(
    Avatar::new()
        .image(AvatarImage::Photo(0))
        .initials("CN")
        .badge(AvatarBadge::Dot),
)`;

const composition = `Avatar                    circular root, self-rounded quads
├── AvatarImage           .image(..) -> img + ObjectFit::Cover
├── AvatarFallback        .initials(..) — shown on missing source
└── AvatarBadge           .badge(..) — bottom-right status dot

AvatarGroup               overlapping row: -10px margin + ring
├── Avatar × N
└── AvatarGroupCount      "+3" circle — or an icon in the same slot`;

const install = `# Download the runnable example crate, then:
cargo run --locked -- --avatar

# Other demos:
cargo run --locked -- --avatar badge
cargo run --locked -- --avatar badge-icon
cargo run --locked -- --avatar group
cargo run --locked -- --avatar group-count
cargo run --locked -- --avatar group-icon
cargo run --locked -- --avatar sizes`;

const cargoToml = `[dependencies]
gpui-kit = "0.6.6"
image = { version = "0.25", default-features = false, features = ["jpeg"] }

# avatar.rs renders real pixels through gpui's img
# element — bundled JPEGs decode once into cached
# RenderImage frames, so no network is needed.`;

const imageNote = `// Pictures go through gpui's image pipeline, so the
// fallback is real: point .image(..) at a source that
// fails to decode and the initials render instead.
Avatar::new()
    .image(AvatarImage::Broken)   // -> fallback
    .initials("ER");`;

export async function AvatarDoc() {
  const source = await readFile(
    path.join(process.cwd(), "examples/gpui-command/src/avatar.rs"),
    "utf8",
  );

  return (
    <>
      <main id="main-content" className="detail-main">
        <Breadcrumb className="mb-5">
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink asChild><Link href="/">Components</Link></BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbPage>Avatar</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
        <header className="detail-heading">
          <div>
            <h1>Avatar</h1>
            <p>An image element with a fallback for representing the user.</p>
          </div>
          <CopyCodeButton code={usage} compact />
        </header>

        <GpuiAvatarWorkbench source={source} />

        <section className="doc-section" id="installation">
          <h2>Installation</h2>
          <p>Avatar 是单个 Rust 文件（<code className="inline-code">src/avatar.rs</code>）。图片走 gpui 的真实 <code className="inline-code">img</code> 元素 —— 演示用的是 4 张内嵌 JPEG 小头像（<code className="inline-code">assets/avatar-*.jpg</code>，每张约 3KB），解码成 <code className="inline-code">RenderImage</code> 后走真实 img 管线；换成你自己的 <code className="inline-code">ImageSource</code>（URI / Embedded / Render）即可。</p>
          <CodeBlock code={cargoToml} label="Cargo.toml" />
          <p><a className="gpui-source-link" href="/gpui-command/source.zip" download>Download runnable example <ArrowUpRight size={13} /></a></p>
          <CodeBlock code={install} label="Terminal" />
        </section>

        <section className="doc-section" id="usage">
          <h2>Usage</h2>
          <p><code className="inline-code">Avatar</code> 是 render-once 元素（<code className="inline-code">IntoElement</code>），builder 配置图片源、首字母 fallback、尺寸与角标；group 叠压由调用方用负外边距组合。</p>
          <CodeBlock code={usage} label="Example.rs" />
          <p className="source-verification">网页预览与桌面端运行同一份 Rust / GPUI 组件。</p>
        </section>

        <section className="doc-section" id="composition">
          <h2>Composition</h2>
          <p>组合结构对齐 shadcn/ui 的 Avatar / AvatarGroup。</p>
          <CodeBlock code={composition} label="Structure" copy={false} />
          <CodeBlock code={imageNote} label="Example.rs" />
        </section>

        <section className="doc-section" id="notes">
          <h2>Notes</h2>
          <p><strong>Circular clipping.</strong> GPUI 的 <code className="inline-code">overflow_hidden</code> 裁剪是矩形的，不会沿圆角裁子元素 —— 圆形头像要给每个绘制 quad（<code className="inline-code">img</code>、fallback 块）各自设置 <code className="inline-code">.rounded(px(9999.))</code>。avatar.rs 已处理。</p>
          <p><strong>Group overlap.</strong> <code className="inline-code">-10px</code> 外边距 + 每个头像 <code className="inline-code">.border_2()</code> 页面色描边，即可得到 shadcn 的分割叠压效果。</p>
        </section>

        <section className="doc-section" id="accessibility">
          <h2>Accessibility</h2>
          <div className="implementation-notes">
            <div>
              <h3>Semantics</h3>
              <p>头像用图片或首字母均可识别用户；状态角标带独立含义（在线/忙碌），以 badge 颜色 + 图形表达。纯展示组件，无键盘焦点。</p>
            </div>
            <div>
              <h3>Fallback</h3>
              <p>图片加载失败始终渲染首字母 fallback —— 不依赖网络成功，也避免空白占位。fallback 文字颜色与背景保持可读对比度。</p>
            </div>
          </div>
        </section>

        <section className="doc-section" id="api">
          <h2>API Reference</h2>
          <p>元素暴露的 builder 方法。</p>
          <Table className="api-table">
            <TableHeader>
              <TableRow className="hover:bg-transparent">
                <TableHead>API</TableHead>
                <TableHead>Signature</TableHead>
                <TableHead>Notes</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow><TableCell><code>Avatar::new()</code></TableCell><TableCell><code>-&gt; Avatar</code></TableCell><TableCell>Default size, no image — initials only.</TableCell></TableRow>
              <TableRow><TableCell><code>.image(..)</code></TableCell><TableCell><code>AvatarImage</code></TableCell><TableCell>Photo(seed) | Broken — real img element path.</TableCell></TableRow>
              <TableRow><TableCell><code>.initials(..)</code></TableCell><TableCell><code>impl Into&lt;SharedString&gt;</code></TableCell><TableCell>Fallback text shown when no image or load fails.</TableCell></TableRow>
              <TableRow><TableCell><code>.size(..)</code></TableCell><TableCell><code>AvatarSize</code></TableCell><TableCell>Sm 24 | Default 32 | Lg 40 diameter.</TableCell></TableRow>
              <TableRow><TableCell><code>.badge(..)</code></TableCell><TableCell><code>AvatarBadge</code></TableCell><TableCell>Dot | Icon — bottom-right status indicator with ring.</TableCell></TableRow>
              <TableRow><TableCell><code>.ring(..)</code></TableCell><TableCell><code>impl Into&lt;Hsla&gt;</code></TableCell><TableCell>Page-surface color behind the badge/group ring.</TableCell></TableRow>
              <TableRow><TableCell><code>.dark(bool)</code></TableCell><TableCell><code>bool</code></TableCell><TableCell>Dark scheme for fallback bg, ink and default ring.</TableCell></TableRow>
            </TableBody>
          </Table>
        </section>

        <section className="doc-section" id="reference">
          <h2>Source &amp; inspiration</h2>
          <p>结构与视觉参考 <a className="gpui-source-link" href="https://ui.shadcn.com/docs/components/base/avatar" target="_blank" rel="noreferrer">shadcn/ui Avatar <ArrowUpRight size={12} /></a>。本页交互由 Rust / GPUI 绘制，基础样式来自 <a className="gpui-source-link" href="https://github.com/longbridge/gpui-kit" target="_blank" rel="noreferrer">gpui-kit <ArrowUpRight size={12} /></a>。</p>
        </section>
        <SiteFooter />
      </main>

      <aside className="resource-rail detail-rail" aria-label="On this page">
        <div className="on-this-page">
          <span>On this page</span>
          <a href="#preview">Overview</a>
          <a href="#installation">Installation</a>
          <a href="#usage">Usage</a>
          <a href="#composition">Composition</a>
          <a href="#notes">Notes</a>
          <a href="#accessibility">Accessibility</a>
          <a href="#api">API Reference</a>
        </div>
        <div className="component-facts">
          <h2>At a glance</h2>
          <dl>
            <div><dt>Framework</dt><dd>GPUI</dd></div>
            <div><dt>Platform</dt><dd>Desktop</dd></div>
            <div><dt>Examples</dt><dd>7</dd></div>
            <div><dt>Source</dt><dd>Rust</dd></div>
          </dl>
        </div>
        <Link className="guide-callout" href="/guide">
          <span>Getting started</span>
          <strong>Run it natively.<ArrowUpRight /></strong>
          <p>Download the example and build for your desktop.</p>
        </Link>
      </aside>
    </>
  );
}
