# GPUI component studies

## Accordion

`/` and `/gpui/accordion` preview the same native accordion on desktop and
WebAssembly, modeled on the
[shadcn/ui Accordion](https://ui.shadcn.com/docs/components/base/accordion)
documentation (Base UI composition).

```sh
cargo run --locked -- --accordion
cargo run --locked -- --accordion multiple
cargo run --locked -- --accordion disabled
cargo run --locked -- --accordion card
```

`src/accordion.rs` provides the styled `Accordion` entity and `AccordionEntry`
items, composed from the unstyled `gpui-base` primitives (`AccordionItem`,
`AccordionHeader`, `AccordionTrigger`, `AccordionPanel`). Single-open and
collapsible by default; `multiple(true)` keeps several entries open and
`AccordionEntry::disabled` removes an entry from pointer and arrow-key
interaction. Panels expand through `MotionReveal` driven by a keyed
`transition`, so reveals interrupt cleanly and respect reduced motion.
Up/Down/Home/End move between headers and Enter/Space toggles the focused
entry; every toggle emits `AccordionChange`.

## Alert

`/gpui/alert` previews the same native alert on desktop and WebAssembly,
modeled on the
[shadcn/ui Alert](https://ui.shadcn.com/docs/components/base/alert)
documentation.

```sh
cargo run --locked -- --alert
cargo run --locked -- --alert destructive
cargo run --locked -- --alert action
cargo run --locked -- --alert custom
```

`src/alert.rs` provides the styled `Alert` entity with a builder API:
`variant` (`Default` | `Destructive`), `icon` (`Info` | `Check` | `Error` |
`Warning`, drawn with `PathBuilder`), `title`, `description`, and `action`.
`surface` / `edge` / `tint` override the palette per alert, matching
shadcn's custom-colors example. The action button is focusable and emits
`AlertActionEvent`; intro motion uses `transition` and respects reduced
motion.

## Alert Dialog

`/gpui/alert-dialog` previews the same native alert dialog on desktop and
WebAssembly, modeled on the
[shadcn/ui Alert Dialog](https://ui.shadcn.com/docs/components/base/alert-dialog)
documentation.

```sh
cargo run --locked -- --alert-dialog
cargo run --locked -- --alert-dialog sm
cargo run --locked -- --alert-dialog media
cargo run --locked -- --alert-dialog sm-media
cargo run --locked -- --alert-dialog destructive
```

`src/alert_dialog.rs` composes the unstyled `gpui-base` modal primitives
(`AlertDialog`, `AlertDialogTrigger`, `AlertDialogBackdrop`,
`AlertDialogPopup`, `AlertDialogTitle`, `AlertDialogDescription`,
`AlertDialogCancel`, `AlertDialogAction`) behind a styled
`AlertDialogSpec` builder. `DialogHandle` shares open state between the
trigger, the host view and imperative close; the backdrop is not
dismissable, Escape resolves Cancel and Enter resolves Confirm. The
`Sm` size narrows the card and stacks the footer, `media` adds an icon
box, and `destructive` paints the confirm action red.

## Aspect Ratio

`/gpui/aspect-ratio` previews the same native ratio box on desktop and
WebAssembly, modeled on the
[shadcn/ui Aspect Ratio](https://ui.shadcn.com/docs/components/base/aspect-ratio)
documentation.

```sh
cargo run --locked -- --aspect-ratio
cargo run --locked -- --aspect-ratio square
cargo run --locked -- --aspect-ratio portrait
```

`src/aspect_ratio.rs` provides the `AspectRatio` layout element: it takes
the full parent width and derives height from `ratio` via gpui's
`aspect_ratio` style. Children render inside the box — the demo paints a
"photo" placeholder (sky, sun, ridgelines) with `canvas` + `PathBuilder`
and a ratio chip overlay. Note `canvas` has no intrinsic size: give it
`size_full` to receive real paint bounds.

## Attachment

`/gpui/attachment` previews the same native attachment card on desktop and
WebAssembly, modeled on the
[shadcn/ui Attachment](https://ui.shadcn.com/docs/components/base/attachment)
documentation.

```sh
cargo run --locked -- --attachment
cargo run --locked -- --attachment image
cargo run --locked -- --attachment states
cargo run --locked -- --attachment sizes
cargo run --locked -- --attachment group
cargo run --locked -- --attachment trigger
```

`src/attachment.rs` is a styled card entity (no gpui-base primitives):
media box, title/description column, an independently focusable remove
action emitting `AttachmentRemoveEvent`, and an optional card-wide trigger
emitting `AttachmentOpenEvent`. Five upload states render as idle,
progress bar (uploading), dimmed title (processing), destructive treatment
with a textual reason (error), and done. Sizes default/sm/xs plus a
vertical orientation for image thumbnails; the `group` demo is a
horizontal `overflow_x_scroll` row tracked with `ScrollHandle`.

## Avatar

`/gpui/avatar` previews the same native avatar on desktop and WebAssembly,
modeled on the
[shadcn/ui Avatar](https://ui.shadcn.com/docs/components/base/avatar)
documentation.

```sh
cargo run --locked -- --avatar
cargo run --locked -- --avatar badge
cargo run --locked -- --avatar group-count
cargo run --locked -- --avatar sizes
```

`src/avatar.rs` renders real pixels through gpui's `img` element: four
small bundled JPEGs (`assets/avatar-*.jpg`, ~3 KB each) decode into cached
`RenderImage` frames, and `with_fallback` draws initials when a source
fails. Sizes sm/default/lg, a ringed status badge (dot or icon), and
overlapping `AvatarGroup` rows with an optional `+N` / icon count. Note
`overflow_hidden` clips rectangularly — each painted quad gets its own
`.rounded(px(9999.))` to make the circle.

## Badge

`/gpui/badge` previews the same native badge on desktop and WebAssembly,
modeled on the
[shadcn/ui Badge](https://ui.shadcn.com/docs/components/base/badge)
documentation.

```sh
cargo run --locked -- --badge
cargo run --locked -- --badge icons
cargo run --locked -- --badge spinner
cargo run --locked -- --badge link
cargo run --locked -- --badge custom
```

`src/badge.rs` is a `Badge` entity with six variants (default /
secondary / destructive / outline / ghost / link), inline icons in
start/end slots (check, bookmark, arrow — all hand-drawn via
`PathBuilder`), a spinning-arc icon driven by
`cx.background_executor().now()` + `request_animation_frame` (frozen
under reduced motion — `std::time::Instant` does not exist on wasm),
custom `tone` tints, and a `clickable` link badge that takes focus and
emits `BadgePressEvent`.

## Breadcrumb

`/gpui/breadcrumb` previews the same native breadcrumb on desktop and
WebAssembly, modeled on the
[shadcn/ui Breadcrumb](https://ui.shadcn.com/docs/components/base/breadcrumb)
documentation.

```sh
cargo run --locked -- --breadcrumb
cargo run --locked -- --breadcrumb separator
cargo run --locked -- --breadcrumb collapsed
cargo run --locked -- --breadcrumb rtl
```

`src/breadcrumb.rs` is a `Breadcrumb` entity whose `CrumbItem` list maps
to shadcn's parts: `link(..)` ancestors (hover, focusable, emit
`BreadcrumbNavigateEvent`), `page(..)` plain text, and `Ellipsis` for
collapsed middle levels. Separators (`Chevron` / `Slash` / `Dot`) are
hand-painted with `PathBuilder`; `.rtl(true)` reverses the trail and
mirrors chevrons.

## Bubble

`/gpui/bubble` previews the same native chat bubble on desktop and
WebAssembly, modeled on the
[shadcn/ui Bubble](https://ui.shadcn.com/docs/components/base/bubble)
documentation.

```sh
cargo run --locked -- --bubble
cargo run --locked -- --bubble alignment
cargo run --locked -- --bubble reactions
cargo run --locked -- --bubble buttons
cargo run --locked -- --bubble collapsible
```

`src/bubble.rs` is a `Bubble` entity with seven variants (default /
secondary / muted / tinted / outline / ghost / destructive), start/end
alignment, edge-overlapping reaction chips (`absolute` + negative
offset), a `pressable` button-bubble emitting `BubblePressEvent`, and a
`collapsible` long-text mode with a Show more/less toggle. Ghost drops
the frame and the 80% max-width for assistant text.

## Button

`/gpui/button` previews the same native button on desktop and
WebAssembly, modeled on the
[shadcn/ui Button](https://ui.shadcn.com/docs/components/base/button)
documentation.

```sh
cargo run --locked -- --button
cargo run --locked -- --button sizes
cargo run --locked -- --button icons
cargo run --locked -- --button rounded
cargo run --locked -- --button spinner
cargo run --locked -- --button group
```

`src/button.rs` is a render-once `Button` element wrapping
`gpui_base::Button` — focus, Tab order, Enter/Space activation and
disabled inertness come from the primitive. It offers six variants
(default / secondary / destructive / outline / ghost / link), eight
sizes (xs/sm/default/lg plus four square icon sizes), hand-drawn
`PathBuilder` icons (ArrowUp / ArrowUpRight / GitBranch / Plus) and a
request-animation-frame spinner that freezes under reduced motion.
`.pill(true)` gives rounded-full; `.join(Start|Middle|End)` joins
buttons into a group with collapsed borders.

## Tabs

`motion_tabs.rs` runs the same native tab control on desktop and WebAssembly,
adapted from [beUI Tabs](https://beui.dev/components/motion/tabs). Its website
page is currently unpublished.

```sh
cargo run --locked -- --tabs pill
cargo run --locked -- --tabs overflow
cargo run --locked -- --tabs segment
cargo run --locked -- --tabs underline
```

`src/motion_tabs.rs` provides `MotionTabs`, `TabItem` and `TabsVariant`.
The selected indicator animates its position and width; label colors follow
the moving indicator. Overflowing lists scroll within their own viewport.
The optional demo host supplies the bundled Geist font and a centered stage;
applications can keep the control as an entity and react to selection changes.


## Button family

`motion_button.rs` showcases the same native Rust buttons on desktop and
WebAssembly, ported from the public
[beUI Button source](https://beui.dev/components/motion/button). Its website
page is currently unpublished. Select Metallic, Button, Stateful or Magnetic
in the preview.

```sh
cargo run --locked -- --metallic
cargo run --locked -- --metallic base
cargo run --locked -- --metallic stateful
cargo run --locked -- --metallic magnetic
```

`src/motion_button.rs` provides the reusable component and optional demo host;
`src/button_metallic.rs` paints the chrome surface. Buttons use bundled Geist
Medium (OFL, `assets/Geist-OFL.txt`). Initializing a custom host requires the
font/assets setup demonstrated by `motion_button::setup` and `run_native`.
The stateful demo simulates work; applications own their actual asynchronous
operations and set the button's loading/success/error states themselves.

## Action Swap

The default demo is now **Action Swap**: a 130 × 42 px black copy button, matching the original website's light preview. `src/action_swap.rs` implements the same native component in both the macOS app and the WebAssembly preview. The website lives at `/gpui/action-swap` (also `/gpui`).

```sh
cargo run --locked                 # Action Swap
cargo run --locked -- --command-menu  # earlier command menu experiment
```

Action Swap copies its own Rust source. The new label/icon fade in, sharpen from a 4 px Gaussian blur and move 4 px over 280 ms with the reference CSS ease-out curve, hold the success state for 1.6 s, then reset. Failed browser clipboard writes show **Try again**. Fast repeated clicks are ignored while a copy is in progress or success feedback is visible. Pending work is owned by the entity and cancels when the entity is dropped. GPUI Base supplies the button's keyboard activation and accessible label. GPUI animation respects reduced motion; the browser host passes the system preference at startup. The native SVG renderer applies Gaussian blur to bundled font outlines and icon paths; no browser CSS filter is used. Outgoing content is removed immediately, matching the reference instead of crossfading two labels. The outlines can be regenerated with `scripts/generate-swap-outlines.py` (requires fonttools).

To use the component, keep an `Entity<ActionSwapButton>` created with `cx.new(|_| ActionSwapButton::new(text))` on your host view and render it with `.child(self.copy_button.clone())`. The example's crate name remains `justdo-command` to preserve the first experiment's build paths. `ActionSwapDemo` and `setup` provide the optional standalone showcase host. The browser clipboard adapter requires the target-specific dependencies in `Cargo.toml`.

The complete source archive includes the component examples, pinned toolchain and bundled fonts. Web clipboard access requires HTTPS or localhost and an explicit user gesture; iframe embeds need `allow="clipboard-write"`.

---

## Earlier experiment: command menu

A desktop-first Rust command palette built on GPUI Base. `src/lib.rs` is shared by the native binary and the WebAssembly preview. No React imitation of the component is used.

## Run on desktop

Install Rust through rustup. On macOS, install Xcode and its command-line tools. From this directory:

```sh
cargo run --locked -- --command-menu
```

The local `rust-toolchain.toml` pins nightly because GPUI's current dependencies use unstable Rust APIs. It does not change your global default. The first build downloads and compiles GPUI and may take several minutes. Windows and Linux require the corresponding GPUI platform dependencies and have not been validated for this experiment.

Click the search input and type `theme`, `file`, or `workspace`. Up/down cycles the filtered list and scrolls the selected row into view. Enter or a click emits the selected command; Escape closes the menu. Use the reopen button or Cmd/Ctrl+K to reset and reopen it. All six actions are demonstrative: the result names the command and does not open files, change preferences, or launch a terminal.

## Use in your app

Construct `CommandPalette::new(commands, window, cx)` inside `cx.new`. Each `Command` has a stable ID, label, description, category, search keywords and a text glyph. Keep the returned entity and subscribe to `CommandExecuted`:

```rust
let palette = cx.new(|cx| CommandPalette::new(commands, window, cx));
let subscription = cx.subscribe(&palette, |host, _, event: &CommandExecuted, cx| {
    // Match event.0.id and execute the host application's own action.
    // Keep `subscription` on your host entity so the listener stays alive.
});
```

The prototype includes its own centered stage and result state to make it easy to try. Extract the surrounding stage when embedding it into an existing window. Commands are supplied on construction; dynamic command registration, fuzzy ranking, shortcuts per command and full accessibility semantics are future work. Search is case-insensitive AND matching across label, description, category and keywords.

## Website preview

From the repository root:

```sh
cargo +nightly-2026-09-25 install wasm-bindgen-cli --version 0.2.121 --locked
npm run build:gpui
npm run dev
```

Open `/gpui`. `build:gpui` produces the optimized WASM module, JavaScript bindings and downloadable source archive. The browser host uses GPUI's single-threaded Web runtime (no cross-origin isolation headers needed). A WebGPU-capable desktop browser is required. Unsupported browsers see a fallback with retry and can download the native example.

The generated web assets are checked in so standard Next.js deployment does not need a Rust toolchain. After editing Rust, regenerate them before shipping. Each component page displays its corresponding Rust module directly from source.

## Checks

```sh
cargo fmt --check
cargo test --locked --lib
cargo build --locked
cargo build --locked --lib --target wasm32-unknown-unknown --release
```

This is an experiment, not a production compatibility promise. IME composition, assistive technologies and other operating systems need further testing.

## Licensing

Project code follows the repository's GPL-3.0 license (included in the download). IBM Plex Sans is bundled under the SIL Open Font License in `assets/OFL.txt`; the font is from the upstream GPUI Kit gallery. GPUI Kit and GPUI retain their own Apache-2.0 licenses.
