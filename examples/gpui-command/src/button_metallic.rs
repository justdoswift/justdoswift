//! Native painting for beUI's MetallicButton chrome surface.
//!
//! Adapted from https://beui.dev/components/motion/button (MIT).
//! The original timing, geometry, nine silver stops, screen reflection and
//! translucent hovered face are retained; labels and input live in `buttons`.
use gpui_kit::*;
use std::f32::consts::PI;

type Vertex = [f32; 2];

const SILVER: [(f32, u32); 9] = [
    (0., 0x111111),
    (0.14, 0x737373),
    (0.26, 0xfafafa),
    (0.38, 0x525252),
    (0.50, 0x0a0a0a),
    (0.64, 0xa3a3a3),
    (0.75, 0xffffff),
    (0.87, 0x404040),
    (1., 0x111111),
];

fn ease_in_out(time: f32) -> f32 {
    let time = time.clamp(0., 1.);
    let (mut low, mut high) = (0_f32, 1_f32);
    for _ in 0..18 {
        let t = (low + high) * 0.5;
        let x = 3. * (1. - t).powi(2) * t * 0.77 + 3. * (1. - t) * t * t * 0.175 + t.powi(3);
        if x < time {
            low = t;
        } else {
            high = t;
        }
    }
    let t = (low + high) * 0.5;
    3. * (1. - t) * t * t + t.powi(3)
}

fn capsule(bounds: Bounds<Pixels>) -> Vec<Vertex> {
    let x = f32::from(bounds.origin.x);
    let y = f32::from(bounds.origin.y);
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    let r = h.min(w) * 0.5;
    let mut points = Vec::with_capacity(132);
    for (cx, cy, start) in [
        (x + w - r, y + r, -PI * 0.5),
        (x + w - r, y + h - r, 0.),
        (x + r, y + h - r, PI * 0.5),
        (x + r, y + r, PI),
    ] {
        for step in 0..=32 {
            let angle = start + step as f32 / 32. * PI * 0.5;
            points.push([cx + r * angle.cos(), cy + r * angle.sin()]);
        }
    }
    points
}

fn dot(point: Vertex, direction: Vertex) -> f32 {
    point[0] * direction[0] + point[1] * direction[1]
}

// Clip convex capsules against gradient-stop planes. Painting the resulting
// polygons uses fractional coordinates; no per-frame SVGs or image atlas entries
// are generated, so a running reflection cannot grow an image cache.
fn clip(polygon: &[Vertex], direction: Vertex, offset: f32) -> Vec<Vertex> {
    let Some(&last) = polygon.last() else {
        return Vec::new();
    };
    let mut output = Vec::with_capacity(polygon.len() + 2);
    let mut previous = last;
    let mut previous_distance = dot(previous, direction) - offset;
    for &current in polygon {
        let distance = dot(current, direction) - offset;
        if (distance >= 0.) != (previous_distance >= 0.) {
            let t = previous_distance / (previous_distance - distance);
            output.push([
                previous[0] + (current[0] - previous[0]) * t,
                previous[1] + (current[1] - previous[1]) * t,
            ]);
        }
        if distance >= 0. {
            output.push(current);
        }
        previous = current;
        previous_distance = distance;
    }
    output
}

fn paint_polygon(
    polygon: &[Vertex],
    bounds: Bounds<Pixels>,
    background: impl Into<Background>,
    window: &mut Window,
) {
    if polygon.len() < 3 {
        return;
    }
    let mut path = PathBuilder::fill();
    path.move_to(point(px(polygon[0][0]), px(polygon[0][1])));
    for vertex in &polygon[1..] {
        path.line_to(point(px(vertex[0]), px(vertex[1])));
    }
    path.close();
    if let Ok(mut path) = path.build() {
        // All stops share the same coordinate space, including during a press.
        path.bounds = bounds;
        window.paint_path(path, background);
    }
}

