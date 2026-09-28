//! shadcn/ui-style carousel for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/carousel (Embla).
use crate::button::{Button, ButtonIcon, ButtonSize, ButtonVariant, IconPos};
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";
/// Scroll animation duration in seconds.
const ANIM_SECS: f32 = 0.32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CarouselAxis {
    Horizontal,
    Vertical,
}

/// Emitted whenever the selected slide changes (button, key or loop wrap).
#[derive(Clone, Copy, Debug)]
pub struct CarouselSelectEvent {
    pub index: usize,
    pub count: usize,
}
impl gpui_kit::EventEmitter<CarouselSelectEvent> for Carousel {}

/// An Embla-style slide carousel. Items scroll one slide per nav click with a
/// cubic ease-out; `loop_(true)` wraps around seamlessly via a duplicated
/// track. Keyboard: focusable root, ←/→ (or ↑/↓ in vertical) scroll.
pub struct Carousel {
    index: usize,
    /// Current animated track offset in px (>= 0).
    offset: f32,
    /// Some((from_px, to_px, started_at_secs)) while animating.
    anim: Option<(f32, f32, f32)>,
    /// Lap size to subtract once a loop-wrap animation lands.
    normalize: f32,
    count: usize,
    per_view: usize,
    gap: f32,
    /// Slide extent along the scroll axis.
    slide: f32,
    /// Cross-axis extent (height for horizontal, width for vertical).
    cross: f32,
    axis: CarouselAxis,
    looped: bool,
    dark: bool,
    focus: FocusHandle,
    /// Elapsed-seconds clock — executor-backed so it works on wasm.
    clock: Option<Box<dyn Fn() -> f32>>,
}

impl Carousel {
    /// Create a carousel in an entity context (needs `cx` for the focus handle).
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            index: 0,
            offset: 0.,
            anim: None,
            normalize: 0.,
            count: 5,
            per_view: 1,
            gap: 16.,
            slide: 336.,
            cross: 200.,
            axis: CarouselAxis::Horizontal,
            looped: false,
            dark: false,
            focus: cx.focus_handle(),
            clock: None,
        }
    }
    /// Number of slides (demo slides are auto-generated 1..=count).
    pub fn count(mut self, count: usize) -> Self {
        self.count = count.max(1);
        self
    }
    /// Slides visible per viewport — Embla's `basis-1/n`.
    pub fn per_view(mut self, n: usize) -> Self {
        self.per_view = n.clamp(1, self.count.max(1));
        self
    }
    /// Gap between slides — Embla's `pl-*` on items.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
    /// Slide extent along the scroll axis (width horizontal / height vertical).
    pub fn slide_px(mut self, px: f32) -> Self {
        self.slide = px;
        self
    }
    /// Cross-axis extent (height horizontal / width vertical).
    pub fn cross_px(mut self, px: f32) -> Self {
        self.cross = px;
        self
    }
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.axis = if vertical {
            CarouselAxis::Vertical
        } else {
            CarouselAxis::Horizontal
        };
        self
    }
    /// Embla `opts.loop` — wraps past the ends seamlessly.
    pub fn loop_(mut self, looped: bool) -> Self {
        self.looped = looped;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    fn stride(&self) -> f32 {
        self.slide + self.gap
    }
    fn total(&self) -> f32 {
        self.count as f32 * self.stride()
    }
    /// Highest reachable index without looping.
    fn max_index(&self) -> usize {
        self.count.saturating_sub(self.per_view)
    }
    fn can_prev(&self) -> bool {
        self.looped || self.index > 0
    }
    fn can_next(&self) -> bool {
        self.looped || self.index < self.max_index()
    }

    fn go(&mut self, delta: i64, cx: &mut Context<Self>) {
        if self.count <= self.per_view && !self.looped {
            return;
        }
        let from = self.offset;
        // Compute the visual target offset (may overshoot by one slide when
        // wrapping so the motion stays continuous) and the index after wrap.
        let (target, index, normalize) = if self.looped {
            let next = self.index as i64 + delta;
            let index = next.rem_euclid(self.count as i64) as usize;
            let mut target = index as f32 * self.stride();
            // Keep the wrap animation monotonic: if the raw target would go
            // backwards when moving forwards (or vice versa) push it one lap.
            if delta > 0 && target <= from {
                target += self.total();
            } else if delta < 0 && target >= from {
                target -= self.total();
            }
            (target, index, self.total())
        } else {
            let index = (self.index as i64 + delta).clamp(0, self.max_index() as i64) as usize;
            if index == self.index {
                return;
            }
            (index as f32 * self.stride(), index, 0.)
        };
        self.index = index;
        self.anim = Some((from, target, self.secs()));
        self.normalize = normalize;
        cx.emit(CarouselSelectEvent {
            index,
            count: self.count,
        });
        cx.notify();
    }

    fn secs(&mut self) -> f32 {
        self.clock.as_ref().map(|c| c()).unwrap_or(0.)
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let (back, fwd) = match self.axis {
            CarouselAxis::Horizontal => ("left", "right"),
            CarouselAxis::Vertical => ("up", "down"),
        };
        let key = event.keystroke.key.as_str();
        let delta = if key == back {
            -1
        } else if key == fwd {
            1
        } else {
            return;
        };
        self.go(delta, cx);
        window.prevent_default();
        cx.stop_propagation();
    }
}

