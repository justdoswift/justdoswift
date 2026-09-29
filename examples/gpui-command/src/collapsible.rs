//! shadcn/ui-style collapsible for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/collapsible (React Aria).
//! One entity owns open state + the reveal animation; trigger row toggles it,
//! content is an arbitrary element revealed via MotionReveal (height wipe).
use gpui_kit::{self as kit, *};
use kit::base::{Easing, MotionReveal, Transition, transition};
use std::borrow::Cow;
use std::f32::consts::PI;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

const FONT: &str = "Geist";
/// Panel reveal wipe duration.
const REVEAL: Duration = Duration::from_millis(220);

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Emitted when the panel expands or collapses.
#[derive(Clone, Debug, PartialEq)]
pub struct CollapsibleChangeEvent {
    pub open: bool,
}

/// Which chevron the trigger shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ChevronStyle {
    /// Trailing chevron-down that flips 180° (default card style).
    Down,
    /// Leading chevron-right that rotates 90° (file-tree style).
    Right,
}

/// Optional icon before the trigger label (file-tree rows).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RowIcon {
    None,
    Folder,
    File,
}

/// One collapsible — a trigger row plus a reveal panel holding arbitrary
/// content. `cx.new(|_| Collapsible::new().trigger("Details").content(el))`.
pub struct Collapsible {
    open: bool,
    trigger: SharedString,
    /// Secondary text right of the label (e.g. "Toggle details").
    hint: SharedString,
    /// Panel body factory — elements are consumed per render, so the caller
    /// hands us a constructor instead of the element itself.
    content: Option<Box<dyn Fn() -> AnyElement>>,
    chevron: ChevronStyle,
    icon: RowIcon,
    disabled: bool,
    dark: bool,
    id: usize,
    focus: Option<FocusHandle>,
}
impl EventEmitter<CollapsibleChangeEvent> for Collapsible {}

impl Collapsible {
    pub fn new() -> Self {
        let id = NEXT_KEY.fetch_add(1, Ordering::Relaxed);
        Self {
            open: false,
            trigger: "".into(),
            hint: "".into(),
            content: None,
            chevron: ChevronStyle::Down,
            icon: RowIcon::None,
            disabled: false,
            dark: false,
            id,
            focus: None,
        }
    }
    /// Trigger label (`CollapsibleTrigger`).
    pub fn trigger(mut self, label: &str) -> Self {
        self.trigger = label.into();
        self
    }
    /// Quiet text after the label, e.g. "Toggle details".
    pub fn hint(mut self, hint: &str) -> Self {
        self.hint = hint.into();
        self
    }
    /// Panel content (`CollapsibleContent`) — a factory producing the element
    /// each render, e.g. `.content(|| div().child("…"))`.
    pub fn content(mut self, content: impl Fn() -> AnyElement + 'static) -> Self {
        self.content = Some(Box::new(content));
        self
    }
    /// Initial state (`defaultExpanded` equivalent). For controlled updates
    /// use [`Collapsible::set_open`].
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
    /// Leading chevron-right (90° rotate) instead of trailing chevron-down.
    pub fn chevron(mut self, style: ChevronStyle) -> Self {
        self.chevron = style;
        self
    }
    /// Folder/file glyph before the label (file-tree rows).
    pub fn icon(mut self, icon: RowIcon) -> Self {
        self.icon = icon;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Controlled update (`isExpanded` / `onExpandedChange` equivalent).
    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.open != open {
            self.open = open;
            cx.emit(CollapsibleChangeEvent { open });
            cx.notify();
        }
    }

    fn toggle(&mut self, cx: &mut Context<Self>) {
        if !self.disabled {
            self.set_open(!self.open, cx);
        }
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        match event.keystroke.key.as_str() {
            "enter" | "space" => {
                self.toggle(cx);
                window.prevent_default();
                cx.stop_propagation();
            }
            _ => {}
        }
    }
}

