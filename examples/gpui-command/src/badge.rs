//! shadcn/ui-style badge for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/badge
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

/// Emitted when a clickable (link-style) badge is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct BadgePressEvent {
    pub label: SharedString,
}

/// Visual style, mirroring shadcn's `variant` prop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

/// Optional inline icon.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeIcon {
    Check,
    Bookmark,
    ArrowUpRight,
    /// Continuously rotating arc; frozen when reduced motion is on.
    Spinner,
}

/// Which side of the label the icon sits on (`data-icon="inline-*"`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeIconPos {
    Start,
    End,
}

/// Tinted badge colors for the custom-color examples.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BadgeTone {
    Blue,
    Green,
    Sky,
    Purple,
    Red,
}

/// A styled badge entity. Keep it on the host view and render with
/// `.child(self.badge.clone())`.
pub struct Badge {
    label: SharedString,
    variant: BadgeVariant,
    icon: Option<BadgeIcon>,
    icon_pos: BadgeIconPos,
    tone: Option<BadgeTone>,
    clickable: bool,
    dark: bool,
    clock: Option<Box<dyn Fn() -> f32>>,
}
impl EventEmitter<BadgePressEvent> for Badge {}

impl Badge {
    pub fn new() -> Self {
        Self {
            label: SharedString::from("Badge"),
            variant: BadgeVariant::Default,
            icon: None,
            icon_pos: BadgeIconPos::Start,
            tone: None,
            clickable: false,
            dark: false,
            clock: None,
        }
    }
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = label.into();
        self
    }
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }
    /// Inline icon; `pos` mirrors `data-icon="inline-start|end"`.
    pub fn icon(mut self, icon: BadgeIcon, pos: BadgeIconPos) -> Self {
        self.icon = Some(icon);
        self.icon_pos = pos;
        self
    }
    /// Custom tinted color scheme (overrides `variant` surface/ink).
    pub fn tone(mut self, tone: BadgeTone) -> Self {
        self.tone = Some(tone);
        self
    }
    /// Link-style badges are focusable and pressable; pressing emits
    /// [`BadgePressEvent`].
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

fn tone_colors(tone: BadgeTone, dark: bool) -> (Rgba, Rgba) {
    let (bg, ink) = match (tone, dark) {
        (BadgeTone::Blue, false) => (0xdbeafe, 0x1d4ed8),
        (BadgeTone::Blue, true) => (0x1e3a5f, 0x93c5fd),
        (BadgeTone::Green, false) => (0xdcfce7, 0x15803d),
        (BadgeTone::Green, true) => (0x14532d, 0x86efac),
        (BadgeTone::Sky, false) => (0xe0f2fe, 0x0369a1),
        (BadgeTone::Sky, true) => (0x0c3a50, 0x7dd3fc),
        (BadgeTone::Purple, false) => (0xf3e8ff, 0x7e22ce),
        (BadgeTone::Purple, true) => (0x3b0764, 0xd8b4fe),
        (BadgeTone::Red, false) => (0xfee2e2, 0xb91c1c),
        (BadgeTone::Red, true) => (0x7f1d1d, 0xfca5a5),
    };
    (rgb(bg).into(), rgb(ink).into())
}

/// Rotating arc, like shadcn's badge spinner. `angle` turns 0..1.
fn spinner(angle: f32, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let center = o + point(px(s * 0.5), px(s * 0.5));
            let r = px(s * 0.36);
            // 240° sweep starting at `angle`.
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
}

fn badge_icon(icon: BadgeIcon, color: Hsla) -> impl IntoElement {
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
                BadgeIcon::Check => {
                    b.move_to(at(2.4, 6.2));
                    b.line_to(at(5., 8.8));
                    b.line_to(at(9.8, 3.4));
                }
                BadgeIcon::Bookmark => {
                    b.move_to(at(3.2, 1.8));
                    b.line_to(at(8.8, 1.8));
                    b.line_to(at(8.8, 10.4));
                    b.line_to(at(6., 8.4));
                    b.line_to(at(3.2, 10.4));
                    b.line_to(at(3.2, 1.8));
                }
                BadgeIcon::ArrowUpRight => {
                    b.move_to(at(3., 9.));
                    b.line_to(at(9., 3.));
                    b.move_to(at(4.6, 3.));
                    b.line_to(at(9., 3.));
                    b.line_to(at(9., 7.4));
                }
                BadgeIcon::Spinner => return,
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .into_any_element()
}

