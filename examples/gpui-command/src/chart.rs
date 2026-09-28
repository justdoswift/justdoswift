//! shadcn/ui-style charts for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/chart (Recharts).
//! No chart library — grid, bars, lines, areas and donut slices are painted
//! directly (divs + canvas paths); the tooltip is hit-tested per x-band.
use gpui_kit::{self as kit, *};
use kit::base::{Easing, Transition, transition};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;
/// Hover fade for the tooltip popover.
const HOVER_FADE: Duration = Duration::from_millis(140);
/// Entrance: series grow/fade in after a short warm-up so the animation
/// doesn't collide with WASM/WebGPU cold start.
const ENTER_DELAY: f32 = 0.15;
const ENTER_SECS: f32 = 0.55;

const FONT: &str = "Geist";
/// Plot height for cartesian charts.
const PLOT_H: f32 = 220.;
/// Donut outer radius.
const DONUT_R: f32 = 84.;
/// Donut inner radius.
const DONUT_INNER: f32 = 54.;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ChartKind {
    /// Grouped bars — one cluster per category.
    Bar,
    /// Single stacked column per category.
    Stacked,
    Line,
    /// Line with a gradient fill down to the baseline.
    Area,
    Donut,
}

/// shadcn `ChartConfig` entry — label + light/dark colors per series (or per
/// slice for a donut).
#[derive(Clone)]
pub struct ChartSeries {
    pub label: SharedString,
    pub light: u32,
    pub dark: u32,
}
impl ChartSeries {
    pub fn new(label: &str, light: u32, dark: u32) -> Self {
        Self {
            label: label.into(),
            light,
            dark,
        }
    }
    fn color(&self, dark: bool) -> Hsla {
        rgb(if dark { self.dark } else { self.light }).into()
    }
}

/// Tooltip series indicator style (`indicator` prop).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndicatorStyle {
    Dot,
    Line,
}

/// One interactive chart — hover the plot to see the tooltip.
pub struct Chart {
    kind: ChartKind,
    title: SharedString,
    caption: SharedString,
    /// Chart config — series (cartesian) or slice (donut) labels and colors.
    series: Vec<ChartSeries>,
    /// (x category label, values aligned with `series`).
    points: Vec<(SharedString, Vec<f32>)>,
    /// Extra inner label rendered in the donut hole, e.g. "1,125".
    center_label: SharedString,
    hover: Option<usize>,
    mouse: Point<Pixels>,
    indicator: IndicatorStyle,
    dark: bool,
    legend: bool,
    /// Plot bounds captured during the last paint (window coords) — used to
    /// place the tooltip next to the cursor.
    plot_bounds: Rc<Cell<Bounds<Pixels>>>,
    /// Donut slice angle ranges (radians) captured at render for hit-tests.
    slice_angles: Rc<RefCell<Vec<(f32, f32)>>>,
    /// Hovered band/slice index — read by the canvas paint pass so band
    /// washes and slice dimming animate without entity re-renders.
    hover_cell: Rc<Cell<Option<usize>>>,
    /// Elapsed-seconds clock (executor-backed, wasm-safe) shared with the
    /// canvas paint pass for hover fade smoothing.
    clock: Option<Rc<dyn Fn() -> f32>>,
    /// Paint-pass animation state (wash/slice fades) — lives here so entity
    /// re-renders don't restart the animations.
    anim: Rc<RefCell<CanvasAnim>>,
}

