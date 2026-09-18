import { getComponent, swiftComponents } from "@/lib/components";

export const dynamic = "force-static";

export function generateStaticParams() {
  return swiftComponents.map(({ slug }) => ({ slug }));
}

export async function GET(_request: Request, { params }: { params: Promise<{ slug: string }> }) {
  const { slug } = await params;
  const component = getComponent(slug);
  if (!component) return new Response("Component not found", { status: 404 });
  const filename = `${component.title.replaceAll(" ", "")}.swift`;
  return new Response(component.source, {
    headers: {
      "Content-Type": "text/plain; charset=utf-8",
      "Content-Disposition": `attachment; filename="${filename}"`,
      "X-Content-Type-Options": "nosniff",
    },
  });
}
