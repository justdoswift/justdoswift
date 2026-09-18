export function BrandMark({ compact = false }: { compact?: boolean }) {
  return (
    <span className="brand">
      <svg className="brand-mark" viewBox="0 0 28 28" fill="none" aria-hidden="true">
        <rect x="2" y="2" width="24" height="24" rx="8" fill="currentColor" />
        <path d="M9 9v5a5 5 0 0 0 10 0V9M9 14h10" stroke="var(--background)" strokeWidth="2.4" strokeLinecap="round" />
      </svg>
      {!compact && <span>Just Do Swift<span className="brand-period">.</span></span>}
    </span>
  );
}
