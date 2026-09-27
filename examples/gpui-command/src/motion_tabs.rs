//! Native GPUI adaptation of beUI Motion Tabs (MIT).
//! https://beui.dev/components/motion/tabs
//! Desktop and WASM share the same native text, paths, clipping, and springs.
use gpui_kit::{self as kit, *};
use kit::base::{Button, ElementExt as _, Tab, Tabs};
use kit::prelude::FluentBuilder as _;
use std::{borrow::Cow, cell::Cell, rc::Rc};

#[derive(Clone, Debug)]
pub struct TabItem {
    value: SharedString,
    label: SharedString,
    content: Option<SharedString>,
    disabled: bool,
}
impl TabItem {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            content: None,
            disabled: false,
        }
    }
    pub fn content(mut self, text: impl Into<SharedString>) -> Self {
        self.content = Some(text.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabsVariant {
    #[default]
    Pill,
    Segment,
    Underline,
}
impl TabsVariant {
    fn padding(self) -> f32 {
        match self {
            Self::Pill => 4.,
            Self::Segment => 2.,
            Self::Underline => 0.,
        }
    }
    fn gap(self) -> f32 {
        if self == Self::Segment { 0. } else { 4. }
    }
    fn height(self) -> f32 {
        match self {
            Self::Pill => 40.,
            Self::Segment => 36.,
            Self::Underline => 44.,
        }
    }
    fn trigger_padding(self) -> f32 {
        if self == Self::Underline { 12. } else { 14. }
    }
    fn radius(self) -> f32 {
        match self {
            Self::Pill => 999.,
            Self::Segment => 8.,
            Self::Underline => 0.,
        }
    }
}

/// The source's 245 / 36 / 1.2 spring is overdamped, with two real roots.
/// Analytical integration retains velocity on interruption and is independent
/// of frame rate, including the first frame after a long idle interval.
#[derive(Clone, Copy, Debug)]
struct Spring {
    value: f32,
    velocity: f32,
    target: f32,
}
impl Spring {
    fn new(value: f32) -> Self {
        Self {
            value,
            velocity: 0.,
            target: value,
        }
    }
    fn advance(&mut self, dt: f32) {
        if dt <= 0. {
            return;
        }
        let decay = 36. / (2. * 1.2);
        let discriminant = (decay * decay - 245. / 1.2_f32).sqrt();
        let r1 = -decay + discriminant;
        let r2 = -decay - discriminant;
        let displacement = self.value - self.target;
        let a = (self.velocity - r2 * displacement) / (r1 - r2);
        let b = displacement - a;
        let a = a * (r1 * dt).exp();
        let b = b * (r2 * dt).exp();
        self.value = self.target + a + b;
        self.velocity = r1 * a + r2 * b;
        if (self.value - self.target).abs() < 0.001 && self.velocity.abs() < 0.01 {
            self.snap();
        }
    }
    fn snap(&mut self) {
        self.value = self.target;
        self.velocity = 0.;
    }
    fn active(self) -> bool {
        self.value != self.target || self.velocity != 0.
    }
}

fn ease_out(t: f32) -> f32 {
    if t <= 0. {
        return 0.;
    }
    if t >= 1. {
        return 1.;
    }
    let cubic = |s: f32, a: f32, b: f32| {
        3. * (1. - s).powi(2) * s * a + 3. * (1. - s) * s * s * b + s * s * s
    };
    let (mut lo, mut hi) = (0., 1.);
    for _ in 0..18 {
        let mid = (lo + hi) / 2.;
        if cubic(mid, 0.16, 0.3) < t {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    cubic((lo + hi) / 2., 1., 1.)
}

#[derive(Clone, Copy, Debug)]
struct Slot {
    x: f32,
    width: f32,
}

fn navigate(enabled: &[bool], current: usize, key: &str) -> Option<usize> {
    if enabled.is_empty() {
        return None;
    }
    match key {
        "home" => enabled.iter().position(|v| *v),
        "end" => enabled.iter().rposition(|v| *v),
        "left" | "right" => {
            let n = enabled.len();
            (1..=n)
                .map(|step| {
                    if key == "right" {
                        (current + step) % n
                    } else {
                        (current + n - step % n) % n
                    }
                })
                .find(|&i| enabled[i])
        }
        _ => None,
    }
}

/// Overlay arrows occupy no layout space. Only an existing edge reserves the
/// source's 36 px clearance when revealing a newly selected or focused item.
fn reveal_offset(slot: Slot, scroll: f32, viewport: f32, total: f32) -> f32 {
    let max = (total - viewport).max(0.);
    let scroll = scroll.clamp(0., max);
    let left = scroll + if scroll > 1. { 36. } else { 0. };
    let right = scroll + viewport - if scroll < max - 1. { 36. } else { 0. };
    let delta = if slot.x < left {
        slot.x - left
    } else if slot.x + slot.width > right {
        slot.x + slot.width - right
    } else {
        0.
    };
    (scroll + delta).clamp(0., max)
}

fn shape(text: SharedString, color: Hsla, weight: FontWeight, window: &mut Window) -> ShapedLine {
    window.text_system().shape_line(
        text.clone(),
        px(14.),
        &[TextRun {
            len: text.len(),
            font: Font {
                family: "Geist".into(),
                weight,
                ..Default::default()
            },
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }],
        None,
    )
}

#[derive(Clone)]
struct Label {
    normal: ShapedLine,
    hover: ShapedLine,
    white: ShapedLine,
}
struct Geometry {
    slots: Vec<Slot>,
    labels: Vec<Label>,
    width: f32,
}
type ChangeHandler = Rc<dyn Fn(&str, &mut Window, &mut Context<MotionTabs>)>;

/// A controlled or locally managed native tab list with roving keyboard focus.
/// `set_selected` changes selection without invoking the user-action callback.
pub struct MotionTabs {
    items: Vec<TabItem>,
    variant: TabsVariant,
    selected: Option<usize>,
    dark: bool,
    max_width: f32,
    on_change: Option<ChangeHandler>,
    geometry: Option<Geometry>,
    focus: Vec<FocusHandle>,
    focus_subscriptions: Vec<Subscription>,
    hovered: Option<usize>,
    pressed_key: Option<(usize, String)>,
    x: Spring,
    width: Spring,
    scroll: Spring,
    viewport: f32,
    viewport_measurement: Rc<Cell<f32>>,
    clock: Option<Box<dyn Fn() -> f32>>,
    last_frame: f32,
    panel_started: f32,
}
impl MotionTabs {
    pub fn new(items: impl IntoIterator<Item = TabItem>) -> Self {
        let items: Vec<_> = items.into_iter().collect();
        let selected = items.iter().position(|item| !item.disabled);
        Self {
            items,
            variant: TabsVariant::Pill,
            selected,
            dark: false,
            max_width: f32::INFINITY,
            on_change: None,
            geometry: None,
            focus: vec![],
            focus_subscriptions: vec![],
            hovered: None,
            pressed_key: None,
            x: Spring::new(0.),
            width: Spring::new(0.),
            scroll: Spring::new(0.),
            viewport: 0.,
            viewport_measurement: Rc::new(Cell::new(0.)),
            clock: None,
            last_frame: 0.,
            panel_started: 0.,
        }
    }
    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        let value = value.into();
        self.selected = self
            .items
            .iter()
            .position(|item| item.value == value && !item.disabled)
            .or(self.selected);
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width.max(1.);
        self
    }
    pub fn on_change(
        mut self,
        callback: impl Fn(&str, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(callback));
        self
    }
    pub fn selected_value(&self) -> Option<&str> {
        self.selected.map(|i| self.items[i].value.as_ref())
    }
    pub fn set_selected(&mut self, value: impl AsRef<str>, cx: &mut Context<Self>) {
        if let Some(i) = self
            .items
            .iter()
            .position(|item| item.value.as_ref() == value.as_ref() && !item.disabled)
        {
            self.select(i, cx);
        }
    }
    fn now(&self) -> f32 {
        self.clock.as_ref().map_or(0., |clock| clock())
    }
    fn advance(&mut self, reduce: bool) {
        let now = self.now();
        let dt = (now - self.last_frame).max(0.);
        self.last_frame = now;
        for spring in [&mut self.x, &mut self.width, &mut self.scroll] {
            if reduce {
                spring.snap();
            } else {
                spring.advance(dt);
            }
        }
    }
    fn reveal(&mut self, index: usize, reduce: bool) {
        if let Some(geometry) = &self.geometry {
            self.scroll.target = reveal_offset(
                geometry.slots[index],
                self.scroll.value,
                self.viewport,
                geometry.width,
            );
            if reduce {
                self.scroll.snap();
            }
        }
    }
    fn select(&mut self, index: usize, cx: &mut Context<Self>) -> bool {
        if self.items[index].disabled || self.selected == Some(index) {
            return false;
        }
        self.advance(cx.reduce_motion());
        self.selected = Some(index);
        self.panel_started = self.now();
        if let Some(geometry) = &self.geometry {
            self.x.target = geometry.slots[index].x;
            self.width.target = geometry.slots[index].width;
        }
        self.reveal(index, cx.reduce_motion());
        cx.notify();
        true
    }
    fn activate(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.items[index].disabled {
            return;
        }
        self.focus[index].focus(window, cx);
        let changed = self.select(index, cx);
        if !changed {
            self.advance(cx.reduce_motion());
            self.reveal(index, cx.reduce_motion());
            cx.notify();
        }
        if changed && let Some(callback) = self.on_change.clone() {
            let value = self.items[index].value.clone();
            callback(value.as_ref(), window, cx);
        }
    }
    fn scroll_by(&mut self, delta: f32, immediate: bool, cx: &mut Context<Self>) {
        self.advance(cx.reduce_motion());
        let total = self.geometry.as_ref().map_or(0., |g| g.width);
        self.scroll.target =
            (self.scroll.target + delta).clamp(0., (total - self.viewport).max(0.));
        if immediate || cx.reduce_motion() {
            self.scroll.snap();
        }
        cx.notify();
    }
    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.modifiers.modified() {
            return;
        }
        let Some(index) = self
            .focus
            .iter()
            .position(|handle| handle.is_focused(window))
        else {
            return;
        };
        let key = event.keystroke.key.as_str();
        if let Some(next) = navigate(
            &self
                .items
                .iter()
                .map(|item| !item.disabled)
                .collect::<Vec<_>>(),
            index,
            key,
        ) {
            self.pressed_key = None;
            self.activate(next, window, cx);
        } else if matches!(key, "enter" | "space") {
            self.pressed_key = Some((index, key.to_string()));
        } else {
            return;
        }
        window.prevent_default();
        cx.stop_propagation();
    }
    fn key_up(&mut self, event: &KeyUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((index, key)) = self.pressed_key.take() {
            if key == event.keystroke.key && self.focus[index].is_focused(window) {
                self.activate(index, window, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        }
    }
    fn ensure_geometry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.geometry.is_some() {
            return;
        }
        let normal: Hsla = rgb(if self.dark { 0x868686 } else { 0x636363 }).into();
        let foreground: Hsla = rgb(if self.dark { 0xf2f2f2 } else { 0x0b0b0b }).into();
        let mut x = self.variant.padding();
        let mut labels = Vec::with_capacity(self.items.len());
        let mut slots = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let normal = shape(item.label.clone(), normal, FontWeight::MEDIUM, window);
            let width = f32::from(normal.width()) + 2. * self.variant.trigger_padding();
            slots.push(Slot { x, width });
            labels.push(Label {
                normal,
                hover: shape(item.label.clone(), foreground, FontWeight::MEDIUM, window),
                white: shape(
                    item.label.clone(),
                    rgb(0xffffff).into(),
                    FontWeight::MEDIUM,
                    window,
                ),
            });
            x += width + self.variant.gap();
        }
        let width = x - if self.items.is_empty() {
            0.
        } else {
            self.variant.gap()
        } + self.variant.padding();
        self.viewport = width.min(self.max_width).max(1.);
        self.viewport_measurement.set(self.viewport);
        self.geometry = Some(Geometry {
            slots,
            labels,
            width,
        });
        self.focus = self.items.iter().map(|_| cx.focus_handle()).collect();
        self.focus_subscriptions = self
            .focus
            .iter()
            .enumerate()
            .map(|(index, handle)| {
                cx.on_focus(handle, window, move |this, _, cx| {
                    this.advance(cx.reduce_motion());
                    this.reveal(index, cx.reduce_motion());
                    cx.notify();
                })
            })
            .collect();
        if let Some(i) = self.selected {
            let slot = self.geometry.as_ref().unwrap().slots[i];
            self.x = Spring::new(slot.x);
            self.width = Spring::new(slot.width);
            self.reveal(i, true);
        }
    }
}

/// Translate the complete tab hitbox subtree without changing its layout size.
/// Painting below uses the unsnapped scroll value, retaining subpixel motion.
struct ScrolledTabs {
    child: AnyElement,
    offset: Pixels,
}
impl IntoElement for ScrolledTabs {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for ScrolledTabs {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        window.with_element_offset(point(self.offset, px(0.)), |window| {
            self.child.prepaint(window, cx);
        });
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}

fn paint_rect(
    bounds: Bounds<Pixels>,
    radius: f32,
    color: impl Into<Background>,
    window: &mut Window,
) {
    let radius = radius
        .min(f32::from(bounds.size.width) / 2.)
        .min(f32::from(bounds.size.height) / 2.);
    window.paint_quad(quad(
        bounds,
        px(radius),
        color,
        px(0.),
        transparent_black(),
        BorderStyle::Solid,
    ));
}
fn paint_indicator(bounds: Bounds<Pixels>, radius: f32, window: &mut Window) {
    // Quads snap their bounds to device pixels in GPUI. A native path retains
    // fractional spring positions and widths without allocating SVG textures.
    let left = f32::from(bounds.origin.x);
    let top = f32::from(bounds.origin.y);
    let right = left + f32::from(bounds.size.width);
    let bottom = top + f32::from(bounds.size.height);
    let radius = radius
        .min((right - left).max(0.) / 2.)
        .min((bottom - top).max(0.) / 2.);
    let k = radius * 0.5522848;
    let p = |x, y| point(px(x), px(y));
    let mut path = PathBuilder::fill();
    path.move_to(p(left + radius, top));
    path.line_to(p(right - radius, top));
    path.cubic_bezier_to(
        p(right, top + radius),
        p(right - radius + k, top),
        p(right, top + radius - k),
    );
    path.line_to(p(right, bottom - radius));
    path.cubic_bezier_to(
        p(right - radius, bottom),
        p(right, bottom - radius + k),
        p(right - radius + k, bottom),
    );
    path.line_to(p(left + radius, bottom));
    path.cubic_bezier_to(
        p(left, bottom - radius),
        p(left + radius - k, bottom),
        p(left, bottom - radius + k),
    );
    path.line_to(p(left, top + radius));
    path.cubic_bezier_to(
        p(left + radius, top),
        p(left, top + radius - k),
        p(left + radius - k, top),
    );
    path.close();
    if let Ok(path) = path.build() {
        window.paint_path(path, rgb(0x0285f7));
    }
}
fn paint_chevron(bounds: Bounds<Pixels>, left: bool, dark: bool, window: &mut Window) {
    let c = bounds.center();
    let sign = if left { -1. } else { 1. };
    let mut path = PathBuilder::stroke(px(1.667));
    path.move_to(c + point(px(-2.5 * sign), px(-5.)));
    path.line_to(c + point(px(2.5 * sign), px(0.)));
    path.line_to(c + point(px(-2.5 * sign), px(5.)));
    if let Ok(path) = path.build() {
        window.paint_path(path, rgb(if dark { 0xf2f2f2 } else { 0x0b0b0b }));
    }
}

impl Render for MotionTabs {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.clock.is_none() {
            let executor = cx.background_executor().clone();
            let start = executor.now();
            self.clock = Some(Box::new(move || {
                executor.now().duration_since(start).as_secs_f32()
            }));
        }
        self.ensure_geometry(window, cx);
        let reduce = cx.reduce_motion();
        self.advance(reduce);
        let measured = self.viewport_measurement.get();
        if (measured - self.viewport).abs() > 0.1 {
            self.viewport = measured.max(1.);
            if let Some(index) = self.selected {
                self.reveal(index, reduce);
            }
        }
        let geometry = self.geometry.as_ref().unwrap();
        let natural_width = geometry.width;
        let viewport = self.viewport;
        let desired_width = natural_width.min(self.max_width).max(1.);
        let variant = self.variant;
        let height = variant.height();
        let dark = self.dark;
        let scroll = self
            .scroll
            .value
            .clamp(0., (natural_width - viewport).max(0.));
        let left_edge = scroll > 1.;
        let right_edge = scroll < natural_width - viewport - 1.;
        let selected = self.selected;
        let hovered = self.hovered;
        let slots = geometry.slots.clone();
        let labels = geometry.labels.clone();
        let disabled: Vec<_> = self.items.iter().map(|item| item.disabled).collect();
        let indicator_x = self.x.value;
        let indicator_width = self.width.value;
        let panel = selected.and_then(|i| self.items[i].content.clone());
        let panel_progress = if reduce {
            1.
        } else {
            ease_out((self.now() - self.panel_started) / 0.18)
        };
        let animating = self.x.active()
            || self.width.active()
            || self.scroll.active()
            || (panel.is_some() && panel_progress < 1.);
        if animating {
            let weak = cx.entity().downgrade();
            window.on_next_frame(move |_, cx| {
                let _ = weak.update(cx, |_, cx| cx.notify());
            });
        }
        let painting = canvas(
            |_, _, _| (),
            move |bounds, _, window, cx| {
                let origin = bounds.origin;
                let surface: Hsla = rgb(if dark { 0x1c1c1c } else { 0xf5f5f5 }).into();
                if variant != TabsVariant::Underline {
                    paint_rect(bounds, variant.radius(), surface, window);
                } else {
                    let mut border: Hsla = rgb(if dark { 0xffffff } else { 0x0b0b0b }).into();
                    border.a = if dark { 0.05 } else { 0.06 };
                    paint_rect(
                        Bounds::new(
                            origin + point(px(0.), px(height - 1.)),
                            size(bounds.size.width, px(1.)),
                        ),
                        0.,
                        border,
                        window,
                    );
                }
                window.with_content_mask(Some(ContentMask { bounds }), |window| {
                    let indicator = Bounds::new(
                        origin
                            + point(
                                px(indicator_x - scroll),
                                px(if variant == TabsVariant::Underline {
                                    height - 1.
                                } else {
                                    variant.padding()
                                }),
                            ),
                        size(
                            px(indicator_width),
                            px(if variant == TabsVariant::Underline {
                                1.
                            } else {
                                32.
                            }),
                        ),
                    );
                    if selected.is_some() {
                        paint_indicator(indicator, variant.radius(), window);
                    }
                    for (i, (slot, label)) in slots.iter().zip(labels.iter()).enumerate() {
                        let x = f32::from(origin.x) + slot.x - scroll + variant.trigger_padding();
                        let y = f32::from(origin.y)
                            + if variant == TabsVariant::Underline {
                                9.
                            } else {
                                variant.padding() + 6.
                            };
                        let position = point(px(x), px(y));
                        let line = if hovered == Some(i)
                            || (variant == TabsVariant::Underline && selected == Some(i))
                        {
                            &label.hover
                        } else {
                            &label.normal
                        };
                        if disabled[i] {
                            let mut color: Hsla =
                                rgb(if dark { 0x868686 } else { 0x636363 }).into();
                            color.a = 0.5;
                            let text = line.text.clone();
                            let _ = shape(text, color, FontWeight::MEDIUM, window).paint(
                                position,
                                px(20.),
                                TextAlign::Left,
                                None,
                                window,
                                cx,
                            );
                        } else {
                            let _ =
                                line.paint(position, px(20.), TextAlign::Left, None, window, cx);
                        }
                        if variant != TabsVariant::Underline && selected.is_some() {
                            // Every label, including intermediate tabs crossed by a
                            // long glide, is clipped against the actual indicator.
                            let clip_left = (slot.x - scroll).max(indicator_x - scroll);
                            let clip_right = (slot.x + slot.width - scroll)
                                .min(indicator_x + indicator_width - scroll);
                            if clip_right > clip_left {
                                let clip = Bounds::new(
                                    origin + point(px(clip_left), px(0.)),
                                    size(px(clip_right - clip_left), px(height)),
                                );
                                window.with_content_mask(
                                    Some(ContentMask { bounds: clip }),
                                    |window| {
                                        let _ = label.white.paint(
                                            position,
                                            px(20.),
                                            TextAlign::Left,
                                            None,
                                            window,
                                            cx,
                                        );
                                    },
                                );
                            }
                        }
                    }
                    // The source masks the overflowing content into its card
                    // surface. Native gradients give the same fade without CPU
                    // rasterizing a backdrop-filter for every scroll frame.
                    let surface = if variant == TabsVariant::Underline {
                        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into()
                    } else {
                        surface
                    };
                    let mut clear: Hsla = surface;
                    clear.a = 0.;
                    if left_edge {
                        paint_rect(
                            Bounds::new(origin, size(px(40.), px(height))),
                            0.,
                            linear_gradient(
                                90.,
                                linear_color_stop(surface, 0.),
                                linear_color_stop(clear, 1.),
                            ),
                            window,
                        );
                    }
                    if right_edge {
                        paint_rect(
                            Bounds::new(
                                origin + point(bounds.size.width - px(40.), px(0.)),
                                size(px(40.), px(height)),
                            ),
                            0.,
                            linear_gradient(
                                90.,
                                linear_color_stop(clear, 0.),
                                linear_color_stop(surface, 1.),
                            ),
                            window,
                        );
                    }
                });
            },
        )
        .absolute()
        .size_full();

        let mut row = div()
            .absolute()
            .left_0()
            .top_0()
            .flex()
            .w(px(natural_width))
            .h(px(height))
            .px(px(variant.padding()))
            .gap(px(variant.gap()));
        for (index, item) in self.items.iter().enumerate() {
            let focus = self.focus[index].clone();
            row = row.child(
                Tab::new(("motion-tab", index))
                    .selected(selected == Some(index))
                    .disabled(item.disabled)
                    .accessibility_label(item.label.clone())
                    .set_position(index + 1, self.items.len())
                    .track_focus(&focus)
                    .tab_index(index as isize)
                    .tab_stop(selected == Some(index) && !item.disabled)
                    .w(px(geometry.slots[index].width))
                    .h(px(if variant == TabsVariant::Underline {
                        44.
                    } else {
                        32.
                    }))
                    .mt(px(variant.padding()))
                    .flex_shrink_0()
                    .rounded(px(variant.radius().min(16.)))
                    .when(!item.disabled, |tab| tab.cursor_pointer())
                    .focus_visible(|style| {
                        style.shadow(vec![
                            BoxShadow::new(px(0.), px(0.), rgba(0x0285f780).into())
                                .spread_radius(px(2.)),
                        ])
                    })
                    .on_hover(cx.listener(move |this, hovered, _, cx| {
                        this.hovered = if *hovered {
                            Some(index)
                        } else if this.hovered == Some(index) {
                            None
                        } else {
                            this.hovered
                        };
                        cx.notify();
                    }))
                    .on_click(
                        cx.listener(move |this, _, window, cx| this.activate(index, window, cx)),
                    ),
            );
        }
        let measurement = self.viewport_measurement.clone();
        let weak = cx.entity().downgrade();
        let mut strip = Tabs::new("motion-tabs-list")
            .relative()
            .w_full()
            .h(px(height))
            .overflow_hidden()
            .rounded(px(variant.radius().min(height / 2.)))
            .on_key_down(cx.listener(Self::key_down))
            .on_key_up(cx.listener(Self::key_up))
            .on_scroll_wheel(cx.listener(|this, event: &ScrollWheelEvent, window, cx| {
                let delta = event.delta.pixel_delta(px(20.));
                let dx = if event.modifiers.shift && f32::from(delta.x).abs() < 0.1 {
                    f32::from(delta.y)
                } else {
                    f32::from(delta.x)
                };
                if dx.abs() > 0.
                    && this
                        .geometry
                        .as_ref()
                        .is_some_and(|g| g.width > this.viewport + 1.)
                {
                    this.scroll_by(-dx, true, cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
            }))
            .on_prepaint(move |bounds, window, _| {
                let width = f32::from(bounds.size.width);
                if (measurement.replace(width) - width).abs() > 0.1 {
                    let weak = weak.clone();
                    window.on_next_frame(move |_, cx| {
                        let _ = weak.update(cx, |_, cx| cx.notify());
                    });
                }
            })
            .child(painting)
            .child(ScrolledTabs {
                child: row.into_any_element(),
                offset: px(-scroll),
            });
        for left in [true, false] {
            if if left { left_edge } else { right_edge } {
                strip = strip.child(
                    Button::new(if left {
                        "scroll-tabs-left"
                    } else {
                        "scroll-tabs-right"
                    })
                    // Roving tabs provide complete keyboard access and reveal
                    // themselves. Pointer-only scroll controls must not create
                    // extra cycling tab stops inside the WASM iframe.
                    .tab_stop(false)
                    // Overlay controls must occlude the tab hitboxes beneath
                    // them; a transparent GPUI Button does not do so by default.
                    .block_mouse_except_scroll()
                    .accessibility_label(if left {
                        "Scroll tabs left"
                    } else {
                        "Scroll tabs right"
                    })
                    .absolute()
                    .top_0()
                    .when(left, |b| b.left_0())
                    .when(!left, |b| b.right_0())
                    .w(px(36.))
                    .h(px(height))
                    .cursor_pointer()
                    .hover(|s| s.opacity(0.7))
                    .focus_visible(|s| s.border_2().border_color(rgb(0x0285f7)))
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.scroll_by(if left { -0.8 } else { 0.8 } * this.viewport, false, cx);
                        cx.stop_propagation();
                    }))
                    .child(
                        canvas(
                            |_, _, _| (),
                            move |bounds, _, window, _| paint_chevron(bounds, left, dark, window),
                        )
                        .size_full(),
                    ),
                );
            }
        }
        div()
            .id("motion-tabs")
            .tab_group()
            .w(px(desired_width))
            .max_w_full()
            .flex()
            .flex_col()
            .child(strip)
            .when_some(panel, |root, text| {
                let mut color: Hsla = rgb(if dark { 0x868686 } else { 0x636363 }).into();
                color.a = panel_progress;
                let line = shape(text.clone(), color, FontWeight::NORMAL, window);
                root.child(
                    div()
                        .id("motion-tabs-panel")
                        .role(Role::TabPanel)
                        .aria_label(text)
                        .mt(px(16.))
                        .w_full()
                        .h(px(20.))
                        .child(
                            canvas(
                                |_, _, _| (),
                                move |bounds, _, window, cx| {
                                    let _ = line.paint(
                                        bounds.origin
                                            + point(px(0.), px(4. * (1. - panel_progress))),
                                        px(20.),
                                        TextAlign::Left,
                                        None,
                                        window,
                                        cx,
                                    );
                                },
                            )
                            .size_full(),
                        ),
                )
            })
    }
}

struct TabsDemo {
    tabs: Entity<MotionTabs>,
    dark: bool,
}
impl TabsDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let tabs = match variant {
            "overflow" => MotionTabs::new(
                [
                    "Overview",
                    "Activity",
                    "Analytics",
                    "Members",
                    "Billing",
                    "Settings",
                ]
                .map(|label| TabItem::new(label, label)),
            )
            .selected("Overview")
            .max_width(320.),
            "segment" => MotionTabs::new([
                TabItem::new("day", "Day"),
                TabItem::new("week", "Week"),
                TabItem::new("month", "Month"),
            ])
            .variant(TabsVariant::Segment)
            .selected("day"),
            "underline" => MotionTabs::new([
                TabItem::new("all", "All"),
                TabItem::new("open", "Open"),
                TabItem::new("closed", "Closed"),
            ])
            .variant(TabsVariant::Underline)
            .selected("all"),
            _ => MotionTabs::new([
                TabItem::new("overview", "Overview").content("High-level summary."),
                TabItem::new("activity", "Activity").content("Recent events."),
                TabItem::new("settings", "Settings").content("Preferences."),
            ])
            .selected("overview"),
        }
        .dark(dark);
        Self {
            tabs: cx.new(|_| tabs),
            dark,
        }
    }
}
impl Render for TabsDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("tabs-demo")
            .tab_group()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .px(px(24.))
            .bg(rgb(if self.dark { 0x202020 } else { 0xf8f8f8 }))
            .on_key_down(|event, window, cx| {
                let mut modifiers = event.keystroke.modifiers;
                modifiers.shift = false;
                if event.keystroke.key == "tab" && !modifiers.modified() {
                    let previous = window.focused(cx);
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    // A single roving tab stop must not trap browser focus
                    // inside the iframe. Only consume a successful traversal.
                    if previous != window.focused(cx) {
                        window.prevent_default();
                        cx.stop_propagation();
                    }
                }
            })
            .child(self.tabs.clone())
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
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(420.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| TabsDemo::new(&variant, dark, cx))
    })
    .expect("open tabs gallery");
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
    kit::application()
        .with_assets(crate::motion_button::ButtonAssets)
        .run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{Slot, Spring, navigate, reveal_offset};
    #[test]
    fn overdamped_step_is_finite_monotone_and_frame_rate_independent() {
        let mut spring = Spring::new(0.);
        spring.target = 100.;
        let mut previous = 0.;
        for _ in 0..120 {
            spring.advance(1. / 120.);
            assert!(spring.value.is_finite() && spring.value >= previous && spring.value <= 100.);
            previous = spring.value;
        }
        let mut one_step = Spring::new(0.);
        one_step.target = 100.;
        one_step.advance(1.);
        assert!((one_step.value - spring.value).abs() < 0.001);
        assert!((one_step.velocity - spring.velocity).abs() < 0.01);
    }
    #[test]
    fn interrupted_indicator_keeps_position_and_velocity() {
        let mut spring = Spring::new(4.);
        spring.target = 180.;
        spring.advance(0.08);
        let old = spring;
        spring.target = 60.;
        assert_eq!(spring.value, old.value);
        assert_eq!(spring.velocity, old.velocity);
        spring.advance(1. / 120.);
        assert!(spring.value.is_finite());
        spring.advance(4.);
        assert_eq!(spring.value, 60.);
        assert_eq!(spring.velocity, 0.);
    }
    #[test]
    fn keyboard_wraps_skips_disabled_and_handles_empty_lists() {
        let enabled = [true, false, true, false];
        assert_eq!(navigate(&enabled, 0, "right"), Some(2));
        assert_eq!(navigate(&enabled, 0, "left"), Some(2));
        assert_eq!(navigate(&enabled, 2, "right"), Some(0));
        assert_eq!(navigate(&enabled, 2, "home"), Some(0));
        assert_eq!(navigate(&enabled, 0, "end"), Some(2));
        assert_eq!(navigate(&[false, false], 0, "right"), None);
        assert_eq!(navigate(&[], 0, "home"), None);
    }
    #[test]
    fn overflow_reveal_respects_only_visible_overlay_arrows_and_clamps() {
        assert_eq!(
            reveal_offset(Slot { x: 4., width: 90. }, 0., 320., 525.),
            0.
        );
        assert_eq!(
            reveal_offset(
                Slot {
                    x: 285.,
                    width: 90.
                },
                0.,
                320.,
                525.
            ),
            91.
        );
        assert_eq!(
            reveal_offset(
                Slot {
                    x: 439.,
                    width: 82.
                },
                0.,
                320.,
                525.
            ),
            205.
        );
        assert_eq!(
            reveal_offset(Slot { x: 4., width: 90. }, 205., 320., 525.),
            0.
        );
        assert_eq!(
            reveal_offset(Slot { x: 4., width: 90. }, 50., 600., 525.),
            0.
        );
    }
}