impl Chart {
    pub fn new(kind: ChartKind) -> Self {
        Self {
            kind,
            title: "Chart".into(),
            caption: "".into(),
            series: Vec::new(),
            points: Vec::new(),
            center_label: "".into(),
            hover: None,
            mouse: point(px(0.), px(0.)),
            indicator: IndicatorStyle::Dot,
            dark: false,
            legend: true,
            plot_bounds: Rc::new(Cell::new(Bounds::default())),
            slice_angles: Rc::new(RefCell::new(Vec::new())),
            hover_cell: Rc::new(Cell::new(None)),
            clock: None,
            anim: Rc::new(RefCell::new(CanvasAnim {
                last: 0.,
                t0: None,
                washes: Vec::new(),
                slices: Vec::new(),
            })),
        }
    }
    /// Card-style heading shown above the plot.
    pub fn title(mut self, title: &str, caption: &str) -> Self {
        self.title = title.into();
        self.caption = caption.into();
        self
    }
    /// The `ChartConfig` — one entry per series (cartesian) or slice (donut).
    pub fn config(mut self, series: Vec<ChartSeries>) -> Self {
        self.series = series;
        self
    }
    /// Rows of (category label, per-series values). For a donut each row is a
    /// slice: (label, [value]) and the row takes `series[i]`'s color.
    pub fn data(mut self, points: Vec<(&str, Vec<f32>)>) -> Self {
        self.points = points
            .into_iter()
            .map(|(l, v)| (SharedString::from(l.to_string()), v))
            .collect();
        self
    }
    pub fn center_label(mut self, label: &str) -> Self {
        self.center_label = label.into();
        self
    }
    pub fn indicator(mut self, style: IndicatorStyle) -> Self {
        self.indicator = style;
        self
    }
    pub fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// Largest stacked or single value across the data — drives the y scale.
    fn max_value(&self) -> f32 {
        let m = match self.kind {
            ChartKind::Stacked => self
                .points
                .iter()
                .map(|(_, v)| v.iter().sum::<f32>())
                .fold(0., f32::max),
            _ => self
                .points
                .iter()
                .flat_map(|(_, v)| v.iter().copied())
                .fold(0., f32::max),
        };
        m.max(1.)
    }

    fn set_hover(&mut self, index: Option<usize>, pos: Point<Pixels>, cx: &mut Context<Self>) {
        let moved = self.hover != index;
        self.hover = index;
        self.hover_cell.set(index);
        self.mouse = pos;
        // Repaint on every move while a tooltip is showing so it tracks the
        // cursor; band/slice changes repaint regardless.
        if moved || index.is_some() {
            cx.notify();
        }
    }

    fn donut_hit(&self, pos: Point<Pixels>) -> Option<usize> {
        let b = self.plot_bounds.get();
        let c = b.center();
        let dx: f32 = (pos.x - c.x).into();
        let dy: f32 = (pos.y - c.y).into();
        let r = (dx * dx + dy * dy).sqrt();
        if !(DONUT_INNER..=DONUT_R).contains(&r) {
            return None;
        }
        let angle =
            (dy.atan2(dx) + std::f32::consts::PI * 2.5).rem_euclid(std::f32::consts::PI * 2.);
        self.slice_angles
            .borrow()
            .iter()
            .position(|(a, b)| angle >= *a && angle < *b)
    }

    /// Shared colors for the current theme.
    fn palette(&self) -> Palette {
        let d = self.dark;
        Palette {
            ink: rgb(if d { 0xededed } else { 0x171717 }).into(),
            muted: rgb(if d { 0xa3a3a3 } else { 0x737373 }).into(),
            edge: rgb(if d { 0x3b3b3b } else { 0xe4e4e4 }).into(),
            surface: rgb(if d { 0x232323 } else { 0xffffff }).into(),
            accent: rgb(if d { 0x2e2e2e } else { 0xf4f4f4 }).into(),
        }
    }
}

struct Palette {
    ink: Hsla,
    muted: Hsla,
    edge: Hsla,
    surface: Hsla,
    accent: Hsla,
}

