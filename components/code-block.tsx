import { FileCode2 } from "lucide-react";
import { CopyCodeButton } from "@/components/copy-code-button";

const tokens = /("(?:\\.|[^"\\])*"|\/\/.*|@\w+|\b(?:import|struct|let|var|private|some|return|if|else|guard|in|true|false|nil|try|await|for|func)\b|\b[A-Z][A-Za-z0-9_]*\b|\b\d+(?:\.\d+)?\b)/g;

function tokenClass(token: string) {
  if (token.startsWith('"')) return "syntax-string";
  if (token.startsWith("//")) return "syntax-comment";
  if (token.startsWith("@")) return "syntax-attribute";
  if (/^[A-Z]/.test(token)) return "syntax-type";
  if (/^\d/.test(token)) return "syntax-number";
  return "syntax-keyword";
}

export function CodeBlock({ code, label, copy = true }: { code: string; label: string; copy?: boolean }) {
  return (
    <div className="code-panel">
      <div className="code-filebar"><span><FileCode2 />{label}</span>{copy && <CopyCodeButton code={code} compact />}</div>
      <pre tabIndex={0} aria-label={label}><code>{code.split("\n").map((line, index) => (
        <span className="code-line" key={index}><span className="line-number" aria-hidden="true">{index + 1}</span><span>{line.split(tokens).map((part, tokenIndex) => tokenIndex % 2 ? <span className={tokenClass(part)} key={tokenIndex}>{part}</span> : part) || " "}</span></span>
      ))}</code></pre>
    </div>
  );
}
