//! shadcn/ui-style button for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/button
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, base::Button as BaseButton, *};
use std::borrow::Cow;
use std::rc::Rc;

const FONT: &str = "Geist";

/// Visual style, mirroring shadcn's `variant` prop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// Control geometry, mirroring shadcn's `size` prop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonSize {
    Xs,
    Sm,
    Default,
    Lg,
    IconXs,
    IconSm,
    Icon,
    IconLg,
}

/// Optional inline icon (`data-icon="inline-start|end"`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonIcon {
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowUpRight,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    GitBranch,
    Minus,
    Plus,
    /// Continuously rotating arc; frozen when reduced motion is on.
    Spinner,
}

/// Corner group membership for joined buttons (`ButtonGroup`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ButtonJoin {
    Solo,
    Start,
    Middle,
    End,
}

/// Where the icon sits relative to the label.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IconPos {
    Start,
    End,
}

/// A shadcn-styled button element built on `gpui_base::Button`, which owns
/// focus, keyboard activation (Enter/Space), and the disabled inert behavior.
#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    label: Option<SharedString>,
    a11y: Option<SharedString>,
    icon: Option<ButtonIcon>,
    icon_pos: IconPos,
    variant: ButtonVariant,
    size: ButtonSize,
    pill: bool,
    join: ButtonJoin,
    vjoin: Option<ButtonJoin>,
    disabled: bool,
    dark: bool,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            a11y: None,
            icon: None,
            icon_pos: IconPos::Start,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            pill: false,
            join: ButtonJoin::Solo,
            vjoin: None,
            disabled: false,
            dark: false,
            on_click: None,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Accessibility name for icon-only buttons (no visible label).
    pub fn a11y_label(mut self, label: impl Into<SharedString>) -> Self {
        self.a11y = Some(label.into());
        self
    }
    /// Inline icon; `pos` mirrors `data-icon="inline-start|end"`.
    pub fn icon(mut self, icon: ButtonIcon, pos: IconPos) -> Self {
        self.icon = Some(icon);
        self.icon_pos = pos;
        self
    }
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }
    /// `rounded-full` — used by the Rounded demo.
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }
    /// Joined position inside a horizontal button group (squares the inner
    /// corners and overlaps the border).
    pub fn join(mut self, join: ButtonJoin) -> Self {
        self.join = join;
        self
    }
    /// Joined position inside a vertical button group.
    pub fn vjoin(mut self, join: ButtonJoin) -> Self {
        self.vjoin = Some(join);
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
    /// Pointer, Enter, and Space activation (provided by the base Button).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(handler));
        self
    }
}

/// (surface, ink, edge, hover, active) per variant + theme.
fn variant_colors(variant: ButtonVariant, dark: bool) -> (Hsla, Hsla, Hsla, Hsla, Hsla) {
    let c = |v: u32| -> Hsla { rgb(v).into() };
    let clear = rgba(0).into();
    match (variant, dark) {
        (ButtonVariant::Default, false) => (
            c(0x171717),
            c(0xffffff),
            c(0x171717),
            c(0x2e2e2e),
            c(0x404040),
        ),
        (ButtonVariant::Default, true) => (
            c(0xededed),
            c(0x171717),
            c(0xededed),
            c(0xffffff),
            c(0xd4d4d4),
        ),
        (ButtonVariant::Secondary, false) => (
            c(0xe4e4e4),
            c(0x171717),
            c(0xe4e4e4),
            c(0xd9d9d9),
            c(0xcccccc),
        ),
        (ButtonVariant::Secondary, true) => (
            c(0x333333),
            c(0xededed),
            c(0x333333),
            c(0x3f3f3f),
            c(0x4a4a4a),
        ),
        (ButtonVariant::Destructive, false) => (
            c(0xdc2626),
            c(0xffffff),
            c(0xdc2626),
            c(0xb91c1c),
            c(0x991b1b),
        ),
        (ButtonVariant::Destructive, true) => (
            c(0x7f1d1d),
            c(0xfecaca),
            c(0x7f1d1d),
            c(0x991b1b),
            c(0xb91c1c),
        ),
        (ButtonVariant::Outline, false) => (
            c(0xffffff),
            c(0x171717),
            c(0xe4e4e4),
            c(0xf5f5f5),
            c(0xebebeb),
        ),
        (ButtonVariant::Outline, true) => (
            c(0x232323),
            c(0xededed),
            c(0x3b3b3b),
            c(0x2c2c2c),
            c(0x363636),
        ),
        (ButtonVariant::Ghost, false) => (clear, c(0x171717), clear, c(0xf0f0f0), c(0xe4e4e4)),
        (ButtonVariant::Ghost, true) => (clear, c(0xededed), clear, c(0x2c2c2c), c(0x3b3b3b)),
        (ButtonVariant::Link, false) => (clear, c(0x171717), clear, clear, clear),
        (ButtonVariant::Link, true) => (clear, c(0xededed), clear, clear, clear),
    }
}