fn fmt_int(v: f32) -> String {
    let n = v.round() as i64;
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// shadcn `ChartTooltipContent` — label row + per-series indicator/name/value.
fn tooltip(
    label: &SharedString,
    series: &[ChartSeries],
    values: &[f32],
    indicator: IndicatorStyle,
    dark: bool,
) -> Div {
    let p = Palette {
        ink: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        edge: rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(),
        surface: rgb(if dark { 0x2a2a2a } else { 0xffffff }).into(),
        accent: rgb(0).into(),
    };
    div()
        .absolute()
        .flex()
        .flex_col()
        .gap_1()
        .px_2p5()
        .py_1p5()
        .min_w(px(130.))
        .rounded_md()
        .border_1()
        .border_color(p.edge)
        .bg(p.surface)
        .shadow_md()
        .font_family(FONT)
        .text_xs()
        .child(
            div()
                .text_color(p.muted)
                .font_weight(FontWeight::MEDIUM)
                .child(label.clone()),
        )
        .children(series.iter().zip(values.iter()).map(|(s, v)| {
            let dot = match indicator {
                IndicatorStyle::Dot => div()
                    .w(px(8.))
                    .h(px(8.))
                    .rounded_full()
                    .bg(s.color(dark))
                    .into_any_element(),
                IndicatorStyle::Line => div()
                    .w(px(3.))
                    .h(px(10.))
                    .rounded_sm()
                    .bg(s.color(dark))
                    .into_any_element(),
            };
            div()
                .flex()
                .items_center()
                .gap_1p5()
                .child(dot)
                .child(div().flex_1().text_color(p.muted).child(s.label.clone()))
                .child(
                    div()
                        .text_color(p.ink)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(fmt_int(*v)),
                )
                .into_any_element()
        }))
}

/// shadcn `ChartLegendContent` — centered row of dot + label (+ totals).
fn legend(series: &[ChartSeries], totals: &[f32], dark: bool) -> Div {
    let p = Palette {
        ink: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        edge: rgb(0).into(),
        surface: rgb(0).into(),
        accent: rgb(0).into(),
    };
    div()
        .flex()
        .items_center()
        .justify_center()
        .gap_5()
        .children(series.iter().zip(totals.iter()).map(|(s, t)| {
            div()
                .flex()
                .items_center()
                .gap_1p5()
                .font_family(FONT)
                .text_xs()
                .child(div().w(px(8.)).h(px(8.)).rounded_sm().bg(s.color(dark)))
                .child(div().text_color(p.muted).child(s.label.clone()))
                .child(
                    div()
                        .text_color(p.ink)
                        .font_weight(FontWeight::SEMIBOLD)
                        .child(fmt_int(*t)),
                )
                .into_any_element()
        }))
}

/// Per-frame animation state living inside the canvas paint pass — band wash
/// and slice dimming animate by repainting the canvas only; the entity tree
/// never re-renders for them.
struct CanvasAnim {
    /// Last-paint timestamp (dt-based smoothing).
    last: f32,
    /// First-paint timestamp — entrance starts ENTER_DELAY after this.
    t0: Option<f32>,
    /// Per-band wash alpha 0..1 (cartesian hover highlight).
    washes: Vec<f32>,
    /// Per-slice alpha 1..0.35 (donut hover dimming).
    slices: Vec<f32>,
}

/// Paint grid, band washes, bars/lines/areas or donut slices — plus their
/// hover animations. Hover state arrives via `hover_cell`; the fades run off
/// `clock` + `request_animation_frame`, so they cost a canvas repaint instead
/// of a full entity re-render.
fn chart_canvas(
    kind: ChartKind,
    series: Vec<ChartSeries>,
    points: Vec<(SharedString, Vec<f32>)>,
    max: f32,
    accent: Hsla,
    clock: Rc<dyn Fn() -> f32>,
    hover_cell: Rc<Cell<Option<usize>>>,
    anim: Rc<RefCell<CanvasAnim>>,
    dark: bool,
    reduce: bool,
    bounds_cell: Rc<Cell<Bounds<Pixels>>>,
    angles_cell: Rc<RefCell<Vec<(f32, f32)>>>,
) -> Canvas<()> {
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    canvas(
        move |bounds, _, _| {
            bounds_cell.set(bounds);
        },
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let w: f32 = bounds.size.width.into();
            let h: f32 = bounds.size.height.into();
            let n = points.len().max(1);

            // ── Hover fades (paint-pass driven, dt-smoothed) ────────────
            let now = clock();
            let hovered = hover_cell.get();
            let mut still_animating = false;
            let mut enter = 1f32;
            {
                let mut a = anim.borrow_mut();
                // Entrance progress: starts ENTER_DELAY after first paint,
                // ease-out cubic over ENTER_SECS. reduce_motion skips it.
                if !reduce {
                    let t0 = *a.t0.get_or_insert(now + ENTER_DELAY);
                    let raw = ((now - t0) / ENTER_SECS).clamp(0., 1.);
                    enter = 1. - (1. - raw).powi(3);
                    still_animating |= raw < 1.;
                }
                let dt = (now - a.last).clamp(0., 0.1);
                a.last = now;
                // Exponential smoothing ≈140ms settle at 60fps.
                let k = 1. - (-dt / 0.05).exp();
                a.washes.resize(n, 0.);
                a.slices.resize(n, 1.);
                for i in 0..n {
                    let wash_t = if hovered == Some(i) { 1. } else { 0. };
                    let slice_t = match hovered {
                        None => 1.,
                        Some(hv) if hv == i => 1.,
                        Some(_) => 0.35,
                    };
                    let w0 = a.washes[i];
                    let s0 = a.slices[i];
                    a.washes[i] = w0 + (wash_t - w0) * k;
                    a.slices[i] = s0 + (slice_t - s0) * k;
                    still_animating |=
                        (a.washes[i] - wash_t).abs() > 0.01 || (a.slices[i] - slice_t).abs() > 0.01;
                }
            }
            if still_animating {
                window.request_animation_frame();
            }
            let washes = anim.borrow().washes.clone();
            let slices = anim.borrow().slices.clone();
            let band = w / n as f32;

            if kind == ChartKind::Donut {
                let c = o + point(px(w / 2.), px(h / 2.));
                let total: f32 = points.iter().map(|(_, v)| v[0]).sum::<f32>().max(1.);
                let mut a0 = 0f32;
                let mut ranges = Vec::with_capacity(points.len());
                for (i, (_, v)) in points.iter().enumerate() {
                    let sweep = v[0] / total * std::f32::consts::PI * 2. * enter;
                    let a1 = a0 + sweep;
                    ranges.push((a0, a1));
                    let base = series
                        .get(i)
                        .map(|s| s.color(dark))
                        .unwrap_or_else(|| rgb(0x888888).into());
                    let alpha = slices.get(i).copied().unwrap_or(1.) * enter;
                    let color = hsla(base.h, base.s, base.l, alpha);
                    paint_ring_segment(c, px(DONUT_R), px(DONUT_INNER), a0, a1, color, window);
                    a0 = a1;
                }
                *angles_cell.borrow_mut() = ranges;
                return;
            }

            // Horizontal grid lines (CartesianGrid vertical={false}).
            for i in 0..=4 {
                let y = o.y + px(h * i as f32 / 4.);
                let mut b = PathBuilder::stroke(px(1.));
                b.move_to(point(o.x, y));
                b.line_to(point(o.x + px(w), y));
                if let Ok(path) = b.build() {
                    window.paint_path(path, edge);
                }
            }
            // Band hover washes (behind the series).
            for (i, hl) in washes.iter().enumerate() {
                if *hl > 0.01 {
                    window.paint_quad(fill(
                        Bounds::new(point(o.x + px(band * i as f32), o.y), size(px(band), px(h))),
                        hsla(accent.h, accent.s, accent.l, *hl),
                    ));
                }
            }
            let top_round = Corners {
                top_left: px(3.),
                top_right: px(3.),
                bottom_right: px(0.),
                bottom_left: px(0.),
            };
            match kind {
                ChartKind::Bar => {
                    let k = series.len().max(1) as f32;
                    let group_w = 26. * k + 4. * (k - 1.);
                    for (i, (_, values)) in points.iter().enumerate() {
                        let gx = o.x + px(band * (i as f32 + 0.5) - group_w / 2.);
                        for (si, v) in values.iter().enumerate() {
                            let bh = (h * v / max * enter).max(0.);
                            if bh < 0.5 {
                                continue;
                            }
                            let c = series
                                .get(si)
                                .map(|s| s.color(dark))
                                .unwrap_or_else(|| rgb(0x888888).into());
                            let c = hsla(c.h, c.s, c.l, c.a * enter);
                            window.paint_quad(
                                fill(
                                    Bounds::new(
                                        point(gx + px(si as f32 * 30.), o.y + px(h - bh)),
                                        size(px(26.), px(bh)),
                                    ),
                                    c,
                                )
                                .corner_radii(top_round),
                            );
                        }
                    }
                }
                ChartKind::Stacked => {
                    for (i, (_, values)) in points.iter().enumerate() {
                        let cx0 = o.x + px(band * (i as f32 + 0.5) - 18.);
                        let mut y = h;
                        for (si, v) in values.iter().enumerate() {
                            let sh = (h * v / max * enter).max(0.);
                            y -= sh;
                            if sh < 0.5 {
                                continue;
                            }
                            let c = series
                                .get(si)
                                .map(|s| s.color(dark))
                                .unwrap_or_else(|| rgb(0x888888).into());
                            let c = hsla(c.h, c.s, c.l, c.a * enter);
                            let q = fill(
                                Bounds::new(point(cx0, o.y + px(y)), size(px(36.), px(sh))),
                                c,
                            );
                            // Topmost segment gets the rounded corners.
                            let q = if si == values.len() - 1 {
                                q.corner_radii(top_round)
                            } else {
                                q
                            };
                            window.paint_quad(q);
                        }
                    }
                }
                ChartKind::Line | ChartKind::Area => {
                    for (si, s) in series.iter().enumerate() {
                        let pts: Vec<Point<Pixels>> = points
                            .iter()
                            .enumerate()
                            .map(|(i, (_, v))| {
                                point(
                                    o.x + px(band * (i as f32 + 0.5)),
                                    o.y + px(
                                        h * (1. - v.get(si).copied().unwrap_or(0.) / max * enter)
                                    ),
                                )
                            })
                            .collect();
                        if pts.is_empty() {
                            continue;
                        }
                        if kind == ChartKind::Area {
                            let mut b = PathBuilder::fill();
                            b.move_to(point(pts[0].x, o.y + px(h)));
                            for p in &pts {
                                b.line_to(*p);
                            }
                            b.line_to(point(pts[pts.len() - 1].x, o.y + px(h)));
                            b.close();
                            if let Ok(path) = b.build() {
                                let c = s.color(dark);
                                window.paint_path(
                                    path,
                                    linear_gradient(
                                        90.,
                                        linear_color_stop(hsla(c.h, c.s, c.l, 0.4 * enter), 0.),
                                        linear_color_stop(hsla(c.h, c.s, c.l, 0.02 * enter), 1.),
                                    ),
                                );
                            }
                        }
                        let mut b = PathBuilder::stroke(px(2.));
                        for (i, p) in pts.iter().enumerate() {
                            if i == 0 {
                                b.move_to(*p);
                            } else {
                                b.line_to(*p);
                            }
                        }
                        let c = s.color(dark);
                        let c = hsla(c.h, c.s, c.l, c.a * enter);
                        if let Ok(path) = b.build() {
                            window.paint_path(path, c);
                        }
                        // Vertex dots.
                        for p in &pts {
                            let mut d = PathBuilder::fill();
                            let r = px(3.5);
                            for k in 0..=24 {
                                let a = k as f32 / 24. * std::f32::consts::PI * 2.;
                                let q = *p + point(r * a.cos(), r * a.sin());
                                if k == 0 {
                                    d.move_to(q);
                                } else {
                                    d.line_to(q);
                                }
                            }
                            if let Ok(path) = d.build() {
                                window.paint_path(path, c);
                            }
                        }
                    }
                }
                _ => {}
            }
        },
    )
}

