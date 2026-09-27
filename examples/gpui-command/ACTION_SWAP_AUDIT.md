# Action Swap source comparison — 2026-09-25

The reference page contains three different implementations. Matching their names
does not make their timing, layering or interaction interchangeable.

## Sources checked

- Live page: https://justdoswift.com/components/action-swap
- Its loaded production JavaScript/CSS and computed browser styles.
- Downloaded Swift: https://justdoswift.com/components/action-swap/source
- Upstream beUI source: https://beui.dev/components/motion/action-swap
- Local equivalents: `components/preview-stage.tsx`, `app/globals.css`,
  `components/copy-code-button.tsx`, `components/motion/action-swap.tsx`,
  `lib/components.ts`.

The production assets confirm that the central preview uses `demo-swap`, not
the beUI component used by the page's copy-source controls.

## Three distinct behaviors

| Reference | Motion | Interaction |
| --- | --- | --- |
| Central black preview | 280 ms CSS ease-out; new content only; opacity 0→1, blur 4→0 px, vertical offset −4→0 px for Copied and +4→0 px for Copy code | Immediate toggle on every click; no clipboard operation or timed reset |
| Copy-source controls / beUI blur | 200 ms easeInOut; outgoing and incoming layers coexist; text scale .94↔1, icon scale .25↔1, blur 8↔0 px; no vertical movement; whole-button press scale .97 | The local wrapper awaits clipboard success and restores idle after 1700 ms |
| Downloaded SwiftUI example | Spring response .34, damping fraction .72; system symbol replacement and numeric-text transitions | Runs action, shows success, restores idle after 1500 ms using .2 s easeOut |

The SwiftUI source does not explicitly implement a Gaussian blur and is not the
renderer of the central browser preview. Its system transitions should not be
described as an exact specification for the CSS or beUI versions.

## Current GPUI differences

1. **Pixel snapping in motion.** The original implementation drove `.top()`.
   GPUI 0.3.6 converts absolute layout lengths to device pixels and snaps final
   layout bounds (`src/taffy.rs`, `AbsoluteLength::to_taffy`, `layout_bounds`).
   A 4 px offset therefore has about five positions at 1× scale, nine at 2×.
   CSS `transform: translateY()` retains fractional translation. This audit's
   code change uses `Svg::with_transformation(Transformation::translate(...))`,
   whose floating-point paint matrix bypasses layout rounding. This corrects
   the mechanism; it is not a claim of measured pixel-for-pixel parity.
2. **Mixed interaction model.** GPUI awaits clipboard success, ignores further
   clicks during success feedback and resets after 1600 ms. None of the three
   reference implementations has exactly that combination. The central preview
   was verified to stay Copied and return immediately on another click.
3. **Different text rendering.** The browser uses Arial 12 px / 18 px line height.
   Measured label widths are 57.375 px (Copy code) and 38.03125 px (Copied).
   GPUI uses fixed IBM Plex Sans outlines, 56.256 px and 37.68 px respectively.
   These are small visual differences, not evidence of a major timing problem.
   Baked labels also limit reuse with arbitrary text and localization.
4. **Mount and hover behavior.** The CSS preview animates its first appearance.
   GPUI skips the initial animation and adds hover/pressed background changes.
5. **Rasterization cost is unmeasured.** Each new quantized blur level produces a
   distinct SVG cache key; a cache miss parses and rasterizes SVG synchronously.
   This could affect smoothness but has not been established by a frame trace.
   Do not present it as a confirmed frame-rate regression.

The current GPUI ease-out solver matches CSS cubic-bezier(0,0,.58,1). Its 280 ms,
4 px displacement and 4 px blur match the central preview's parameters. The
GPUI Base button has no hidden padding, and no evidence was found that its
112×42 content is being compressed by the 130×42 outer button.

## Follow-through by chosen reference

- Central preview: preserve immediate reversible toggling, mount animation,
  single incoming layer, matching font metrics and fractional paint movement.
- beUI blur: retain old/new layers through exit, animate text/icon independently,
  reproduce separate clipping regions and press feedback. Merely changing blur
  strength or duration will not reproduce it.
- SwiftUI example: reproduce its system transition behavior from an actual
  native run; the spring constants alone do not specify the symbol/text effect.

Keep a single reference for each comparison. Compare both directions and rapid
repeated activation, not only static screenshots or one completed click.