impl Render for Badge {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let spinning = self.icon == Some(BadgeIcon::Spinner);
        if self.clock.is_none() {
            let executor = cx.background_executor().clone();
            let start = executor.now();
            self.clock = Some(Box::new(move || {
                executor.now().duration_since(start).as_secs_f32()
            }));
        }
        if spinning && !cx.reduce_motion() {
            window.request_animation_frame();
        }
        let angle = if spinning {
            self.clock.as_ref().map_or(0., |clock| clock()) * 1.2 % 1.
        } else {
            0.
        };

        let dark = self.dark;
        let (mut surface, mut ink, mut edge, hover_underline) = match self.variant {
            BadgeVariant::Default => (
                rgb(if dark { 0xededed } else { 0x171717 }),
                rgb(if dark { 0x171717 } else { 0xffffff }),
                rgb(if dark { 0xededed } else { 0x171717 }),
                false,
            ),
            BadgeVariant::Secondary => (
                rgb(if dark { 0x333333 } else { 0xe4e4e4 }),
                rgb(if dark { 0xededed } else { 0x171717 }),
                rgb(if dark { 0x333333 } else { 0xe4e4e4 }),
                false,
            ),
            BadgeVariant::Destructive => (
                rgb(if dark { 0x8c2f26 } else { 0xc9372c }),
                rgb(0xffffff),
                rgb(if dark { 0x8c2f26 } else { 0xc9372c }),
                false,
            ),
            BadgeVariant::Outline => (
                rgb(if dark { 0x232323 } else { 0xffffff }),
                rgb(if dark { 0xededed } else { 0x171717 }),
                rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }),
                false,
            ),
            BadgeVariant::Ghost => (
                rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }),
                rgb(if dark { 0xededed } else { 0x171717 }),
                rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }),
                false,
            ),
            BadgeVariant::Link => (
                rgb(0x000000).opacity(0.),
                rgb(if dark { 0xededed } else { 0x171717 }),
                rgb(0x000000).opacity(0.),
                true,
            ),
        };
        if let Some(tone) = self.tone {
            let (bg, tone_ink) = tone_colors(tone, dark);
            surface = bg;
            ink = tone_ink;
            edge = bg;
        }

        let mut label = div()
            .font_family(FONT)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .text_color(ink)
            .whitespace_nowrap()
            .child(self.label.clone());
        if hover_underline {
            label = label.underline();
        }

        let mut badge = div()
            .flex_none()
            .flex()
            .items_center()
            .gap(px(4.))
            .rounded(px(6.))
            .pl(px(8.))
            .pr(px(8.))
            .h(px(22.))
            .bg(surface)
            .border_1()
            .border_color(edge);

        if let Some(icon) = self.icon {
            let icon_el = if icon == BadgeIcon::Spinner {
                spinner(angle, ink.into()).into_any_element()
            } else {
                badge_icon(icon, ink.into()).into_any_element()
            };
            let icon_box = div().flex_none().size(px(12.)).child(icon_el);
            badge = match self.icon_pos {
                BadgeIconPos::Start => badge.child(icon_box).child(label),
                BadgeIconPos::End => badge.child(label).child(icon_box),
            };
        } else {
            badge = badge.child(label);
        }

        if !self.clickable {
            return badge.into_any_element();
        }
        let focus = window
            .use_keyed_state(format!("badge-{}", self.label), cx, |_, cx| {
                cx.focus_handle()
            })
            .read(cx)
            .clone();
        let entity = cx.entity().downgrade();
        let name = self.label.clone();
        badge
            .id(format!("badge-{}", self.label))
            .track_focus(&focus.tab_index(0))
            .cursor_pointer()
            .focus_visible(|style| style.border_color(ink))
            .on_click(move |_, _, cx| {
                if let Some(badge) = entity.upgrade() {
                    let _ = badge.update(cx, |_, cx| {
                        cx.emit(BadgePressEvent {
                            label: name.clone(),
                        });
                        cx.notify();
                    });
                }
            })
            .into_any_element()
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Variants,
    Icons,
    Spinner,
    Link,
    Custom,
}