/// A donut ring sector between angles a0..a1 (radians, 0 = top).
fn paint_ring_segment(
    center: Point<Pixels>,
    r_out: Pixels,
    r_in: Pixels,
    a0: f32,
    a1: f32,
    color: Hsla,
    window: &mut Window,
) {
    // Angles stored from top going clockwise; convert to standard position.
    let to_pt = |a: f32, r: Pixels| {
        let t = a - std::f32::consts::PI / 2.;
        center + point(r * t.cos(), r * t.sin())
    };
    let large = (a1 - a0) > std::f32::consts::PI;
    let mut b = PathBuilder::fill();
    b.move_to(to_pt(a0, r_out));
    b.arc_to(point(r_out, r_out), px(0.), large, true, to_pt(a1, r_out));
    b.line_to(to_pt(a1, r_in));
    b.arc_to(point(r_in, r_in), px(0.), large, false, to_pt(a0, r_in));
    b.close();
    if let Ok(path) = b.build() {
        window.paint_path(path, color);
    }
}

impl Render for Chart {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let p = self.palette();
        let kind = self.kind;
        let dark = self.dark;
        let max = self.max_value();
        let count = self.points.len();
        let weak = cx.entity().downgrade();

        // Wasm-safe elapsed-seconds clock, shared with the canvas paint pass.
        if self.clock.is_none() {
            let executor = cx.background_executor().clone();
            let start = executor.now();
            self.clock = Some(Rc::new(move || {
                executor.now().duration_since(start).as_secs_f32()
            }));
        }
        let hover_policy = Transition::new(HOVER_FADE).easing(Easing::EaseOut);