/// Rotating arc, like shadcn's `<Spinner />`. `angle` turns 0..1.
fn spinner(angle: f32, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let center = o + point(px(s * 0.5), px(s * 0.5));
            let r = px(s * 0.36);
            let start = angle * 360.;
            let steps = 16;
            let mut b = PathBuilder::stroke(px((s * 0.11).max(1.4)));
            for i in 0..=steps {
                let rad = (start + i as f32 * 240. / steps as f32).to_radians();
                let p = center + point(r * rad.cos(), r * rad.sin());
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
    .size_full()
}

fn button_icon(icon: ButtonIcon, color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 12.;
            let pxk = |v: f32| px(v * k);
            let at = |x: f32, y: f32| o + point(pxk(x), pxk(y));
            let mut b = PathBuilder::stroke(px(1.3 * k.max(0.7)));
            match icon {
                ButtonIcon::ArrowDown => {
                    b.move_to(at(6., 2.));
                    b.line_to(at(6., 10.));
                    b.move_to(at(2.6, 6.6));
                    b.line_to(at(6., 10.));
                    b.line_to(at(9.4, 6.6));
                }
                ButtonIcon::ArrowLeft => {
                    b.move_to(at(10., 6.));
                    b.line_to(at(2., 6.));
                    b.move_to(at(5.4, 2.6));
                    b.line_to(at(2., 6.));
                    b.line_to(at(5.4, 9.4));
                }
                ButtonIcon::ArrowRight => {
                    b.move_to(at(2., 6.));
                    b.line_to(at(10., 6.));
                    b.move_to(at(6.6, 2.6));
                    b.line_to(at(10., 6.));
                    b.line_to(at(6.6, 9.4));
                }
                ButtonIcon::ArrowUp => {
                    b.move_to(at(6., 10.));
                    b.line_to(at(6., 2.));
                    b.move_to(at(2.6, 5.4));
                    b.line_to(at(6., 2.));
                    b.line_to(at(9.4, 5.4));
                }
                ButtonIcon::ArrowUpRight => {
                    b.move_to(at(3., 9.));
                    b.line_to(at(9., 3.));
                    b.move_to(at(4.6, 3.));
                    b.line_to(at(9., 3.));
                    b.line_to(at(9., 7.4));
                }
                ButtonIcon::GitBranch => {
                    b.move_to(at(3.6, 2.8));
                    b.line_to(at(3.6, 9.2));
                    b.move_to(at(3.6, 4.2));
                    b.curve_to(at(3.6, 7.), at(8.4, 4.6));
                    b.line_to(at(8.4, 4.6));
                    for (cx, cy) in [(3.6, 2.4), (3.6, 9.6), (8.8, 4.6)] {
                        // Quarter arcs (curve_to takes one control point).
                        b.move_to(at(cx + 1., cy));
                        b.curve_to(at(cx + 1., cy + 1.), at(cx, cy + 1.));
                        b.curve_to(at(cx - 1., cy + 1.), at(cx - 1., cy));
                        b.curve_to(at(cx - 1., cy - 1.), at(cx, cy - 1.));
                        b.curve_to(at(cx + 1., cy - 1.), at(cx + 1., cy));
                    }
                }
                ButtonIcon::Plus => {
                    b.move_to(at(6., 2.6));
                    b.line_to(at(6., 9.4));
                    b.move_to(at(2.6, 6.));
                    b.line_to(at(9.4, 6.));
                }
                ButtonIcon::Minus => {
                    b.move_to(at(2.6, 6.));
                    b.line_to(at(9.4, 6.));
                }
                ButtonIcon::ChevronDown => {
                    b.move_to(at(2.8, 4.4));
                    b.line_to(at(6., 7.6));
                    b.line_to(at(9.2, 4.4));
                }
                ButtonIcon::ChevronUp => {
                    b.move_to(at(2.8, 7.6));
                    b.line_to(at(6., 4.4));
                    b.line_to(at(9.2, 7.6));
                }
                ButtonIcon::ChevronLeft => {
                    b.move_to(at(7.6, 2.8));
                    b.line_to(at(4.4, 6.));
                    b.line_to(at(7.6, 9.2));
                }
                ButtonIcon::ChevronRight => {
                    b.move_to(at(4.4, 2.8));
                    b.line_to(at(7.6, 6.));
                    b.line_to(at(4.4, 9.2));
                }
                ButtonIcon::Spinner => return,
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size_full()
    .into_any_element()
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let dark = self.dark;
        let (surface, ink, edge, hover, active) = variant_colors(self.variant, dark);
        let link = self.variant == ButtonVariant::Link;
        let spinning = self.icon == Some(ButtonIcon::Spinner);
        let a11y = self
            .a11y
            .clone()
            .or_else(|| self.label.clone())
            .unwrap_or_else(|| "button".into());

        // Spinner clock persists in keyed state so the element stays
        // stateless. The key derives from the a11y label, not `self.id`
        // (the base Button already uses that key for its FocusHandle).
        let angle = if spinning {
            let clock_key = format!("button-clock-{a11y}");
            let start = *window
                .use_keyed_state(SharedString::from(clock_key), cx, |_, cx| {
                    cx.background_executor().now()
                })
                .read(cx);
            if !cx.reduce_motion() {
                window.request_animation_frame();
            }
            cx.background_executor()
                .now()
                .duration_since(start)
                .as_secs_f32()
                * 1.2
                % 1.
        } else {
            0.
        };

        // (height, padding-x, icon glyph, small text, horizontal gap)
        let (h, pad_x, icon_px, small_text, gap) = match self.size {
            ButtonSize::Xs => (px(24.), px(8.), px(12.), true, px(4.)),
            ButtonSize::Sm => (px(32.), px(12.), px(14.), false, px(6.)),
            ButtonSize::Default => (px(36.), px(16.), px(14.), false, px(6.)),
            ButtonSize::Lg => (px(40.), px(24.), px(16.), false, px(8.)),
            ButtonSize::IconXs => (px(24.), px(0.), px(12.), true, px(0.)),
            ButtonSize::IconSm => (px(32.), px(0.), px(14.), false, px(0.)),
            ButtonSize::Icon => (px(36.), px(0.), px(16.), false, px(0.)),
            ButtonSize::IconLg => (px(40.), px(0.), px(18.), false, px(0.)),
        };
        let icon_only = matches!(
            self.size,
            ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::Icon | ButtonSize::IconLg
        );

        let radius = px(if self.pill { 9999. } else { 8. });

        let mut el = BaseButton::new(self.id.clone())
            .accessibility_label(a11y)
            .disabled(self.disabled)
            .flex_none()
            .h(h)
            .when(icon_only, |style| style.w(h))
            .when(!icon_only && !link, |style| style.px(pad_x))
            .when(link, |style| style.px(px(4.)))
            .gap(gap)
            .font_family(FONT)
            .when(small_text, |style| style.text_xs())
            .when(!small_text, |style| style.text_sm())
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap()
            .text_color(ink)
            .bg(surface)
            .border_1()
            .border_color(edge)
            .cursor_pointer()
            .hover(|style| style.bg(hover))
            .active(|style| style.bg(active))
            .focus_visible(|style| style.border_color(ink))
            .styles(|styles| styles.disabled(|style| style.opacity(0.45)));

        // Corner grouping, matching shadcn's approach: non-first children
        // drop their leading border (`border-l-0`) so the previous button's
        // trailing border is the shared seam — no negative margins needed.
        if let Some(vjoin) = self.vjoin {
            let (tl, tr, br, bl) = match vjoin {
                ButtonJoin::Start => (radius, radius, px(0.), px(0.)),
                ButtonJoin::Middle => (px(0.), px(0.), px(0.), px(0.)),
                ButtonJoin::End => (px(0.), px(0.), radius, radius),
                ButtonJoin::Solo => (radius, radius, radius, radius),
            };
            el = el
                .rounded_tl(tl)
                .rounded_tr(tr)
                .rounded_br(br)
                .rounded_bl(bl)
                .when(
                    vjoin == ButtonJoin::Middle || vjoin == ButtonJoin::End,
                    |style| style.border_t_0(),
                );
        } else if self.join == ButtonJoin::Solo {
            el = el.rounded(radius);
        } else {
            let (tl, tr, br, bl) = match self.join {
                ButtonJoin::Start => (radius, px(0.), px(0.), radius),
                ButtonJoin::Middle => (px(0.), px(0.), px(0.), px(0.)),
                ButtonJoin::End => (px(0.), radius, radius, px(0.)),
                ButtonJoin::Solo => unreachable!(),
            };
            el = el
                .rounded_tl(tl)
                .rounded_tr(tr)
                .rounded_br(br)
                .rounded_bl(bl)
                .when(
                    self.join == ButtonJoin::Middle || self.join == ButtonJoin::End,
                    |style| style.border_l_0(),
                );
        }

        if let Some(icon) = self.icon {
            let icon_el = if icon == ButtonIcon::Spinner {
                spinner(angle, ink).into_any_element()
            } else {
                button_icon(icon, ink)
            };
            let icon_el = div()
                .flex_none()
                .size(icon_px)
                .flex()
                .items_center()
                .justify_center()
                .child(icon_el)
                .into_any_element();
            if let Some(label) = self.label.clone() {
                let label_el = div()
                    .when(link, |style| style.underline())
                    .child(label)
                    .into_any_element();
                match self.icon_pos {
                    IconPos::Start => el = el.child(icon_el).child(label_el),
                    IconPos::End => el = el.child(label_el).child(icon_el),
                }
            } else {
                el = el.child(icon_el);
            }
        } else if let Some(label) = self.label.clone() {
            el = el.child(div().when(link, |style| style.underline()).child(label));
        }

        if let Some(on_click) = self.on_click {
            el = el.on_click(move |event, window, cx| on_click(event, window, cx));
        }
        el
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Variants,
    Sizes,
    Icons,
    Rounded,
    Spinner,
    Group,
}

pub struct ButtonDemo {
    variant: String,
    dark: bool,
    outcome: Option<SharedString>,
}

impl ButtonDemo {
    fn new(variant: &str, dark: bool, _cx: &mut Context<Self>) -> Self {
        Self {
            variant: variant.to_string(),
            dark,
            outcome: None,
        }
    }

    /// Builds one demo button wired to report presses into `outcome`.
    fn press(&self, id: &str, label: &str, cx: &mut Context<Self>) -> Button {
        let weak = cx.entity().downgrade();
        let name: SharedString = label.to_string().into();
        Button::new(id.to_string())
            .label(name.clone())
            .dark(self.dark)
            .on_click(move |_, _, cx| {
                if let Some(demo) = weak.upgrade() {
                    let _ = demo.update(cx, |demo, cx| {
                        demo.outcome = Some(format!("Clicked {name}").into());
                        cx.notify();
                    });
                }
            })
    }

    /// Icon-only demo button; the a11y name is also the outcome label.
    fn press_icon(&self, id: &str, a11y: &str, cx: &mut Context<Self>) -> Button {
        let weak = cx.entity().downgrade();
        let name: SharedString = a11y.to_string().into();
        Button::new(id.to_string())
            .a11y_label(name.clone())
            .dark(self.dark)
            .on_click(move |_, _, cx| {
                if let Some(demo) = weak.upgrade() {
                    let _ = demo.update(cx, |demo, cx| {
                        demo.outcome = Some(format!("Clicked {name}").into());
                        cx.notify();
                    });
                }
            })
    }
}

impl Render for ButtonDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let kind = match self.variant.as_str() {
            "sizes" => DemoKind::Sizes,
            "icons" => DemoKind::Icons,
            "rounded" => DemoKind::Rounded,
            "spinner" => DemoKind::Spinner,
            "group" => DemoKind::Group,
            _ => DemoKind::Variants,
        };

        let row = || {
            div()
                .flex()
                .items_center()
                .justify_center()
                .gap_3()
                .flex_wrap()
        };
        let content: AnyElement = match kind {
            DemoKind::Variants => row()
                .child(self.press("v-default", "Default", cx))
                .child(
                    self.press("v-secondary", "Secondary", cx)
                        .variant(ButtonVariant::Secondary),
                )
                .child(
                    self.press("v-destructive", "Destructive", cx)
                        .variant(ButtonVariant::Destructive),
                )
                .child(
                    self.press("v-outline", "Outline", cx)
                        .variant(ButtonVariant::Outline),
                )
                .child(
                    self.press("v-ghost", "Ghost", cx)
                        .variant(ButtonVariant::Ghost),
                )
                .child(
                    self.press("v-link", "Link", cx)
                        .variant(ButtonVariant::Link),
                )
                .into_any_element(),
            DemoKind::Sizes => row()
                .child(
                    self.press("s-xs", "Extra Small", cx)
                        .size(ButtonSize::Xs)
                        .icon(ButtonIcon::ArrowUpRight, IconPos::End),
                )
                .child(self.press("s-sm", "Small", cx).size(ButtonSize::Sm))
                .child(self.press("s-default", "Default", cx))
                .child(self.press("s-lg", "Large", cx).size(ButtonSize::Lg))
                .into_any_element(),
            DemoKind::Icons => row()
                .child(
                    self.press("i-branch", "New Branch", cx)
                        .icon(ButtonIcon::GitBranch, IconPos::Start),
                )
                .child(
                    self.press("i-add", "Add", cx)
                        .variant(ButtonVariant::Outline)
                        .icon(ButtonIcon::Plus, IconPos::Start),
                )
                .child(
                    self.press_icon("i-icon", "Move up", cx)
                        .size(ButtonSize::Icon)
                        .variant(ButtonVariant::Outline)
                        .icon(ButtonIcon::ArrowUp, IconPos::Start),
                )
                .into_any_element(),
            DemoKind::Rounded => row()
                .child(
                    self.press("r-get", "Get Started", cx)
                        .pill(true)
                        .icon(ButtonIcon::ArrowUpRight, IconPos::End),
                )
                .child(
                    self.press_icon("r-icon", "Add item", cx)
                        .pill(true)
                        .size(ButtonSize::Icon)
                        .variant(ButtonVariant::Secondary)
                        .icon(ButtonIcon::Plus, IconPos::Start),
                )
                .into_any_element(),
            DemoKind::Spinner => row()
                .child(
                    self.press("sp-gen", "Generating", cx)
                        .icon(ButtonIcon::Spinner, IconPos::Start)
                        .disabled(true),
                )
                .child(
                    self.press("sp-dl", "Downloading", cx)
                        .variant(ButtonVariant::Secondary)
                        .icon(ButtonIcon::Spinner, IconPos::Start)
                        .disabled(true),
                )
                .into_any_element(),
            DemoKind::Group => div()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    self.press("g-archive", "Archive", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::Start),
                )
                .child(
                    self.press("g-report", "Report", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::Middle),
                )
                .child(
                    self.press("g-snooze", "Snooze", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::End),
                )
                .into_any_element(),
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(content)
                    .child(
                        div()
                            .h(px(18.))
                            .text_xs()
                            .text_color(muted)
                            .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
                    ),
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(400.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| ButtonDemo::new(&variant, dark, cx))
    })
    .expect("open button gallery");
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
    cx.new(|cx| ButtonDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{Button, ButtonIcon, ButtonJoin, ButtonSize, ButtonVariant, IconPos};

    #[test]
    fn builders_store_content() {
        let button = Button::new("b")
            .label("Deploy")
            .a11y_label("Deploy now")
            .variant(ButtonVariant::Destructive)
            .size(ButtonSize::Lg)
            .icon(ButtonIcon::Plus, IconPos::End)
            .pill(true)
            .join(ButtonJoin::Middle)
            .disabled(true)
            .dark(true);
        assert_eq!(button.label.as_deref(), Some("Deploy"));
        assert_eq!(button.a11y.as_deref(), Some("Deploy now"));
        assert_eq!(button.variant, ButtonVariant::Destructive);
        assert_eq!(button.size, ButtonSize::Lg);
        assert_eq!(button.icon, Some(ButtonIcon::Plus));
        assert_eq!(button.icon_pos, IconPos::End);
        assert!(button.pill && button.disabled && button.dark);
        assert_eq!(button.join, ButtonJoin::Middle);
    }
}