// GPUI normalizes gradient directions to its rectangle's aspect ratio. Undo
// that normalization so this paints the CSS angle, not a stretched approximation.
fn gradient_angle(direction: Vertex, bounds: Bounds<Pixels>) -> (f32, f32) {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    let (x, y) = if w > h {
        (direction[0], direction[1] * w / h)
    } else {
        (direction[0] * h / w, direction[1])
    };
    let angle = (y.atan2(x).to_degrees() + 90.).rem_euclid(360.);
    let axis = if direction[0].abs() > direction[1].abs() {
        w
    } else {
        h
    };
    (angle, axis)
}

fn paint_band(
    polygon: &[Vertex],
    bounds: Bounds<Pixels>,
    direction: Vertex,
    from: (f32, Hsla),
    to: (f32, Hsla),
    window: &mut Window,
) {
    let polygon = clip(polygon, direction, from.0);
    let polygon = clip(&polygon, [-direction[0], -direction[1]], -to.0);
    if polygon.len() < 3 {
        return;
    }
    let (angle, axis) = gradient_angle(direction, bounds);
    let center = bounds.center();
    let middle = dot([f32::from(center.x), f32::from(center.y)], direction);
    paint_polygon(
        &polygon,
        bounds,
        linear_gradient(
            angle,
            linear_color_stop(from.1, 0.5 + (from.0 - middle) / axis),
            linear_color_stop(to.1, 0.5 + (to.0 - middle) / axis),
        ),
        window,
    );
}

fn normal_cdf(value: f32) -> f32 {
    let x = value.abs();
    let t = 1. / (1. + 0.2316419 * x);
    let density = (-0.5 * x * x).exp() * 0.3989423;
    let tail = density
        * t
        * (0.31938153 + t * (-0.35656378 + t * (1.7814779 + t * (-1.8212559 + t * 1.3302745))));
    if value >= 0. { 1. - tail } else { tail }
}

fn blurred_ramp(value: f32, sigma: f32) -> f32 {
    let z = value / sigma;
    value * normal_cdf(z) + sigma * (-0.5 * z * z).exp() * 0.3989423
}

fn reflection(position: f32, sigma: f32) -> f32 {
    // Gaussian convolution of the original transparent/white@48%/transparent
    // triangle. This retains the 3px blur without a raster-image cache.
    (blurred_ramp(position, sigma) / 0.48 - blurred_ramp(position - 0.48, sigma) / (0.48 * 0.52)
        + blurred_ramp(position - 1., sigma) / 0.52)
        .clamp(0., 1.)
}