pub struct BadgeDemo {
    badges: Vec<Entity<Badge>>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl BadgeDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "icons" => DemoKind::Icons,
            "spinner" => DemoKind::Spinner,
            "link" => DemoKind::Link,
            "custom" => DemoKind::Custom,
            _ => DemoKind::Variants,
        };
        let d = |b: Badge| b.dark(dark);
        let specs = match kind {
            DemoKind::Variants => vec![
                d(Badge::new().label("Default")),
                d(Badge::new()
                    .label("Secondary")
                    .variant(BadgeVariant::Secondary)),
                d(Badge::new()
                    .label("Destructive")
                    .variant(BadgeVariant::Destructive)),
                d(Badge::new().label("Outline").variant(BadgeVariant::Outline)),
                d(Badge::new().label("Ghost").variant(BadgeVariant::Ghost)),
                d(Badge::new().label("Link").variant(BadgeVariant::Link)),
            ],
            DemoKind::Icons => vec![
                d(Badge::new()
                    .label("Verified")
                    .variant(BadgeVariant::Secondary)
                    .icon(BadgeIcon::Check, BadgeIconPos::Start)),
                d(Badge::new()
                    .label("Bookmark")
                    .variant(BadgeVariant::Outline)
                    .icon(BadgeIcon::Bookmark, BadgeIconPos::Start)),
                d(Badge::new()
                    .label("Trailing")
                    .variant(BadgeVariant::Secondary)
                    .icon(BadgeIcon::Check, BadgeIconPos::End)),
            ],
            DemoKind::Spinner => vec![
                d(Badge::new()
                    .label("Deleting")
                    .variant(BadgeVariant::Secondary)
                    .icon(BadgeIcon::Spinner, BadgeIconPos::Start)),
                d(Badge::new()
                    .label("Generating")
                    .icon(BadgeIcon::Spinner, BadgeIconPos::Start)),
            ],
            DemoKind::Link => vec![d(Badge::new()
                .label("Open Link")
                .variant(BadgeVariant::Link)
                .icon(BadgeIcon::ArrowUpRight, BadgeIconPos::End)
                .clickable(true))],
            DemoKind::Custom => vec![
                d(Badge::new()
                    .label("Blue")
                    .variant(BadgeVariant::Secondary)
                    .tone(BadgeTone::Blue)),
                d(Badge::new()
                    .label("Green")
                    .variant(BadgeVariant::Secondary)
                    .tone(BadgeTone::Green)),
                d(Badge::new()
                    .label("Sky")
                    .variant(BadgeVariant::Secondary)
                    .tone(BadgeTone::Sky)),
                d(Badge::new()
                    .label("Purple")
                    .variant(BadgeVariant::Secondary)
                    .tone(BadgeTone::Purple)),
                d(Badge::new()
                    .label("Red")
                    .variant(BadgeVariant::Secondary)
                    .tone(BadgeTone::Red)),
            ],
        };

        let mut subscriptions = Vec::new();
        let badges = specs
            .into_iter()
            .map(|badge| {
                let entity = cx.new(|_| badge);
                subscriptions.push(cx.subscribe(
                    &entity,
                    |demo, _, event: &BadgePressEvent, cx| {
                        demo.outcome = Some(format!("Clicked {}", event.label).into());
                        cx.notify();
                    },
                ));
                entity
            })
            .collect();
        Self {
            badges,
            dark,
            outcome: None,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for BadgeDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
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
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .children(self.badges.iter().cloned().map(IntoElement::into_element)),
                    )
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
        cx.new(|cx| BadgeDemo::new(&variant, dark, cx))
    })
    .expect("open badge gallery");
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
    cx.new(|cx| BadgeDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{Badge, BadgeIcon, BadgeIconPos, BadgeTone, BadgeVariant};

    #[test]
    fn builders_store_content() {
        let badge = Badge::new()
            .label("Verified")
            .variant(BadgeVariant::Destructive)
            .icon(BadgeIcon::Check, BadgeIconPos::End)
            .tone(BadgeTone::Purple)
            .clickable(true)
            .dark(true);
        assert_eq!(badge.label.as_ref(), "Verified");
        assert_eq!(badge.variant, BadgeVariant::Destructive);
        assert_eq!(badge.icon, Some(BadgeIcon::Check));
        assert_eq!(badge.icon_pos, BadgeIconPos::End);
        assert_eq!(badge.tone, Some(BadgeTone::Purple));
        assert!(badge.clickable && badge.dark);
    }
}