/// Chevron glyph: Down flips 180°, Right rotates 90° when open.
fn chevron(progress: f32, style: ChevronStyle, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let center = bounds.center();
            // Base points form a "v"; Down rotates π·t, Right rotates -π/2·(1-t).
            let angle = match style {
                ChevronStyle::Down => PI * progress,
                ChevronStyle::Right => -PI / 2. * (1. - progress),
            };
            let (sin, cos) = angle.sin_cos();
            let points = [
                point(px(-4.5), px(-1.6)),
                point(px(0.), px(2.4)),
                point(px(4.5), px(-1.6)),
            ];
            let mut b = PathBuilder::stroke(px(1.5));
            for (i, v) in points.iter().enumerate() {
                let p = center
                    + point(
                        px(f32::from(v.x) * cos - f32::from(v.y) * sin),
                        px(f32::from(v.x) * sin + f32::from(v.y) * cos),
                    );
                if i == 0 {
                    b.move_to(p);
                } else {
                    b.line_to(p);
                }
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    // Canvas defaults to zero-height — size it to the slot or the glyph
    // drawn at `bounds.center()` sits on the box's top edge.
    .size_full()
}

/// Folder / file glyph for tree rows.
fn row_icon(icon: RowIcon, color: Hsla) -> Option<AnyElement> {
    if icon == RowIcon::None {
        return None;
    }
    Some(
        canvas(
            move |_, _, _| (),
            move |bounds, _, window, _| {
                let o = bounds.origin;
                let s: f32 = bounds.size.width.into();
                let k = s / 16.;
                let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
                let mut b = PathBuilder::stroke(px(1.3 * k.max(0.7)));
                match icon {
                    RowIcon::Folder => {
                        b.move_to(at(2., 4.));
                        b.line_to(at(2., 13.));
                        b.line_to(at(14., 13.));
                        b.line_to(at(14., 5.5));
                        b.line_to(at(8.5, 5.5));
                        b.line_to(at(7., 4.));
                        b.close();
                    }
                    RowIcon::File => {
                        b.move_to(at(4.5, 2.5));
                        b.line_to(at(4.5, 13.5));
                        b.line_to(at(11.5, 13.5));
                        b.line_to(at(11.5, 5.));
                        b.line_to(at(9., 2.5));
                        b.close();
                        b.move_to(at(9., 2.5));
                        b.line_to(at(9., 5.));
                        b.line_to(at(11.5, 5.));
                    }
                    RowIcon::None => return,
                }
                if let Ok(path) = b.build() {
                    window.paint_path(path, color);
                }
            },
        )
        .size_full()
        .into_any_element(),
    )
}

impl Render for Collapsible {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
        let hover_bg: Hsla = rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into();

        let open = self.open;
        let progress = if cx.reduce_motion() {
            if open { 1. } else { 0. }
        } else {
            transition(
                ("collapsible", SharedString::from(self.id.to_string())),
                if open { 1f32 } else { 0f32 },
                Transition::new(REVEAL).easing(Easing::EaseOut),
                window,
                cx,
            )
        };

        // ── Trigger row ─────────────────────────────────────────────────
        let mut row = div()
            .flex()
            .items_center()
            .gap_2()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(ink);
        if self.chevron == ChevronStyle::Right {
            row = row.child(div().flex_none().size(px(14.)).child(chevron(
                progress,
                self.chevron,
                muted,
            )));
        }
        if let Some(glyph) = row_icon(self.icon, muted) {
            row = row.child(div().flex_none().size(px(14.)).child(glyph));
        }
        row = row.child(div().child(self.trigger.clone()));
        if !self.hint.is_empty() {
            row = row.child(div().text_xs().text_color(muted).child(self.hint.clone()));
        }
        if self.chevron == ChevronStyle::Down {
            row = row
                .child(div().flex_1())
                .child(div().flex_none().size(px(16.)).child(chevron(
                    progress,
                    self.chevron,
                    muted,
                )));
        }

        let mut trigger = div()
            .id(("col-trigger", self.id))
            .rounded_md()
            .py_1p5()
            .px_2()
            .ml(px(-8.))
            .w_full()
            .child(row);
        if self.disabled {
            trigger = trigger.opacity(0.5);
        } else {
            let focus = self.focus.get_or_insert_with(|| cx.focus_handle()).clone();
            let focus_click = focus.clone();
            trigger = trigger
                .track_focus(&focus.tab_index(0))
                .cursor_pointer()
                .hover(|s| s.bg(hover_bg))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    focus_click.focus(window, cx);
                    window.prevent_default();
                })
                .on_click(cx.listener(|this, _, _, cx| this.toggle(cx)))
                .on_key_down(cx.listener(Self::key_down));
        }

        // ── Panel (CollapsibleContent) — MotionReveal height wipe ───────
        let mut panel = div();
        if self.content.is_some() && (open || progress > 0.001) {
            let inner = (self.content.as_ref().unwrap())();
            panel = panel.child(
                div()
                    .pl(px(match self.chevron {
                        ChevronStyle::Right => 22.,
                        ChevronStyle::Down => 2.,
                    }))
                    .child(MotionReveal::new(
                        ("collapsible-reveal", self.id),
                        progress,
                        inner,
                    )),
            );
        }

        div()
            .flex()
            .flex_col()
            .w_full()
            .font_family(FONT)
            .child(trigger)
            .child(panel)
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

fn colors(dark: bool) -> (Hsla, Hsla, Hsla, Hsla, Hsla) {
    (
        rgb(if dark { 0xededed } else { 0x171717 }).into(), // ink
        rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(), // muted
        rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(), // edge
        rgb(if dark { 0x232323 } else { 0xffffff }).into(), // card
        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into(), // surface
    )
}

