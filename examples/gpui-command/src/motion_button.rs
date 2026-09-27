//! Native GPUI adaptation of beUI's Button family (MIT).
//! Reference: https://beui.dev/components/motion/button
//! Animation values are the source's physics/timings, shared by desktop and WASM.
use gpui_kit::{self as kit, *};
use kit::base::Button;
use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
    time::Duration,
};

const FONT: &str = "Geist";
const FONT_BYTES: &[u8] = include_bytes!("../assets/Geist-Medium.ttf");

/// Supplies the bundled font to both GPUI's SVG text resolver and native text system.
pub struct ButtonAssets;
impl AssetSource for ButtonAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        // GPUI's SVG resolver probes these standard bundled-font asset keys.
        Ok(match path {
            "fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf" => Some(Cow::Borrowed(FONT_BYTES)),
            "fonts/lilex/Lilex-Regular.ttf" => Some(Cow::Borrowed(
                include_bytes!("../assets/IBMPlexSans-Regular.ttf").as_slice(),
            )),
            _ => None,
        })
    }
    fn list(&self, _: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    Base,
    Metallic,
    Magnetic,
    Stateful,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Outline,
    Ghost,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Md,
    Lg,
    Icon,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonState {
    Idle,
    Loading,
    Success,
    Error,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ButtonIcon {
    ArrowRight,
    ArrowUpRight,
    Sparkles,
    Download,
    Trash,
    Mail,
    Check,
    X,
    Loader,
}

#[derive(Clone, Copy)]
struct Physics {
    stiffness: f32,
    damping: f32,
    mass: f32,
}
const PRESS: Physics = Physics {
    stiffness: 500.,
    damping: 30.,
    mass: 0.6,
};
const SWAP: Physics = Physics {
    stiffness: 460.,
    damping: 30.,
    mass: 0.55,
};
const MOUSE: Physics = Physics {
    stiffness: 200.,
    damping: 15.,
    mass: 0.3,
};

/// Exact damped spring integration preserves velocity when the target changes.
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
    fn advance(&mut self, dt: f32, physics: Physics) {
        let dt = dt.max(0.);
        let decay = physics.damping / (2. * physics.mass);
        let frequency = (physics.stiffness / physics.mass - decay * decay).sqrt();
        let offset = self.value - self.target;
        let b = (self.velocity + decay * offset) / frequency;
        let (sin, cos) = (frequency * dt).sin_cos();
        let envelope = (-decay * dt).exp();
        self.value = self.target + envelope * (offset * cos + b * sin);
        self.velocity = envelope
            * ((b * frequency - decay * offset) * cos - (offset * frequency + decay * b) * sin);
        if (self.value - self.target).abs() < 0.0001 && self.velocity.abs() < 0.001 {
            self.value = self.target;
            self.velocity = 0.;
        }
    }
    fn snap(&mut self) {
        self.value = self.target;
        self.velocity = 0.;
    }
}
fn spring_progress(seconds: f32, physics: Physics) -> f32 {
    if seconds <= 0. {
        return 0.;
    }
    let mut spring = Spring::new(0.);
    spring.target = 1.;
    spring.advance(seconds, physics);
    spring.value
}
fn bezier(t: f32, controls: [f32; 4]) -> f32 {
    if t <= 0. {
        return 0.;
    }
    if t >= 1. {
        return 1.;
    }
    let [x1, y1, x2, y2] = controls;
    let sample = |s: f32, a: f32, b: f32| {
        3. * (1. - s).powi(2) * s * a + 3. * (1. - s) * s * s * b + s * s * s
    };
    let (mut lo, mut hi) = (0., 1.);
    for _ in 0..20 {
        let m = (lo + hi) * 0.5;
        if sample(m, x1, x2) < t {
            lo = m;
        } else {
            hi = m;
        }
    }
    sample((lo + hi) * 0.5, y1, y2)
}
const EASE_OUT: [f32; 4] = [0.16, 1., 0.3, 1.];
const EASE_IN_OUT: [f32; 4] = [0.77, 0., 0.175, 1.];

#[derive(Clone, Copy)]
struct Tween {
    from: f32,
    target: f32,
    started: f32,
}
impl Tween {
    fn new() -> Self {
        Self {
            from: 0.,
            target: 0.,
            started: 0.,
        }
    }
    fn value(&self, now: f32, duration: f32, ease: [f32; 4]) -> f32 {
        self.from + (self.target - self.from) * bezier((now - self.started) / duration, ease)
    }
    fn retarget(&mut self, target: f32, now: f32, duration: f32, ease: [f32; 4]) {
        self.from = self.value(now, duration, ease);
        self.target = target;
        self.started = now;
    }
}
#[derive(Clone)]
struct Ripple {
    at: f32,
    center: Point<Pixels>,
    size: f32,
}
type ClickHandler = Rc<dyn Fn(&mut MotionButton, &mut Window, &mut Context<MotionButton>)>;

/// Entity-backed reusable button. `on_click` works for pointer, Enter, and Space.
/// Stateful variants are controlled through `set_state`; async work belongs to the caller.
pub struct MotionButton {
    label: SharedString,
    loading_label: SharedString,
    success_label: SharedString,
    error_label: SharedString,
    kind: ButtonKind,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<ButtonIcon>,
    icon_before: bool,
    state: ButtonState,
    previous_state: Option<ButtonState>,
    transition_at: f32,
    incoming_active: bool,
    render_cache: Rc<RefCell<ButtonRenderCache>>,
    dark: bool,
    disabled: bool,
    ripple: bool,
    strength: f32,
    press_scale: f32,
    paused: bool,
    hovered: bool,
    pressed: bool,
    scale: Spring,
    x: Spring,
    y: Spring,
    width: Spring,
    hover: Tween,
    surface: Tween,
    clock: Option<Box<dyn Fn() -> f32>>,
    last_frame: f32,
    last_event: f32,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    ripples: Vec<Ripple>,
    on_click: Option<ClickHandler>,
    task: Option<Task<()>>,
}
impl MotionButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            loading_label: "Loading".into(),
            success_label: "Done".into(),
            error_label: "Try again".into(),
            kind: ButtonKind::Base,
            variant: ButtonVariant::Primary,
            size: ButtonSize::Md,
            icon: None,
            icon_before: false,
            state: ButtonState::Idle,
            previous_state: None,
            transition_at: 0.,
            incoming_active: false,
            render_cache: Rc::new(RefCell::new(ButtonRenderCache::default())),
            dark: false,
            disabled: false,
            ripple: false,
            strength: 0.25,
            press_scale: 0.93,
            paused: false,
            hovered: false,
            pressed: false,
            scale: Spring::new(1.),
            x: Spring::new(0.),
            y: Spring::new(0.),
            width: Spring::new(0.),
            hover: Tween::new(),
            surface: Tween::new(),
            clock: None,
            last_frame: 0.,
            last_event: 0.,
            bounds: Rc::new(Cell::new(Bounds::default())),
            ripples: vec![],
            on_click: None,
            task: None,
        }
    }
    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = kind;
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
    pub fn icon(mut self, icon: ButtonIcon) -> Self {
        self.icon = Some(icon);
        self
    }
    pub fn icon_before(mut self, before: bool) -> Self {
        self.icon_before = before;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn ripple(mut self, ripple: bool) -> Self {
        self.ripple = ripple;
        self
    }
    pub fn press_scale(mut self, scale: f32) -> Self {
        self.press_scale = scale;
        self
    }
    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = paused;
        self
    }
    pub fn strength(mut self, strength: f32) -> Self {
        self.strength = strength;
        self
    }
    pub fn feedback_labels(
        mut self,
        loading: impl Into<SharedString>,
        success: impl Into<SharedString>,
        error: impl Into<SharedString>,
    ) -> Self {
        self.loading_label = loading.into();
        self.success_label = success.into();
        self.error_label = error.into();
        self
    }
    pub fn on_click(
        mut self,
        callback: impl Fn(&mut Self, &mut Window, &mut Context<Self>) + 'static,
    ) -> Self {
        self.on_click = Some(Rc::new(callback));
        self
    }
    pub fn state(&self) -> ButtonState {
        self.state
    }
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        if disabled {
            self.pressed = false;
        }
        cx.notify();
    }
    pub fn set_state(&mut self, state: ButtonState, cx: &mut Context<Self>) {
        if self.state == state {
            return;
        }
        self.advance_to_now();
        self.previous_state = Some(self.state);
        self.incoming_active = true;
        self.state = state;
        self.transition_at = self.now();
        self.last_event = self.now();
        self.pressed = false;
        cx.notify();
    }
    fn advance_to_now(&mut self) {
        let now = self.now();
        let dt = (now - self.last_frame).max(0.);
        self.scale.advance(dt, PRESS);
        self.x.advance(dt, MOUSE);
        self.y.advance(dt, MOUSE);
        self.width.advance(dt, SWAP);
        self.last_frame = now;
    }
    fn now(&self) -> f32 {
        self.clock.as_ref().map_or(0., |clock| clock())
    }
    fn label_for(&self, state: ButtonState) -> SharedString {
        match state {
            ButtonState::Idle => self.label.clone(),
            ButtonState::Loading => self.loading_label.clone(),
            ButtonState::Success => self.success_label.clone(),
            ButtonState::Error => self.error_label.clone(),
        }
    }
    fn current_icon(&self, state: ButtonState) -> Option<ButtonIcon> {
        match state {
            ButtonState::Idle => self.icon,
            ButtonState::Loading => Some(ButtonIcon::Loader),
            ButtonState::Success => Some(ButtonIcon::Check),
            ButtonState::Error => Some(ButtonIcon::X),
        }
    }
    fn is_disabled(&self) -> bool {
        self.disabled || self.state == ButtonState::Loading
    }
    fn pointer_press(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        if self.is_disabled() {
            return;
        }
        self.advance_to_now();
        self.pressed = true;
        self.last_event = self.now();
        if self.ripple && !cx.reduce_motion() {
            let bounds = self.bounds.get();
            self.ripples.push(Ripple {
                at: self.now(),
                center: event.position - bounds.origin,
                size: f32::from(bounds.size.width).max(f32::from(bounds.size.height)) * 2.,
            });
        }
        cx.notify();
    }
    fn set_hovered(&mut self, hovered: bool, cx: &mut Context<Self>) {
        if self.hovered == hovered {
            return;
        }
        self.advance_to_now();
        self.hovered = hovered;
        let now = self.now();
        self.last_event = now;
        self.hover
            .retarget(if hovered { 1. } else { 0. }, now, 2.4, EASE_IN_OUT);
        self.surface
            .retarget(if hovered { 1. } else { 0. }, now, 0.15, [0.4, 0., 0.2, 1.]);
        if !hovered {
            self.pressed = false;
            self.x.target = 0.;
            self.y.target = 0.;
        }
        cx.notify();
    }
    fn release(&mut self, cx: &mut Context<Self>) {
        if self.pressed {
            self.advance_to_now();
            self.pressed = false;
            self.last_event = self.now();
            cx.notify();
        }
    }
    fn demo_action(&mut self, success: bool, cx: &mut Context<Self>) {
        if self.is_disabled() {
            return;
        }
        self.task = None;
        self.set_state(ButtonState::Loading, cx);
        self.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(1400))
                .await;
            let _ = this.update(cx, |this, cx| {
                this.set_state(
                    if success {
                        ButtonState::Success
                    } else {
                        ButtonState::Error
                    },
                    cx,
                )
            });
            cx.background_executor()
                .timer(Duration::from_millis(1800))
                .await;
            let _ = this.update(cx, |this, cx| this.set_state(ButtonState::Idle, cx));
        }));
    }
}