        // ── Plot body — the canvas paints grid, washes, series and all
        //    their animations itself, so those frames cost a repaint only.
        let canvas_el = chart_canvas(
            kind,
            self.series.clone(),
            self.points.clone(),
            max,
            p.accent,
            self.clock
                .clone()
                .unwrap_or_else(|| Rc::new(|| 0f32) as Rc<dyn Fn() -> f32>),
            self.hover_cell.clone(),
            self.anim.clone(),
            dark,
            cx.reduce_motion(),
            self.plot_bounds.clone(),
            self.slice_angles.clone(),
        );

        let mut plot = div().relative().w_full().h(px(PLOT_H));
        // Canvas underlay: grid + washes + series (or the donut itself).
        plot = plot.child(div().absolute().inset_0().child(canvas_el.size_full()));

        // Transparent hover bands (cartesian) — one per category; the donut
        // hit-tests angles on the whole plot instead.
        if kind == ChartKind::Donut {
            let wk = weak.clone();
            let wk2 = weak.clone();
            plot = plot.child(
                div()
                    .id("chart-donut-hover")
                    .absolute()
                    .inset_0()
                    .on_mouse_move(move |e: &MouseMoveEvent, _, cx| {
                        let _ = wk.update(cx, |this, cx| {
                            let hit = this.donut_hit(e.position);
                            this.set_hover(hit, e.position, cx);
                        });
                    })
                    .on_hover(move |h: &bool, _, cx| {
                        if !*h {
                            let _ = wk2.update(cx, |this, cx| {
                                let m = this.mouse;
                                this.set_hover(None, m, cx);
                            });
                        }
                    }),
            );
        } else {
            let cols = (0..count)
                .map(|i| {
                    let wk = weak.clone();
                    div().id(("chart-band", i)).flex_1().h_full().on_mouse_move(
                        move |e: &MouseMoveEvent, _, cx| {
                            let _ = wk.update(cx, |this, cx| {
                                this.set_hover(Some(i), e.position, cx);
                            });
                        },
                    )
                })
                .collect::<Vec<_>>();
            let wk = weak.clone();
            plot = plot.child(
                div()
                    .id("chart-bands")
                    .absolute()
                    .inset_0()
                    .flex()
                    .children(cols)
                    .on_hover(move |h: &bool, _, cx| {
                        if !*h {
                            let _ = wk.update(cx, |this, cx| {
                                let m = this.mouse;
                                this.set_hover(None, m, cx);
                            });
                        }
                    }),
            );
        }

