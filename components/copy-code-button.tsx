"use client";

import { Check, Copy } from "lucide-react";
import { useRef, useState } from "react";
import { ActionSwapBlurButton } from "@/components/motion/action-swap-blur";

const COPY_STATES = [
  { id: "copy", label: "Copy code", icon: <Copy className="size-4" />, ariaLabel: "Copy code" },
  { id: "copied", label: "Copied", icon: <Check className="size-4" />, ariaLabel: "Code copied" },
];

export function CopyCodeButton({ code, compact = false }: { code: string; compact?: boolean }) {
  const [state, setState] = useState("copy");
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);

  async function copyCode() {
    try {
      await navigator.clipboard.writeText(code);
      setState("copied");
      if (timer.current) clearTimeout(timer.current);
      timer.current = setTimeout(() => setState("copy"), 1700);
    } catch {
      setState("copy");
    }
  }

  return (
    <ActionSwapBlurButton
      items={COPY_STATES}
      value={state}
      cycle={false}
      size={compact ? "sm" : "md"}
      variant={compact ? "ghost" : "secondary"}
      onClick={copyCode}
      className={compact ? "text-white/70" : undefined}
    />
  );
}