/// One demo slide — a rounded card with its 1-based number, like the shadcn
/// docs' `<Card>` slides.
fn carousel_slide(
    label: usize,
    slide: f32,
    cross: f32,
    axis: CarouselAxis,
    dark: bool,
) -> AnyElement {
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    let surface: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let mut el = div()
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .bg(surface)
        .border_1()
        .border_color(edge)
        .rounded_lg()
        .font_family(FONT)
        .text_size(px(48.))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(ink)
        .child(label.to_string());
    el = match axis {
        CarouselAxis::Horizontal => el.w(px(slide)).h(px(cross)),
        CarouselAxis::Vertical => el.h(px(slide)).w(px(cross)),
    };
    el.into_any_element()
}

fn nav_button(
    id: &str,
    a11y: &str,
    icon: ButtonIcon,
    enabled: bool,
    axis: CarouselAxis,
    prev: bool,
    dark: bool,
    weak: WeakEntity<Carousel>,
    cross: f32,
) -> Div {
    let btn = Button::new(SharedString::from(id.to_string()))
        .a11y_label(a11y)
        .icon(icon, IconPos::Start)
        .variant(ButtonVariant::Outline)
        .size(ButtonSize::Icon)
        .pill(true)
        .disabled(!enabled)
        .dark(dark)
        .on_click(move |_, _, cx| {
            if let Some(car) = weak.upgrade() {
                let _ = car.update(cx, |this, cx| this.go(if prev { -1 } else { 1 }, cx));
            }
        });
    // Positioned outside the viewport like shadcn's -left-12 / -right-12;
    // vertical carousels stack them top/bottom instead.
    let center = px(cross / 2. - 16.);
    let mut pos = div().absolute();
    pos = match (axis, prev) {
        (CarouselAxis::Horizontal, true) => pos.left(px(-44.)).top(center),
        (CarouselAxis::Horizontal, false) => pos.right(px(-44.)).top(center),
        (CarouselAxis::Vertical, true) => pos.top(px(-44.)).left(center),
        (CarouselAxis::Vertical, false) => pos.bottom(px(-44.)).left(center),
    };
    pos.child(btn)
}

