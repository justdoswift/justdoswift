export function BrandMark({ compact = false }: { compact?: boolean }) {
  return (
    <span className="brand" aria-label="Just Do Swift">
      <span className="brand-mark" aria-hidden>
        <span className="brand-mark-shape brand-mark-back" />
        <span className="brand-mark-shape brand-mark-front" />
      </span>
      {!compact && <span className="brand-name">Just Do Swift</span>}
    </span>
  );
}