/// File-tree leaf row (file glyph + name). The icon sits at the row's
/// left edge — the same column as sibling folders' chevrons (the panel
/// supplies the level indent; no chevron slot is skipped).
fn file_row(name: &str, dark: bool) -> AnyElement {
    let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    div()
        .flex()
        .items_center()
        .gap_2()
        .py(px(3.))
        .text_sm()
        .text_color(ink)
        .font_family(FONT)
        .child(
            div()
                .flex_none()
                .size(px(14.))
                .child(row_icon(RowIcon::File, muted).unwrap()),
        )
        .child(SharedString::from(name.to_string()))
        .into_any_element()
}

/// File-tree folder row — collapsible with leading chevron + folder glyph.
/// Children arrive as a factory: elements are rebuilt every render.
fn folder(
    cx: &mut Context<CollapsibleDemo>,
    name: &'static str,
    open: bool,
    dark: bool,
    children: impl Fn() -> Vec<AnyElement> + 'static,
    subs: &mut Vec<Subscription>,
) -> Entity<Collapsible> {
    let entity = cx.new(|_| {
        Collapsible::new()
            .trigger(name)
            .chevron(ChevronStyle::Right)
            .icon(RowIcon::Folder)
            .open(open)
            .dark(dark)
            .content(move || {
                // Panel wrapper already indents one level (22px = chevron
                // slot); no extra padding here or rows drift right.
                let mut col = div().flex().flex_col();
                for child in children() {
                    col = col.child(child);
                }
                col.into_any_element()
            })
    });
    subs.push(
        cx.subscribe(&entity, move |demo, _, ev: &CollapsibleChangeEvent, cx| {
            demo.outcome = Some(
                format!(
                    "{} {}",
                    if ev.open { "expanded" } else { "collapsed" },
                    name
                )
                .into(),
            );
            cx.notify();
        }),
    );
    entity
}

pub struct CollapsibleDemo {
    items: Vec<Entity<Collapsible>>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl CollapsibleDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let mut subs: Vec<Subscription> = Vec::new();
        let items: Vec<Entity<Collapsible>> = match variant {
            "open" => vec![cx.new(|_| {
                Collapsible::new()
                    .trigger("Can I use this in my project?")
                    .open(true)
                    .dark(dark)
                    .content(|| {
                        div()
                            .text_sm()
                            .text_color(rgb(0x737373))
                            .line_height(relative(1.6))
                            .child("Yes. Free to use for personal and commercial projects. No attribution required.")
                            .into_any_element()
                    })
            })],
            "settings" => vec![cx.new(|_| {
                Collapsible::new()
                    .trigger("Radius")
                    .hint("Set the corner radius of the element.")
                    .dark(dark)
                    .content(move || settings_body(dark))
            })],
            "file-tree" => {
                let ui = folder(
                    cx,
                    "ui",
                    true,
                    dark,
                    move || {
                        ["button.tsx", "card.tsx", "dialog.tsx", "input.tsx", "select.tsx", "table.tsx"]
                            .iter()
                            .map(|f| file_row(f, dark))
                            .collect()
                    },
                    &mut subs,
                );
                let components = {
                    let ui = ui.clone();
                    folder(
                        cx,
                        "components",
                        true,
                        dark,
                        move || {
                            vec![
                                div().child(ui.clone()).into_any_element(),
                                file_row("login-form.tsx", dark),
                                file_row("register-form.tsx", dark),
                            ]
                        },
                        &mut subs,
                    )
                };
                let lib = folder(
                    cx,
                    "lib",
                    false,
                    dark,
                    move || {
                        ["utils.ts", "cn.ts", "api.ts"]
                            .iter()
                            .map(|f| file_row(f, dark))
                            .collect()
                    },
                    &mut subs,
                );
                let hooks = folder(
                    cx,
                    "hooks",
                    false,
                    dark,
                    move || {
                        ["use-media-query.ts", "use-debounce.ts", "use-local-storage.ts"]
                            .iter()
                            .map(|f| file_row(f, dark))
                            .collect()
                    },
                    &mut subs,
                );
                let types = folder(
                    cx,
                    "types",
                    false,
                    dark,
                    move || {
                        ["index.d.ts", "api.d.ts"]
                            .iter()
                            .map(|f| file_row(f, dark))
                            .collect()
                    },
                    &mut subs,
                );
                let public = folder(
                    cx,
                    "public",
                    false,
                    dark,
                    move || {
                        ["favicon.ico", "logo.svg", "images"]
                            .iter()
                            .map(|f| file_row(f, dark))
                            .collect()
                    },
                    &mut subs,
                );
                vec![components, ui, lib, hooks, types, public]
            }
            _ => vec![cx.new(|_| {
                Collapsible::new()
                    .trigger("Product details")
                    .dark(dark)
                    .content(move || {
                        let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
                        let accent: Hsla = rgb(0x0285f7).into();
                        div()
                            .flex()
                            .flex_col()
                            .gap_3()
                            .pt_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(muted)
                                    .line_height(relative(1.6))
                                    .child("This panel can be expanded or collapsed to reveal additional content."),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(accent)
                                    .child("Learn More"),
                            )
                            .into_any_element()
                    })
            })],
        };
        // Track open/close of the non-tree demos too.
        if variant != "file-tree" {
            for entity in &items {
                subs.push(
                    cx.subscribe(entity, |demo, _, ev: &CollapsibleChangeEvent, cx| {
                        demo.outcome = Some(if ev.open { "Expanded" } else { "Collapsed" }.into());
                        cx.notify();
                    }),
                );
            }
        }
        Self {
            items,
            dark,
            outcome: None,
            _subscriptions: subs,
        }
    }
}