        // Tooltip — follows the cursor inside the plot.
        if let Some(i) = self.hover {
            if let Some((label, values)) = self.points.get(i) {
                let b = self.plot_bounds.get();
                let x = f32::from(self.mouse.x - b.origin.x) + 14.;
                let y = f32::from(self.mouse.y - b.origin.y) - 10.;
                // Fade + slight rise each time the hovered band changes.
                let tip_t = transition(
                    ("chart-tip", SharedString::from(i.to_string())),
                    1f32,
                    hover_policy.clone(),
                    window,
                    cx,
                );
                let tip_series: Vec<ChartSeries> = if kind == ChartKind::Donut {
                    vec![
                        self.series
                            .get(i)
                            .cloned()
                            .unwrap_or_else(|| ChartSeries::new("", 0x888888, 0x888888)),
                    ]
                } else {
                    self.series.clone()
                };
                plot = plot.child(
                    tooltip(label, &tip_series, values, self.indicator, dark)
                        .left(px(x))
                        .top(px(y - (1. - tip_t) * 5.))
                        .opacity(tip_t),
                );
            }
        }

        // ── X axis labels ────────────────────────────────────────────────
        let axis = if matches!(kind, ChartKind::Donut) {
            div().into_any_element()
        } else {
            div()
                .flex()
                .w_full()
                .pt_2()
                .children(self.points.iter().map(|(l, _)| {
                    div()
                        .flex_1()
                        .flex()
                        .justify_center()
                        .font_family(FONT)
                        .text_xs()
                        .text_color(p.muted)
                        .child(l.clone())
                }))
                .into_any_element()
        };

        // ── Legend with per-series totals (donut: per-slice value) ────────
        let totals: Vec<f32> = if kind == ChartKind::Donut {
            self.points.iter().map(|(_, v)| v[0]).collect()
        } else {
            (0..self.series.len())
                .map(|si| self.points.iter().map(|(_, v)| v[si]).sum())
                .collect()
        };
        let legend_el = if self.legend {
            legend(&self.series, &totals, dark).into_any_element()
        } else {
            div().into_any_element()
        };

