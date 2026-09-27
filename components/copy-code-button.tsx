"use client";

import { Check, Copy } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";

export function CopyCodeButton({ code, compact = false }: { code: string; compact?: boolean }) {
  const [copied, setCopied] = useState(false);
  const [error, setError] = useState(false);
  const timer = useRef<ReturnType<typeof setTimeout> | null>(null);
  useEffect(() => () => { if (timer.current) clearTimeout(timer.current); }, []);

  async function copyCode() {
    try {
      await navigator.clipboard.writeText(code);
      setError(false);
      setCopied(true);
      if (timer.current) clearTimeout(timer.current);
      timer.current = setTimeout(() => setCopied(false), 1700);
    } catch {
      setCopied(false);
      setError(true);
    }
  }

  return (
    <span className="copy-control">
      <Button
        variant={compact ? "ghost" : "secondary"}
        size={compact ? "sm" : "default"}
        onClick={copyCode}
        aria-label={copied ? "Code copied" : "Copy code"}
      >
        {copied ? <Check /> : <Copy />}
        {copied ? "Copied" : "Copy code"}
      </Button>
      {error && <span className="copy-error" role="status">复制失败，请选中代码手动复制。</span>}
    </span>
  );
}
