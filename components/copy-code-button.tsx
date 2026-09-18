"use client";

import { Check, Copy } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { ActionSwapBlurButton } from "@/components/motion/action-swap-blur";

const COPY_STATES = [
  { id: "copy", label: "Copy code", icon: <Copy className="size-4" />, ariaLabel: "Copy code" },
  { id: "copied", label: "Copied", icon: <Check className="size-4" />, ariaLabel: "Code copied" },
];

export function CopyCodeButton({ code, compact = false }: { code: string; compact?: boolean }) {
  const [state, setState] = useState("copy");
  const [error, setError] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  useEffect(() => () => { if (timer.current) clearTimeout(timer.current); }, []);

  async function copyCode() {
    try {
      await navigator.clipboard.writeText(code);
      setError(false);
      setState("copied");
      if (timer.current) clearTimeout(timer.current);
      timer.current = setTimeout(() => setState("copy"), 1700);
    } catch {
      setState("copy");
      setError(true);
    }
  }

  return (
    <span className="copy-control"><ActionSwapBlurButton
      items={COPY_STATES}
      value={state}
      cycle={false}
      size={compact ? "sm" : "md"}
      variant={compact ? "ghost" : "secondary"}
      onClick={copyCode}
    />{error && <span className="copy-error" role="status">复制失败，请选中代码手动复制。</span>}</span>
  );
}