        // ── Donut center label ───────────────────────────────────────────
        if kind == ChartKind::Donut {
            plot = plot.child(
                div()
                    .absolute()
                    .inset_0()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .font_family(FONT)
                    .child(
                        div()
                            .text_size(px(24.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(p.ink)
                            .child(self.center_label.clone()),
                    )
                    .child(div().text_xs().text_color(p.muted).child("Visitors")),
            );
        }

        div()
            .w(px(560.))
            .flex()
            .flex_col()
            .gap_3()
            .p_5()
            .rounded_xl()
            .border_1()
            .border_color(p.edge)
            .bg(p.surface)
            .font_family(FONT)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_0p5()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(p.ink)
                            .child(self.title.clone()),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(p.muted)
                            .child(self.caption.clone()),
                    ),
            )
            .child(plot)
            .child(axis)
            .child(legend_el)
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

const DESKTOP: (u32, u32) = (0x2563eb, 0x3b82f6);
const MOBILE: (u32, u32) = (0x60a5fa, 0x93c5fd);
// shadcn chart-1..5 palette approximations.
const CHART1: (u32, u32) = (0xe76e50, 0x5b4bc4);
const CHART2: (u32, u32) = (0x2a9d90, 0x34b58c);
const CHART3: (u32, u32) = (0x274754, 0xf2a72f);
const CHART4: (u32, u32) = (0xd9d43e, 0xb455ad);
const CHART5: (u32, u32) = (0xf2a72f, 0xe05252);

const MONTHS: [(&str, [f32; 2]); 6] = [
    ("Jan", [186., 80.]),
    ("Feb", [305., 200.]),
    ("Mar", [237., 120.]),
    ("Apr", [73., 190.]),
    ("May", [209., 130.]),
    ("Jun", [214., 140.]),
];

fn month_data() -> Vec<(&'static str, Vec<f32>)> {
    MONTHS.iter().map(|(m, v)| (*m, v.to_vec())).collect()
}

fn dm_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", DESKTOP.0, DESKTOP.1),
        ChartSeries::new("Mobile", MOBILE.0, MOBILE.1),
    ]
}

pub struct ChartDemo {
    dark: bool,
    chart: Entity<Chart>,
}

impl ChartDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let chart = cx.new(|_| match variant {
            "stacked" => Chart::new(ChartKind::Stacked)
                .title("Bar Chart - Stacked", "Desktop vs mobile stacked per month")
                .config(dm_series())
                .data(month_data())
                .dark(dark),
            "line" => Chart::new(ChartKind::Line)
                .title("Line Chart", "Desktop vs mobile per month")
                .config(dm_series())
                .data(month_data())
                .indicator(IndicatorStyle::Line)
                .dark(dark),
            "area" => Chart::new(ChartKind::Area)
                .title("Area Chart", "Desktop vs mobile per month")
                .config(dm_series())
                .data(month_data())
                .indicator(IndicatorStyle::Line)
                .dark(dark),
            "donut" => Chart::new(ChartKind::Donut)
                .title("Pie Chart - Donut", "Visitors by browser")
                .config(vec![
                    ChartSeries::new("Chrome", CHART1.0, CHART1.1),
                    ChartSeries::new("Safari", CHART2.0, CHART2.1),
                    ChartSeries::new("Firefox", CHART3.0, CHART3.1),
                    ChartSeries::new("Edge", CHART4.0, CHART4.1),
                    ChartSeries::new("Other", CHART5.0, CHART5.1),
                ])
                .data(vec![
                    ("Chrome", vec![275.]),
                    ("Safari", vec![200.]),
                    ("Firefox", vec![187.]),
                    ("Edge", vec![173.]),
                    ("Other", vec![90.]),
                ])
                .center_label("925")
                .dark(dark),
            _ => Chart::new(ChartKind::Bar)
                .title(
                    "Bar Chart - Interactive",
                    "Showing total visitors for the last 6 months",
                )
                .config(dm_series())
                .data(month_data())
                .dark(dark),
        });
        Self { dark, chart }
    }
}

impl Render for ChartDemo {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_2()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(self.chart.clone())
            .child(
                div()
                    .h(px(16.))
                    .text_xs()
                    .text_color(muted)
                    .child("Hover the chart for the tooltip"),
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
        window_bounds: Some(WindowBounds::centered(size(px(700.), px(520.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| ChartDemo::new(&variant, dark, cx))
    })
    .expect("open chart gallery");
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