#[derive(Clone)]
struct Label {
    letters: Rc<[ShapedLine]>,
    width: f32,
}
fn shape_label(
    text: SharedString,
    font_size: f32,
    color: Hsla,
    cascade: bool,
    window: &mut Window,
) -> Label {
    let pieces: Vec<SharedString> = if cascade {
        text.chars().map(|c| c.to_string().into()).collect()
    } else {
        vec![text.clone()]
    };
    let letters: Vec<_> = pieces
        .into_iter()
        .map(|piece| {
            window.text_system().shape_line(
                piece.clone(),
                px(font_size),
                &[TextRun {
                    len: piece.len(),
                    font: Font {
                        family: FONT.into(),
                        weight: FontWeight::MEDIUM,
                        ..Default::default()
                    },
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
        })
        .collect();
    Label {
        width: letters.iter().map(|l| f32::from(l.width())).sum(),
        letters: letters.into(),
    }
}
fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
const BLUR_LEVELS: usize = 13;
fn blur_level(blur: f32) -> u8 {
    (blur.clamp(0., 6.) * 2.).round() as u8
}
fn blur_sigma(level: u8) -> f32 {
    level.min((BLUR_LEVELS - 1) as u8) as f32 * 0.5
}

#[derive(Clone, PartialEq)]
struct LabelGeometry {
    text: [SharedString; 4],
    font_size: f32,
    line_height: f32,
    idle_icon: Option<ButtonIcon>,
}
#[derive(Clone, Hash, PartialEq, Eq)]
enum SvgKey {
    Glyph {
        text: SharedString,
        width: u32,
        ascent: u32,
        descent: u32,
        font_size: u32,
        line_height: u32,
        level: u8,
    },
    Icon {
        icon: ButtonIcon,
        level: u8,
    },
}
struct SvgSpec {
    data: SharedString,
    size: Size<Pixels>,
    level: u8,
    ready_scale: Cell<Option<f32>>,
    urgent: Cell<bool>,
}
impl SvgSpec {
    fn is_ready(&self, scale_factor: f32) -> bool {
        self.ready_scale.get() == Some(scale_factor)
    }
}
fn nearby_levels(level: u8) -> impl Iterator<Item = u8> {
    (0..BLUR_LEVELS as u8).flat_map(move |distance| {
        [
            level.checked_sub(distance),
            (distance > 0)
                .then(|| level + distance)
                .filter(|&next| next < BLUR_LEVELS as u8),
        ]
        .into_iter()
        .flatten()
    })
}
#[derive(Default)]
struct ButtonRenderCache {
    geometry: Option<LabelGeometry>,
    color: Option<Hsla>,
    labels: Vec<Label>,
    specs: HashMap<SvgKey, Rc<SvgSpec>>,
    warm_specs: Vec<Rc<SvgSpec>>,
    warm_queue: VecDeque<Rc<SvgSpec>>,
    urgent_queue: VecDeque<Rc<SvgSpec>>,
    zero_scale: Option<f32>,
    scale_factor: Option<f32>,
}
impl ButtonRenderCache {
    fn glyph(
        &mut self,
        line: &ShapedLine,
        font_size: f32,
        line_height: f32,
        level: u8,
    ) -> Rc<SvgSpec> {
        let width = f32::from(line.width());
        let ascent = f32::from(line.ascent);
        let descent = f32::from(line.descent);
        let key = SvgKey::Glyph {
            text: line.text.clone(),
            width: width.to_bits(),
            ascent: ascent.to_bits(),
            descent: descent.to_bits(),
            font_size: font_size.to_bits(),
            line_height: line_height.to_bits(),
            level,
        };
        self.specs.entry(key).or_insert_with(|| {
            let blur = blur_sigma(level);
            let padding = 20.;
            let w = width + padding * 2.;
            let h = line_height + padding * 2.;
            let baseline = padding + (line_height - ascent - descent) * 0.5 + ascent;
            let data = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}" height="{h}" viewBox="0 0 {w} {h}"><defs><filter id="b" filterUnits="userSpaceOnUse" x="0" y="0" width="{w}" height="{h}"><feGaussianBlur stdDeviation="{blur}"/></filter></defs><text x="{padding}" y="{baseline}" font-family="Geist" font-weight="500" font-size="{font_size}" fill="white" filter="url(#b)">{}</text></svg>"#, xml_escape(&line.text));
            Rc::new(SvgSpec { data: data.into(), size: size(px(w), px(h)), level, ready_scale: Cell::new(None), urgent: Cell::new(false) })
        }).clone()
    }
    fn icon(&mut self, icon: ButtonIcon, level: u8) -> Rc<SvgSpec> {
        self.specs.entry(SvgKey::Icon { icon, level }).or_insert_with(|| {
            let blur = blur_sigma(level);
            let data = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64"><defs><filter id="b" filterUnits="userSpaceOnUse" x="0" y="0" width="64" height="64"><feGaussianBlur stdDeviation="{blur}"/></filter></defs><g filter="url(#b)"><path transform="translate(20 20)" d="{}" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/></g></svg>"#, icon_path(icon));
            Rc::new(SvgSpec { data: data.into(), size: size(px(64.), px(64.)), level, ready_scale: Cell::new(None), urgent: Cell::new(false) })
        }).clone()
    }
    fn request_cold(&mut self, spec: &Rc<SvgSpec>, scale_factor: f32) {
        if !spec.is_ready(scale_factor) && !spec.urgent.replace(true) {
            self.urgent_queue.push_back(spec.clone());
        }
    }
    fn ready_glyph(
        &mut self,
        line: &ShapedLine,
        font_size: f32,
        line_height: f32,
        level: u8,
        scale_factor: f32,
    ) -> Option<Rc<SvgSpec>> {
        let desired = self.glyph(line, font_size, line_height, level);
        self.request_cold(&desired, scale_factor);
        nearby_levels(level).find_map(|level| {
            let spec = self.glyph(line, font_size, line_height, level);
            spec.is_ready(scale_factor).then_some(spec)
        })
    }
    fn ready_icon(
        &mut self,
        icon: ButtonIcon,
        level: u8,
        scale_factor: f32,
    ) -> Option<Rc<SvgSpec>> {
        let desired = self.icon(icon, level);
        self.request_cold(&desired, scale_factor);
        nearby_levels(level).find_map(|level| {
            let spec = self.icon(icon, level);
            spec.is_ready(scale_factor).then_some(spec)
        })
    }
    fn has_pending(&self) -> bool {
        !self.urgent_queue.is_empty() || !self.warm_queue.is_empty()
    }
    fn bootstrap_zero(&mut self, window: &mut Window, cx: &App) {
        let scale_factor = window.scale_factor();
        if self.zero_scale == Some(scale_factor) {
            return;
        }
        let origin = point(px(0.), px(0.));
        let mut succeeded = true;
        // These unfiltered masks provide a safe fallback before the component
        // first becomes visible. Gaussian masks never block the visible pass.
        for spec in self.warm_specs.iter().filter(|spec| spec.level == 0) {
            if !spec.is_ready(scale_factor) {
                succeeded &= svg_paint(
                    spec,
                    Bounds::new(origin, spec.size),
                    origin,
                    1.,
                    0.,
                    transparent_black(),
                    window,
                    cx,
                );
            }
        }
        if succeeded {
            self.zero_scale = Some(scale_factor);
        }
    }
    fn prepare(&mut self, geometry: LabelGeometry, color: Hsla, window: &mut Window) {
        let geometry_changed = self.geometry.as_ref() != Some(&geometry);
        if geometry_changed || self.color != Some(color) {
            self.labels = geometry
                .text
                .iter()
                .map(|text| shape_label(text.clone(), geometry.font_size, color, true, window))
                .collect();
            self.color = Some(color);
        }
        if geometry_changed {
            self.specs.clear();
            self.warm_specs.clear();
            let mut seen = HashSet::new();
            // Build specifications once; expensive SVG rasterization is deferred
            // to a small idle-frame budget, never performed in click handlers.
            let labels = self.labels.clone();
            // Low-sigma masks are most visible. Populate them across all labels
            // first so even an immediate first interaction gets useful hits.
            for level in 0..BLUR_LEVELS as u8 {
                for label in &labels {
                    for letter in label
                        .letters
                        .iter()
                        .filter(|letter| !letter.text.trim().is_empty())
                    {
                        let spec =
                            self.glyph(letter, geometry.font_size, geometry.line_height, level);
                        if seen.insert(spec.data.clone()) {
                            self.warm_specs.push(spec);
                        }
                    }
                }
                for icon in geometry.idle_icon.into_iter().chain([
                    ButtonIcon::Loader,
                    ButtonIcon::Check,
                    ButtonIcon::X,
                ]) {
                    let spec = self.icon(icon, level);
                    if seen.insert(spec.data.clone()) {
                        self.warm_specs.push(spec);
                    }
                }
            }
            self.geometry = Some(geometry);
        }
        let scale_factor = window.scale_factor();
        if geometry_changed || self.scale_factor != Some(scale_factor) {
            self.warm_queue = self.warm_specs.iter().cloned().collect();
            self.urgent_queue.clear();
            self.zero_scale = None;
            for spec in &self.warm_specs {
                spec.urgent.set(false);
            }
            self.scale_factor = Some(scale_factor);
        }
    }
    fn warm_frame(&mut self, window: &mut Window, cx: &App) {
        let executor = cx.background_executor();
        let started = executor.now();
        while let Some(spec) = self
            .urgent_queue
            .pop_front()
            .or_else(|| self.warm_queue.pop_front())
        {
            spec.urgent.set(false);
            if !spec.is_ready(window.scale_factor()) {
                let origin = point(px(0.), px(0.));
                svg_paint(
                    &spec,
                    Bounds::new(origin, spec.size),
                    origin,
                    1.,
                    0.,
                    transparent_black(),
                    window,
                    cx,
                );
            }
            // A single rasterization is indivisible. Do not begin another once
            // the budget is spent, even while an animation is in progress.
            if executor.now().duration_since(started) >= Duration::from_millis(1) {
                break;
            }
        }
    }
}

fn svg_paint(
    spec: &SvgSpec,
    bounds: Bounds<Pixels>,
    pivot: Point<Pixels>,
    drawing_scale: f32,
    rotation: f32,
    color: Hsla,
    window: &mut Window,
    cx: &App,
) -> bool {
    // Keep raster size/cache keys stable; all fractional movement and scaling
    // happens in the GPU sprite matrix after GPUI snaps its atlas coordinates.
    let scale = window.scale_factor();
    let snapped = point(
        px((f32::from(bounds.origin.x) * scale).round() / scale),
        px((f32::from(bounds.origin.y) * scale).round() / scale),
    );
    let correction = bounds.origin - snapped;
    // Stabilize atlas dimensions independently of the translated origin.
    let raster_size = spec
        .size
        .map(|dimension| px((f32::from(dimension) * scale).round().max(1.) / scale));
    let transform = TransformationMatrix::unit()
        .translate(pivot.scale(scale))
        .rotate(radians(rotation))
        .scale(size(drawing_scale, drawing_scale))
        .translate(pivot.scale(-scale))
        .translate(correction.scale(scale));
    let rendered = window.paint_svg(
        Bounds::new(snapped, raster_size),
        spec.data.clone(),
        Some(spec.data.as_bytes()),
        transform,
        color,
        cx,
    );
    let succeeded = rendered.is_ok();
    spec.ready_scale
        .set(succeeded.then_some(window.scale_factor()));
    succeeded
}
fn paint_letter(
    line: &ShapedLine,
    origin: Point<Pixels>,
    font_size: f32,
    line_height: f32,
    scale: f32,
    opacity: f32,
    blur: f32,
    color: Hsla,
    window: &mut Window,
    cx: &mut App,
    cache: &RefCell<ButtonRenderCache>,
) {
    if opacity <= 0.001 || line.text.trim().is_empty() {
        return;
    }
    if blur < 0.03 && opacity > 0.999 && scale == 1. {
        let _ = line.paint(
            origin,
            px(line_height * scale),
            TextAlign::Left,
            None,
            window,
            cx,
        );
        return;
    }
    // A small, cached bank of real Gaussian masks keeps first-use rendering
    // bounded. Position, opacity and scale remain continuous GPU properties.
    let spec = {
        let mut cache = cache.borrow_mut();
        if cache.geometry.is_some() {
            cache.ready_glyph(
                line,
                font_size,
                line_height,
                blur_level(blur),
                window.scale_factor(),
            )
        } else {
            Some(cache.glyph(line, font_size, line_height, blur_level(blur)))
        }
    };
    let Some(spec) = spec else {
        return;
    };
    let padding = 20.;
    svg_paint(
        &spec,
        Bounds::new(origin - point(px(padding), px(padding)), spec.size),
        origin,
        scale,
        0.,
        color.opacity(opacity.clamp(0., 1.)),
        window,
        cx,
    );
}
fn icon_path(icon: ButtonIcon) -> &'static str {
    match icon {
        ButtonIcon::ArrowRight => "M5 12h14m-7-7 7 7-7 7",
        ButtonIcon::ArrowUpRight => "M7 17 17 7M7 7h10v10",
        ButtonIcon::Sparkles => {
            "m12 3 1.9 5.8L20 11l-6.1 2.2L12 19l-1.9-5.8L4 11l6.1-2.2L12 3ZM20 3v4m-2-2h4M4 16v4m-2-2h4"
        }
        ButtonIcon::Download => "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3",
        ButtonIcon::Trash => "M3 6h18M19 6l-1 14H6L5 6M9 6V3h6v3M10 10v6M14 10v6",
        ButtonIcon::Mail => {
            "M4 4h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2ZM22 6l-10 7L2 6"
        }
        ButtonIcon::Check => "M20 6 9 17l-5-5",
        ButtonIcon::X => "M18 6 6 18M6 6l12 12",
        ButtonIcon::Loader => "M21 12a9 9 0 1 1-6.22-8.56",
    }
}
fn paint_icon(
    icon: ButtonIcon,
    center: Point<Pixels>,
    diameter: f32,
    scale: f32,
    opacity: f32,
    blur: f32,
    now: f32,
    color: Hsla,
    window: &mut Window,
    cx: &App,
    cache: &RefCell<ButtonRenderCache>,
) {
    if opacity <= 0.001 || scale <= 0. {
        return;
    }
    let rotation = if icon == ButtonIcon::Loader {
        now * std::f32::consts::TAU
    } else {
        0.
    };
    let spec = {
        let mut cache = cache.borrow_mut();
        if cache.geometry.is_some() {
            cache.ready_icon(icon, blur_level(blur), window.scale_factor())
        } else {
            Some(cache.icon(icon, blur_level(blur)))
        }
    };
    let Some(spec) = spec else {
        return;
    };
    svg_paint(
        &spec,
        Bounds::new(center - point(px(32.), px(32.)), spec.size),
        center,
        diameter / 24. * scale,
        rotation,
        color.opacity(opacity.clamp(0., 1.)),
        window,
        cx,
    );
}

fn paint_ripple(
    bounds: Bounds<Pixels>,
    center: Point<Pixels>,
    radius: f32,
    corner: f32,
    color: Hsla,
    window: &mut Window,
) {
    if radius <= 0.001 {
        return;
    }
    // Draw the analytic intersection of a circle and rounded rectangle as a
    // native path; no frame-dependent image cache or rectangular corner leak.
    let left = f32::from(bounds.left());
    let right = f32::from(bounds.right());
    let top = f32::from(bounds.top());
    let bottom = f32::from(bounds.bottom());
    let cy = f32::from(center.y);
    let cx = f32::from(center.x);
    let start = top.max(cy - radius);
    let end = bottom.min(cy + radius);
    if end <= start {
        return;
    }
    let edges = |y: f32| {
        let circle = (radius * radius - (y - cy).powi(2)).max(0.).sqrt();
        let dy = if y < top + corner {
            top + corner - y
        } else if y > bottom - corner {
            y - (bottom - corner)
        } else {
            0.
        };
        let inset = corner - (corner * corner - dy * dy).max(0.).sqrt();
        (
            (cx - circle).max(left + inset),
            (cx + circle).min(right - inset),
        )
    };
    let mut path = PathBuilder::fill();
    let mut started = false;
    let mut right_points = vec![];
    for i in 0..=64 {
        let y = start + (end - start) * i as f32 / 64.;
        let (l, r) = edges(y);
        if r < l {
            continue;
        }
        if !started {
            path.move_to(point(px(l), px(y)));
            started = true;
        } else {
            path.line_to(point(px(l), px(y)));
        }
        right_points.push(point(px(r), px(y)));
    }
    if started {
        for p in right_points.into_iter().rev() {
            path.line_to(p);
        }
        path.close();
        if let Ok(path) = path.build() {
            window.paint_path(path, color);
        }
    }
}

/// Moves the entire interactive subtree while preserving its layout footprint.
/// GPUI snaps element offsets for hit testing; the canvas receives only the
/// remaining fraction so its drawing still follows the spring continuously.
struct TranslatedButton {
    child: AnyElement,
    offset: Point<Pixels>,
    remainder: Rc<Cell<Point<Pixels>>>,
}
impl IntoElement for TranslatedButton {
    type Element = Self;
    fn into_element(self) -> Self {
        self
    }
}
impl Element for TranslatedButton {
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
        let parent_offset = window.element_offset();
        let snapped_delta = window.pixel_snap_point(parent_offset + self.offset)
            - window.pixel_snap_point(parent_offset);
        self.remainder.set(self.offset - snapped_delta);
        window.with_element_offset(self.offset, |window| {
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
        // AnyElement keeps its translated prepaint bounds, including hitboxes.
        self.child.paint(window, cx);
    }
}

impl Render for MotionButton {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.clock.is_none() {
            let executor = cx.background_executor().clone();
            let start = executor.now();
            self.clock = Some(Box::new(move || {
                executor.now().duration_since(start).as_secs_f32()
            }));
        }
        let now = self.now();
        let dt = (now - self.last_frame).max(0.);
        self.last_frame = now;
        let reduce = cx.reduce_motion();
        let disabled = self.is_disabled();
        self.scale.target = if reduce || disabled {
            1.
        } else if self.pressed {
            self.press_scale
        } else if self.hovered && self.kind != ButtonKind::Stateful {
            1.02
        } else {
            1.
        };
        if reduce {
            self.scale.snap();
            self.x.target = 0.;
            self.y.target = 0.;
            self.x.snap();
            self.y.snap();
        } else {
            self.scale.advance(dt, PRESS);
            self.x.advance(dt, MOUSE);
            self.y.advance(dt, MOUSE);
        }
        let (height, font_size, line_height, padding, gap) = match self.size {
            ButtonSize::Sm => (32., 12., 16., 12., 6.),
            ButtonSize::Md => (40., 14., 20., 20., 8.),
            ButtonSize::Lg => (48., 16., 24., 24., 8.),
            ButtonSize::Icon => (32., 14., 20., 0., 0.),
        };
        let foreground = if self.dark {
            rgb(0xf2f2f2)
        } else {
            rgb(0x0b0b0b)
        };
        let surface = if reduce {
            if self.hovered { 1. } else { 0. }
        } else {
            self.surface.value(now, 0.15, [0.4, 0., 0.2, 1.])
        };
        let color = if self.kind == ButtonKind::Metallic {
            foreground.into()
        } else {
            match self.variant {
                ButtonVariant::Primary => rgb(0xffffff).into(),
                ButtonVariant::Ghost => mix(
                    if self.dark {
                        rgb(0x868686).into()
                    } else {
                        rgb(0x636363).into()
                    },
                    foreground.into(),
                    surface,
                ),
                _ => foreground.into(),
            }
        };
        let stateful = self.kind == ButtonKind::Stateful;
        let age = now - self.transition_at;
        let cache = self.render_cache.clone();
        if stateful {
            cache.borrow_mut().prepare(
                LabelGeometry {
                    text: [
                        self.label.clone(),
                        self.loading_label.clone(),
                        self.success_label.clone(),
                        self.error_label.clone(),
                    ],
                    font_size,
                    line_height,
                    idle_icon: self.icon,
                },
                color,
                window,
            );
        }
        let current = if stateful {
            cache.borrow().labels[self.state as usize].clone()
        } else {
            shape_label(self.label_for(self.state), font_size, color, false, window)
        };
        if let Some(previous_state) = self.previous_state {
            let letter_count = if stateful {
                cache.borrow().labels[previous_state as usize].letters.len()
            } else {
                0
            };
            let exit_duration = 0.16 + letter_count.saturating_sub(1) as f32 * 0.0125;
            if reduce || age >= exit_duration {
                self.previous_state = None;
            }
        }
        let last_delay = current.letters.len().saturating_sub(1) as f32 * 0.025;
        if reduce || spring_progress(age - last_delay, SWAP) == 1. {
            self.incoming_active = false;
        }
        let previous = self.previous_state.map(|state| {
            (
                state,
                if stateful {
                    cache.borrow().labels[state as usize].clone()
                } else {
                    shape_label(self.label_for(state), font_size, color, false, window)
                },
            )
        });
        let animated = stateful && (self.incoming_active || previous.is_some()) && !reduce;
        let warm_enabled = stateful && !reduce;
        let warm_pending = warm_enabled && cache.borrow().has_pending();
        let slot_progress = if animated {
            spring_progress(age, SWAP)
        } else {
            1.
        };
        let incoming_icon = self.current_icon(self.state);
        let icon_space = if stateful {
            if incoming_icon.is_some() {
                24. * slot_progress
            } else {
                0.
            }
        } else if self.icon.is_some() && self.size != ButtonSize::Icon {
            if self.size == ButtonSize::Sm {
                14. + gap
            } else {
                16. + gap
            }
        } else {
            0.
        };
        let outgoing_icon = previous
            .as_ref()
            .and_then(|(state, _)| self.current_icon(*state));
        let exit_progress = if animated {
            bezier(age / 0.16, EASE_OUT)
        } else {
            1.
        };
        let outgoing_space = if animated && outgoing_icon.is_some() {
            24. * (1. - exit_progress)
        } else {
            0.
        };
        let current_text_width = current.width;
        let target_width = if stateful {
            current_text_width
        } else if self.size == ButtonSize::Icon {
            32.
        } else {
            padding * 2. + current_text_width + icon_space
        };
        if self.width.value == 0. {
            self.width = Spring::new(target_width);
        }
        self.width.target = target_width;
        if reduce || !stateful {
            self.width.snap();
        } else {
            self.width.advance(dt, SWAP);
        }
        let text_slot_width = self.width.value;
        let width = if stateful {
            padding * 2. + text_slot_width + icon_space + outgoing_space
        } else {
            self.width.value
        };
        self.ripples.retain(|r| now - r.at < 1.6);
        if !reduce
            && ((self.kind == ButtonKind::Metallic && !self.paused)
                || self.state == ButtonState::Loading
                || now - self.last_event < 2.5
                || !self.ripples.is_empty()
                || warm_pending)
        {
            let weak = cx.entity().downgrade();
            window.on_next_frame(move |_, cx| {
                let _ = weak.update(cx, |_, cx| cx.notify());
            });
        }
        let bounds_capture = self.bounds.clone();
        let bounds_capture_paint = self.bounds.clone();
        let scale = self.scale.value;
        let offset = point(px(self.x.value), px(self.y.value));
        let remainder = Rc::new(Cell::new(point(px(0.), px(0.))));
        let paint_remainder = remainder.clone();
        let kind = self.kind;
        let variant = self.variant;
        let size_kind = self.size;
        let dark = self.dark;
        let hover = if reduce || self.paused {
            0.
        } else {
            self.hover.value(now, 2.4, EASE_IN_OUT)
        };
        let elapsed = if reduce || self.paused { 0. } else { now };
        let icon_before = self.icon_before;
        let ripples = self.ripples.clone();
        let current_state = self.state;
        let canvas = canvas(
            move |bounds, _, _| {
                bounds_capture.set(bounds);
            },
            move |layout, (), window, cx| {
                if stateful {
                    cache.borrow_mut().bootstrap_zero(window, cx);
                }
                let center = layout.center() + paint_remainder.get();
                let bounds =
                    Bounds::centered_at(center, size(px(width * scale), px(height * scale)));
                bounds_capture_paint.set(bounds);
                if kind == ButtonKind::Metallic {
                    crate::button_metallic::paint_metallic(
                        bounds, elapsed, hover, surface, dark, window, cx,
                    );
                } else {
                    let muted: Hsla = if dark { rgb(0x1c1c1c) } else { rgb(0xf5f5f5) }.into();
                    let bg = match variant {
                        ButtonVariant::Primary => {
                            Hsla::from(rgb(0x0285f7)).opacity(1. - 0.1 * surface)
                        }
                        ButtonVariant::Secondary => muted,
                        ButtonVariant::Ghost | ButtonVariant::Outline => {
                            muted.opacity(0.6 * surface)
                        }
                    };
                    let border =
                        if matches!(variant, ButtonVariant::Secondary | ButtonVariant::Outline) {
                            if dark {
                                Hsla::from(rgb(0xffffff)).opacity(0.05)
                            } else {
                                Hsla::from(rgb(0x0b0b0b)).opacity(0.06)
                            }
                        } else {
                            transparent_black()
                        };
                    window.paint_quad(quad(
                        bounds,
                        px(if size_kind == ButtonSize::Icon {
                            8. * scale
                        } else {
                            height * scale / 2.
                        }),
                        bg,
                        px(1.),
                        border,
                        BorderStyle::Solid,
                    ));
                }
                for ripple in &ripples {
                    let progress = bezier((now - ripple.at) / 1.6, EASE_OUT);
                    let radius = ripple.size * (0.05 + 0.95 * progress) * 0.5 * scale;
                    let origin =
                        bounds.origin + point(ripple.center.x * scale, ripple.center.y * scale);
                    paint_ripple(
                        bounds,
                        origin,
                        radius,
                        height * scale / 2.,
                        color.opacity(0.3 * (1. - progress)),
                        window,
                    );
                }
                let total_width = if stateful {
                    (width - padding * 2.) * scale
                } else {
                    (current.width + icon_space) * scale
                };
                let content_x = f32::from(center.x) - total_width / 2.;
                let baseline_y = f32::from(center.y) - line_height * scale / 2.;
                let clip = ContentMask {
                    bounds: Bounds::new(
                        point(
                            px(content_x),
                            px(f32::from(center.y) - line_height * scale / 2.),
                        ),
                        size(px(total_width.max(0.)), px(line_height * scale)),
                    ),
                };
                if stateful {
                    window.with_content_mask(Some(clip), |window| {
                        let leading = if current_state != ButtonState::Idle {
                            icon_space * scale
                        } else {
                            0.
                        };
                        let old_leading = if previous
                            .as_ref()
                            .is_some_and(|(state, _)| *state != ButtonState::Idle)
                        {
                            outgoing_space * scale
                        } else {
                            0.
                        };
                        let text_x = content_x + leading + old_leading;
                        window.with_content_mask(
                            Some(ContentMask {
                                bounds: Bounds::new(
                                    point(px(text_x), px(baseline_y)),
                                    size(px(text_slot_width * scale), px(line_height * scale)),
                                ),
                            }),
                            |window| {
                                if animated {
                                    if let Some((_, old)) = &previous {
                                        paint_cascade(
                                            old,
                                            point(px(text_x), px(baseline_y)),
                                            font_size,
                                            line_height,
                                            scale,
                                            age,
                                            false,
                                            color,
                                            window,
                                            cx,
                                            &cache,
                                        );
                                    }
                                }
                                if animated {
                                    paint_cascade(
                                        &current,
                                        point(px(text_x), px(baseline_y)),
                                        font_size,
                                        line_height,
                                        scale,
                                        age,
                                        true,
                                        color,
                                        window,
                                        cx,
                                        &cache,
                                    );
                                } else {
                                    paint_label(
                                        &current,
                                        point(px(text_x), px(baseline_y)),
                                        font_size,
                                        line_height,
                                        scale,
                                        color,
                                        window,
                                        cx,
                                        &cache,
                                    );
                                }
                            },
                        );
                        if let Some(icon) = incoming_icon {
                            let x = if current_state == ButtonState::Idle {
                                content_x + total_width - icon_space * scale / 2.
                            } else {
                                content_x + icon_space * scale / 2.
                            };
                            window.with_content_mask(
                                Some(ContentMask {
                                    bounds: Bounds::new(
                                        point(px(x - icon_space * scale / 2.), px(baseline_y)),
                                        size(px(icon_space * scale), px(line_height * scale)),
                                    ),
                                }),
                                |window| {
                                    paint_icon(
                                        icon,
                                        point(px(x), center.y),
                                        16. * scale,
                                        0.7 + 0.3 * slot_progress,
                                        slot_progress,
                                        6. * (1. - slot_progress).max(0.),
                                        elapsed,
                                        color,
                                        window,
                                        cx,
                                        &cache,
                                    );
                                },
                            );
                        }
                        if animated {
                            if let Some(icon) = outgoing_icon {
                                let x = if previous
                                    .as_ref()
                                    .is_some_and(|(state, _)| *state == ButtonState::Idle)
                                {
                                    content_x + total_width - outgoing_space * scale / 2.
                                } else {
                                    content_x + leading + outgoing_space * scale / 2.
                                };
                                window.with_content_mask(
                                    Some(ContentMask {
                                        bounds: Bounds::new(
                                            point(
                                                px(x - outgoing_space * scale / 2.),
                                                px(baseline_y),
                                            ),
                                            size(
                                                px(outgoing_space * scale),
                                                px(line_height * scale),
                                            ),
                                        ),
                                    }),
                                    |window| {
                                        paint_icon(
                                            icon,
                                            point(px(x), center.y),
                                            16. * scale,
                                            1. - 0.3 * exit_progress,
                                            1. - exit_progress,
                                            6. * exit_progress,
                                            elapsed,
                                            color,
                                            window,
                                            cx,
                                            &cache,
                                        );
                                    },
                                );
                            }
                        }
                    });
                } else if size_kind == ButtonSize::Icon {
                    if let Some(icon) = incoming_icon {
                        paint_icon(
                            icon,
                            center,
                            16. * scale,
                            1.,
                            1.,
                            0.,
                            elapsed,
                            color,
                            window,
                            cx,
                            &cache,
                        );
                    }
                } else {
                    let text_x = content_x + if icon_before { icon_space * scale } else { 0. };
                    paint_label(
                        &current,
                        point(px(text_x), px(baseline_y)),
                        font_size,
                        line_height,
                        scale,
                        color,
                        window,
                        cx,
                        &cache,
                    );
                    if let Some(icon) = incoming_icon {
                        let d = if size_kind == ButtonSize::Sm {
                            14.
                        } else {
                            16.
                        };
                        let x = if icon_before {
                            content_x + d * scale / 2.
                        } else {
                            content_x + current.width * scale + gap * scale + d * scale / 2.
                        };
                        paint_icon(
                            icon,
                            point(px(x), center.y),
                            d * scale,
                            1.,
                            1.,
                            0.,
                            elapsed,
                            color,
                            window,
                            cx,
                            &cache,
                        );
                    }
                }
                if warm_enabled {
                    cache.borrow_mut().warm_frame(window, cx);
                }
            },
        )
        .size_full();
        let button = Button::new("motion-button")
            .accessibility_label(if size_kind == ButtonSize::Icon {
                self.label.clone()
            } else {
                self.label_for(self.state)
            })
            .disabled(disabled)
            .w(px(width))
            .h(px(height))
            .rounded_full()
            .cursor_pointer()
            .opacity(if disabled { 0.5 } else { 1. })
            .focus_visible(move |style| {
                // An outer focus ring does not shrink/reflow the canvas, and
                // cannot be covered by the button's painted background.
                style.shadow(vec![
                    BoxShadow::new(px(0.), px(0.), rgb(0x0285f7).into()).spread_radius(px(4.)),
                    BoxShadow::new(
                        px(0.),
                        px(0.),
                        if dark { rgb(0x202020) } else { rgb(0xf8f8f8) }.into(),
                    )
                    .spread_radius(px(2.)),
                ])
            })
            .on_hover(cx.listener(|this, hovered, _, cx| this.set_hovered(*hovered, cx)))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event, _, cx| this.pointer_press(event, cx)),
            )
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.release(cx)),
            )
            .on_mouse_up_out(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| this.release(cx)),
            )
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if this.kind == ButtonKind::Magnetic && !this.is_disabled() && !cx.reduce_motion() {
                    this.advance_to_now();
                    let delta = event.position - this.bounds.get().center();
                    this.x.target = f32::from(delta.x) * this.strength;
                    this.y.target = f32::from(delta.y) * this.strength;
                    this.last_event = this.now();
                    cx.notify();
                }
            }))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if !this.is_disabled() && matches!(event.keystroke.key.as_str(), "enter" | "space")
                {
                    this.advance_to_now();
                    this.pressed = true;
                    this.last_event = this.now();
                    cx.notify();
                }
            }))
            .on_key_up(cx.listener(|this, event: &KeyUpEvent, _, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    this.release(cx);
                }
            }))
            .on_click(cx.listener(|this, _, window, cx| {
                this.release(cx);
                if !this.is_disabled() {
                    if let Some(handler) = this.on_click.clone() {
                        handler(this, window, cx);
                    }
                }
            }))
            .child(canvas);
        TranslatedButton {
            child: button.into_any_element(),
            offset,
            remainder,
        }
    }
}
fn mix(a: Hsla, b: Hsla, t: f32) -> Hsla {
    let a: Rgba = a.into();
    let b: Rgba = b.into();
    Rgba {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
    .into()
}
fn paint_label(
    label: &Label,
    origin: Point<Pixels>,
    font_size: f32,
    line_height: f32,
    scale: f32,
    color: Hsla,
    window: &mut Window,
    cx: &mut App,
    cache: &RefCell<ButtonRenderCache>,
) {
    let mut x = origin.x;
    for letter in label.letters.iter() {
        paint_letter(
            letter,
            point(x, origin.y),
            font_size,
            line_height,
            scale,
            1.,
            0.,
            color,
            window,
            cx,
            &cache,
        );
        x += letter.width() * scale;
    }
}
fn paint_cascade(
    label: &Label,
    origin: Point<Pixels>,
    font_size: f32,
    line_height: f32,
    scale: f32,
    age: f32,
    incoming: bool,
    color: Hsla,
    window: &mut Window,
    cx: &mut App,
    cache: &RefCell<ButtonRenderCache>,
) {
    let mut x = origin.x;
    for (index, letter) in label.letters.iter().enumerate() {
        let delay = index as f32 * if incoming { 0.025 } else { 0.0125 };
        let p = if incoming {
            spring_progress(age - delay, SWAP)
        } else {
            bezier((age - delay) / 0.16, EASE_OUT)
        };
        let opacity = if incoming { p } else { 1. - p };
        let y = if incoming {
            (1. - p) * 1.05 * line_height * scale
        } else {
            -p * 1.05 * line_height * scale
        };
        let blur = 6. * if incoming { (1. - p).max(0.) } else { p };
        paint_letter(
            letter,
            point(x, origin.y + px(y)),
            font_size,
            line_height,
            scale,
            opacity,
            blur,
            color,
            window,
            cx,
            &cache,
        );
        x += letter.width() * scale;
    }
}

pub struct ButtonDemo {
    rows: Vec<Vec<Entity<MotionButton>>>,
    dark: bool,
    row_gap: f32,
    column_gap: f32,
}
impl ButtonDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let make = |button: MotionButton, cx: &mut Context<Self>| cx.new(|_| button.dark(dark));
        let rows = match variant {
            "base" => vec![
                vec![
                    make(
                        MotionButton::new("Continue").icon(ButtonIcon::ArrowRight),
                        cx,
                    ),
                    make(
                        MotionButton::new("Download")
                            .variant(ButtonVariant::Secondary)
                            .icon(ButtonIcon::Download)
                            .icon_before(true),
                        cx,
                    ),
                    make(
                        MotionButton::new("Outline").variant(ButtonVariant::Outline),
                        cx,
                    ),
                    make(MotionButton::new("Ghost").variant(ButtonVariant::Ghost), cx),
                ],
                vec![
                    make(MotionButton::new("Small").size(ButtonSize::Sm), cx),
                    make(MotionButton::new("Medium"), cx),
                    make(MotionButton::new("Large").size(ButtonSize::Lg), cx),
                    make(
                        MotionButton::new("Delete")
                            .variant(ButtonVariant::Secondary)
                            .size(ButtonSize::Icon)
                            .icon(ButtonIcon::Trash),
                        cx,
                    ),
                ],
                vec![
                    make(MotionButton::new("Ripple").ripple(true), cx),
                    make(
                        MotionButton::new("Tap me")
                            .variant(ButtonVariant::Outline)
                            .ripple(true),
                        cx,
                    ),
                ],
            ],
            "stateful" => vec![
                vec![make(
                    MotionButton::new("Save changes")
                        .kind(ButtonKind::Stateful)
                        .icon(ButtonIcon::ArrowRight)
                        .feedback_labels("Saving", "Saved", "Try again")
                        .on_click(|this, _, cx| this.demo_action(true, cx)),
                    cx,
                )],
                vec![make(
                    MotionButton::new("Submit")
                        .kind(ButtonKind::Stateful)
                        .variant(ButtonVariant::Secondary)
                        .feedback_labels("Submitting", "Done", "Failed")
                        .on_click(|this, _, cx| this.demo_action(false, cx)),
                    cx,
                )],
            ],
            "magnetic" => vec![vec![
                make(
                    MotionButton::new("Hover me")
                        .kind(ButtonKind::Magnetic)
                        .strength(0.35)
                        .icon(ButtonIcon::ArrowRight),
                    cx,
                ),
                make(
                    MotionButton::new("Subtle pull")
                        .kind(ButtonKind::Magnetic)
                        .variant(ButtonVariant::Secondary)
                        .strength(0.25),
                    cx,
                ),
                make(
                    MotionButton::new("Strong pull")
                        .kind(ButtonKind::Magnetic)
                        .variant(ButtonVariant::Outline)
                        .strength(0.5),
                    cx,
                ),
            ]],
            _ => vec![vec![
                make(
                    MotionButton::new("Continue")
                        .kind(ButtonKind::Metallic)
                        .icon(ButtonIcon::ArrowUpRight),
                    cx,
                ),
                make(
                    MotionButton::new("Generate")
                        .kind(ButtonKind::Metallic)
                        .size(ButtonSize::Sm)
                        .icon(ButtonIcon::Sparkles)
                        .icon_before(true),
                    cx,
                ),
                make(
                    MotionButton::new("Magic tools")
                        .kind(ButtonKind::Metallic)
                        .size(ButtonSize::Icon)
                        .icon(ButtonIcon::Sparkles),
                    cx,
                ),
            ]],
        };
        Self {
            rows,
            dark,
            row_gap: if variant == "base" { 24. } else { 12. },
            column_gap: if variant == "magnetic" { 16. } else { 12. },
        }
    }
}
impl Render for ButtonDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("button-demo")
            .tab_group()
            .on_key_down(|event, window, cx| {
                // The base layer supplies tab stops; the application host owns
                // traversal, including suppressing the browser's iframe jump.
                let mut modifiers = event.keystroke.modifiers;
                modifiers.shift = false;
                if event.keystroke.key == "tab" && !modifiers.modified() {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    window.prevent_default();
                    cx.stop_propagation();
                }
            })
            .size_full()
            .flex()
            .flex_col()
            .gap(px(self.row_gap))
            .items_center()
            .justify_center()
            .bg(if self.dark {
                rgb(0x202020)
            } else {
                rgb(0xf8f8f8)
            })
            .children(self.rows.iter().map(|row| {
                div()
                    .flex()
                    .flex_wrap()
                    .gap(px(self.column_gap))
                    .items_center()
                    .justify_center()
                    .children(row.clone())
            }))
    }
}