fn settings_body(dark: bool) -> AnyElement {
    let (ink, muted, edge, ..) = colors(dark);
    let row = |label: &'static str, desc: &'static str| {
        div()
            .flex()
            .items_center()
            .justify_between()
            .py_2()
            .border_t_1()
            .border_color(edge)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(ink)
                            .child(label),
                    )
                    .child(div().text_xs().text_color(muted).child(desc)),
            )
            .child(
                div()
                    .w(px(64.))
                    .h(px(28.))
                    .rounded_md()
                    .border_1()
                    .border_color(edge)
                    .bg(rgb(if dark { 0x1c1c1c } else { 0xfbfbfb })),
            )
            .into_any_element()
    };
    div()
        .flex()
        .flex_col()
        .child(row("Radius X", "Horizontal corner radius."))
        .child(row("Radius Y", "Vertical corner radius."))
        .into_any_element()
}

impl Render for CollapsibleDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, edge, card, surface) = colors(self.dark);
        let tree = self.items.len() > 1;

        let body = if tree {
            // Explorer panel — folders are entities, leaves are file rows.
            let mut list = div().flex().flex_col().w_full();
            for (i, item) in self.items.iter().enumerate() {
                // Nested "ui" lives inside "components"; render top-level only.
                if i == 1 {
                    continue;
                }
                list = list.child(item.clone());
            }
            for f in [
                "app.tsx",
                "layout.tsx",
                "globals.css",
                "package.json",
                "tsconfig.json",
                "README.md",
                ".gitignore",
            ]
            .iter()
            {
                list = list.child(file_row(f, self.dark));
            }
            div()
                .w(px(340.))
                .rounded_lg()
                .border_1()
                .border_color(edge)
                .bg(card)
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(muted)
                        .pb_2()
                        .child("Explorer"),
                )
                .child(list)
                .into_any_element()
        } else {
            let mut card_el = div()
                .w(px(420.))
                .rounded_lg()
                .border_1()
                .border_color(edge)
                .bg(card)
                .p_4()
                .flex()
                .flex_col();
            for item in &self.items {
                card_el = card_el.child(item.clone());
            }
            card_el.into_any_element()
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_3()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(body)
            .child(
                div()
                    .h(px(18.))
                    .text_xs()
                    .text_color(muted)
                    .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
            )
    }
}

pub fn setup(variant: &str, dark: bool, cx: &mut App) {
    kit::init(cx);
    cx.text_system()
        .add_fonts(vec![
            Cow::Borrowed(include_bytes!("../assets/Geist-Medium.ttf")),
            Cow::Borrowed(include_bytes!("../assets/Geist-Regular.ttf")),
        ])
        .expect("load Geist fonts");
    let variant = variant.to_string();
    let options = WindowOptions {
        #[cfg(not(target_family = "wasm"))]
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(520.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| CollapsibleDemo::new(&variant, dark, cx))
    })
    .expect("open collapsible gallery");
    #[cfg(not(target_family = "wasm"))]
    cx.on_window_closed(|cx, _| {
        if cx.windows().is_empty() {
            cx.quit();
        }
    })
    .detach();
    cx.activate(true);
}

#[cfg(not(target_family = "wasm"))]
/// Mounts this module's demo inside the shared gallery window.
pub fn demo_view(variant: &str, dark: bool, _window: &mut Window, cx: &mut App) -> AnyView {
    cx.new(|cx| CollapsibleDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{ChevronStyle, Collapsible, RowIcon};

    #[test]
    fn builders_store_state() {
        let c = Collapsible::new()
            .trigger("Details")
            .hint("Toggle details")
            .open(true)
            .chevron(ChevronStyle::Right)
            .icon(RowIcon::Folder)
            .disabled(true)
            .dark(true);
        assert!(c.is_open());
        assert_eq!(c.trigger.as_ref(), "Details");
        assert!(c.disabled && c.dark);
    }
}