impl Render for Carousel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.clock.is_none() {
            let executor = cx.background_executor().clone();
            let start = executor.now();
            self.clock = Some(Box::new(move || {
                executor.now().duration_since(start).as_secs_f32()
            }));
        }
        // Advance the scroll animation; ease-out cubic like Embla.
        if let Some((from, target, start)) = self.anim {
            let t = ((self.secs() - start) / ANIM_SECS).clamp(0., 1.);
            let eased = 1. - (1. - t).powi(3);
            self.offset = from + (target - from) * eased;
            if t >= 1. {
                self.offset = target;
                if self.normalize != 0. {
                    // Snap the offset back into the first lap — the duplicated
                    // track makes the wrap invisible.
                    self.offset = self.offset.rem_euclid(self.total());
                    self.normalize = 0.;
                }
                self.anim = None;
            } else {
                window.request_animation_frame();
            }
        }

        let dark = self.dark;
        let axis = self.axis;
        // Duplicated slides when looping — the second lap lets the track slide
        // past the end without blank space.
        let laps = if self.looped { 2 } else { 1 };
        let slides = (0..self.count * laps)
            .map(|i| carousel_slide(i % self.count + 1, self.slide, self.cross, axis, dark))
            .collect::<Vec<_>>();

        let track = match axis {
            CarouselAxis::Horizontal => div()
                .flex()
                .flex_row()
                .h_full()
                .gap(px(self.gap))
                .ml(px(-self.offset))
                .children(slides),
            CarouselAxis::Vertical => div()
                .flex()
                .flex_col()
                .w_full()
                .gap(px(self.gap))
                .mt(px(-self.offset))
                .children(slides),
        };

        let (w, h) = match axis {
            CarouselAxis::Horizontal => (
                self.slide * self.per_view as f32 + self.gap * (self.per_view - 1) as f32,
                self.cross,
            ),
            CarouselAxis::Vertical => (
                self.cross,
                self.slide * self.per_view as f32 + self.gap * (self.per_view - 1) as f32,
            ),
        };

        let weak = cx.entity().downgrade();
        let (p_icon, n_icon) = match axis {
            CarouselAxis::Horizontal => (ButtonIcon::ArrowLeft, ButtonIcon::ArrowRight),
            CarouselAxis::Vertical => (ButtonIcon::ArrowUp, ButtonIcon::ArrowDown),
        };

        div()
            .relative()
            .w(px(w))
            .h(px(h))
            .track_focus(&self.focus)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.focus.focus(window, cx)),
            )
            .on_key_down(cx.listener(Self::key_down))
            .child(div().size_full().overflow_hidden().child(track))
            .child(nav_button(
                "car-prev",
                "Previous slide",
                p_icon,
                self.can_prev(),
                axis,
                true,
                dark,
                weak.clone(),
                self.cross,
            ))
            .child(nav_button(
                "car-next",
                "Next slide",
                n_icon,
                self.can_next(),
                axis,
                false,
                dark,
                weak,
                self.cross,
            ))
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Sizes,
    Spacing,
    Vertical,
    Loop,
}

pub struct CarouselDemo {
    dark: bool,
    carousel: Entity<Carousel>,
    outcome: Option<SharedString>,
}

impl CarouselDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "sizes" => DemoKind::Sizes,
            "spacing" => DemoKind::Spacing,
            "vertical" => DemoKind::Vertical,
            "loop" => DemoKind::Loop,
            _ => DemoKind::Basic,
        };
        let carousel = cx.new(|cx| {
            let mut car = Carousel::new(cx).dark(dark).count(5);
            match kind {
                DemoKind::Basic => {}
                // basis-1/3: three slides per viewport.
                DemoKind::Sizes => car = car.per_view(3).slide_px(104.).gap(16.),
                // Wider pl-*: visible gutters between slides.
                DemoKind::Spacing => car = car.per_view(2).slide_px(160.).gap(32.),
                DemoKind::Vertical => car = car.vertical(true).slide_px(160.).cross_px(320.),
                DemoKind::Loop => car = car.loop_(true),
            }
            car
        });
        cx.subscribe(
            &carousel,
            |this: &mut Self, _car, event: &CarouselSelectEvent, cx| {
                this.outcome = Some(format!("Slide {} of {}", event.index + 1, event.count).into());
                cx.notify();
            },
        )
        .detach();
        Self {
            dark,
            carousel,
            outcome: None,
        }
    }
}

impl Render for CarouselDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(self.carousel.clone())
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
        window_bounds: Some(WindowBounds::centered(size(px(620.), px(420.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| CarouselDemo::new(&variant, dark, cx))
    })
    .expect("open carousel gallery");
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
pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}