/// Paint behind a button's native text and icons.
///
/// `elapsed` is the running rim clock in seconds (pass zero when paused).
/// `hover_progress` is the separately eased 2.4s reflection transition.
/// `surface_progress` is the 150ms hover color transition, not the reflection.
pub fn paint_metallic(
    bounds: Bounds<Pixels>,
    elapsed: f32,
    hover_progress: f32,
    surface_progress: f32,
    dark: bool,
    window: &mut Window,
    _cx: &mut App,
) {
    let w = f32::from(bounds.size.width);
    let h = f32::from(bounds.size.height);
    if w <= 4. || h <= 4. {
        return;
    }
    let outer = capsule(bounds);
    let radius = px(w.min(h) * 0.5);
    window.paint_drop_shadows(
        bounds,
        radius.into(),
        &[BoxShadow::new(px(0.), px(8.), rgba(0x00000029).into()).blur_radius(px(22.))],
    );

    let half_cycle = elapsed.max(0.).rem_euclid(8.) / 4.;
    let drift = if half_cycle <= 1. {
        ease_in_out(half_cycle)
    } else {
        1. - ease_in_out(half_cycle - 1.)
    };
    let rim_w = w * 1.36;
    let rim_x = f32::from(bounds.origin.x) - w * 0.18 + rim_w * 0.13 * drift;
    let direction = [15_f32.to_radians().cos(), 15_f32.to_radians().sin()];
    let length = rim_w * direction[0] + h * direction[1];
    let middle = dot(
        [rim_x + rim_w * 0.5, f32::from(bounds.origin.y) + h * 0.5],
        direction,
    );
    for stops in SILVER.windows(2) {
        paint_band(
            &outer,
            bounds,
            direction,
            (middle + (stops[0].0 - 0.5) * length, rgb(stops[0].1).into()),
            (middle + (stops[1].0 - 0.5) * length, rgb(stops[1].1).into()),
            window,
        );
    }

    let sweep_w = w * 0.52;
    let sweep_x =
        f32::from(bounds.origin.x) - w * 0.58 + sweep_w * 3.10 * hover_progress.clamp(0., 1.);
    let skew = 12_f32.to_radians().tan();
    let norm = (1. + skew * skew).sqrt();
    let sweep_direction = [1. / norm, skew / norm];
    let sweep_origin = (sweep_x + skew * f32::from(bounds.center().y)) / norm;
    let sweep_length = sweep_w / norm;
    let sigma = 3. / sweep_length;
    let y = f32::from(bounds.origin.y);
    // The seven vertical strips approximate Gaussian attenuation at the top
    // and bottom of the clipped reflection. Maximum strip height in its soft
    // edge is 2px; the broad flat middle remains one strip.
    let edge = 6_f32.min(h * 0.25);
    let rows = [
        0.,
        edge / 3.,
        edge * 2. / 3.,
        edge,
        h - edge,
        h - edge * 2. / 3.,
        h - edge / 3.,
        h,
    ];
    for row in rows.windows(2) {
        let slice = clip(&outer, [0., 1.], y + row[0]);
        let slice = clip(&slice, [0., -1.], -(y + row[1]));
        let center_y = (row[0] + row[1]) * 0.5;
        let vertical = normal_cdf(center_y / 3.) - normal_cdf((center_y - h) / 3.);
        for band in 0..24 {
            let a = -sigma * 3. + (1. + sigma * 6.) * band as f32 / 24.;
            let b = -sigma * 3. + (1. + sigma * 6.) * (band + 1) as f32 / 24.;
            let alpha_a = reflection(a, sigma) * vertical * 0.25;
            let alpha_b = reflection(b, sigma) * vertical * 0.25;
            if alpha_a.max(alpha_b) < 0.0001 {
                continue;
            }
            // Screen blending a white light is exactly white source-over at
            // this alpha. The original white .5 × layer opacity .5 = .25.
            paint_band(
                &slice,
                bounds,
                sweep_direction,
                (sweep_origin + a * sweep_length, hsla(0., 0., 1., alpha_a)),
                (sweep_origin + b * sweep_length, hsla(0., 0., 1., alpha_b)),
                window,
            );
        }
    }

    let inner_bounds = bounds.dilate(px(-2.));
    let inner = capsule(inner_bounds);
    let surface = surface_progress.clamp(0., 1.);
    let background = if dark { 21. } else { 252. } / 255.;
    let muted = if dark { 28. } else { 245. } / 255.;
    // CSS color transitions interpolate premultiplied colors. The hover target
    // is 40% opaque, allowing the real moving chrome to show through the face.
    let alpha = 1. - surface * 0.6;
    let color = (background * (1. - surface) + muted * surface * 0.4) / alpha;
    paint_polygon(&inner, inner_bounds, hsla(0., 0., color, alpha), window);
    let inner_radius = px(f32::from(inner_bounds.size.height) * 0.5);
    window.paint_inset_shadows(
        inner_bounds,
        inner_radius.into(),
        &[
            BoxShadow::new(px(0.), px(1.), hsla(0., 0., 1., 0.28)).inset(),
            BoxShadow::new(px(0.), px(-1.), hsla(0., 0., 0., 0.16)).inset(),
        ],
    );
}