pub fn setup(variant: &str, dark: bool, cx: &mut App) {
    kit::init(cx);
    cx.text_system()
        .add_fonts(vec![Cow::Borrowed(FONT_BYTES)])
        .expect("load Geist font");
    let variant = variant.to_string();
    let options = WindowOptions {
        #[cfg(not(target_family = "wasm"))]
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(420.)), cx)),
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
pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application()
        .with_assets(ButtonAssets)
        .run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{
        BLUR_LEVELS, ButtonIcon, ButtonRenderCache, EASE_OUT, PRESS, SWAP, Spring, bezier,
        blur_level, blur_sigma, spring_progress, xml_escape,
    };
    use std::rc::Rc;
    #[test]
    fn spring_is_frame_rate_independent_and_preserves_velocity() {
        let mut coarse = Spring::new(1.);
        coarse.target = 0.93;
        coarse.advance(0.1, PRESS);
        let mut fine = Spring::new(1.);
        fine.target = 0.93;
        for _ in 0..10 {
            fine.advance(0.01, PRESS);
        }
        assert!((coarse.value - fine.value).abs() < 0.00001);
        assert!((coarse.velocity - fine.velocity).abs() < 0.00001);
        let velocity = coarse.velocity;
        coarse.target = 1.02;
        assert_eq!(coarse.velocity, velocity);
        coarse.advance(2., PRESS);
        assert!((coarse.value - 1.02).abs() < 0.0001);
    }
    #[test]
    fn source_curve_and_cascade_delays_have_exact_endpoints() {
        assert_eq!(bezier(0., EASE_OUT), 0.);
        assert_eq!(bezier(1., EASE_OUT), 1.);
        assert!((bezier(0.5, EASE_OUT) - 0.9718).abs() < 0.001);
        assert_eq!(spring_progress(-0.025, SWAP), 0.);
        assert_eq!(spring_progress(0., SWAP), 0.);
        assert!((spring_progress(1., SWAP) - 1.).abs() < 0.0001);
    }
    #[test]
    fn arbitrary_label_svg_input_is_escaped() {
        assert_eq!(xml_escape("A<&\""), "A&lt;&amp;&quot;");
    }
    #[test]
    fn continuous_blur_requests_reuse_a_finite_mask_bank() {
        let mut cache = ButtonRenderCache::default();
        for step in 0..=12_000 {
            let blur = step as f32 * 0.0005;
            let level = blur_level(blur);
            assert!((blur_sigma(level) - blur).abs() <= 0.25001);
            cache.icon(ButtonIcon::Loader, level);
        }
        assert_eq!(cache.specs.len(), BLUR_LEVELS);
        let warmed = cache.icon(ButtonIcon::Loader, blur_level(1.01));
        let visible = cache.icon(ButtonIcon::Loader, blur_level(1.24));
        assert!(Rc::ptr_eq(&warmed, &visible));
        assert!(visible.data.contains("stdDeviation=\"1\""));
        assert!(visible.data.contains("filterUnits=\"userSpaceOnUse\""));
        // Different content needs its own bank; timing/rotation/tint never do.
        cache.icon(ButtonIcon::Check, blur_level(1.01));
        assert_eq!(cache.specs.len(), BLUR_LEVELS + 1);
        assert_eq!(blur_level(-10.), 0);
        assert_eq!(blur_level(99.), 12);
    }
    #[test]
    fn visible_masks_only_use_ready_dpi_and_dedupe_urgent_requests() {
        let mut cache = ButtonRenderCache::default();
        let zero = cache.icon(ButtonIcon::Loader, 0);
        zero.ready_scale.set(Some(1.));
        let first = cache.ready_icon(ButtonIcon::Loader, 8, 1.).unwrap();
        assert!(Rc::ptr_eq(&first, &zero));
        for _ in 0..100 {
            cache.ready_icon(ButtonIcon::Loader, 8, 1.);
        }
        assert_eq!(cache.urgent_queue.len(), 1);
        let nearby = cache.icon(ButtonIcon::Loader, 6);
        nearby.ready_scale.set(Some(1.));
        assert!(Rc::ptr_eq(
            &cache.ready_icon(ButtonIcon::Loader, 8, 1.).unwrap(),
            &nearby
        ));
        assert!(cache.ready_icon(ButtonIcon::Loader, 8, 2.).is_none());
        zero.ready_scale.set(Some(2.));
        assert!(Rc::ptr_eq(
            &cache.ready_icon(ButtonIcon::Loader, 8, 2.).unwrap(),
            &zero
        ));
        let exact = cache.icon(ButtonIcon::Loader, 8);
        exact.ready_scale.set(Some(2.));
        assert!(Rc::ptr_eq(
            &cache.ready_icon(ButtonIcon::Loader, 8, 2.).unwrap(),
            &exact
        ));
    }
}
