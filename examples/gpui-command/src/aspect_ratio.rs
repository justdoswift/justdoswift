//! shadcn/ui-style aspect-ratio box for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/aspect-ratio
use gpui_kit::{self as kit, *};
use kit::base::StyledExt;
use std::borrow::Cow;

const FONT: &str = "Geist";

/// Displays content within a desired width/height ratio, like shadcn's
/// `AspectRatio`: it takes the full width of its parent and derives its height
/// from `ratio`. Place media (or any element) inside as a child.
pub struct AspectRatio {
    style: StyleRefinement,
    ratio: f32,
    children: Vec<AnyElement>,
}

impl AspectRatio {
    /// `ratio` is width / height, e.g. `16. / 9.` or `1.`.
    pub fn new(ratio: f32) -> Self {
        Self {
            style: StyleRefinement::default(),
            ratio,
            children: Vec::new(),
        }
    }
}

impl ParentElement for AspectRatio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AspectRatio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for AspectRatio {
    type Element = AnyElement;
    fn into_element(self) -> Self::Element {
        div()
            .w_full()
            .aspect_ratio(self.ratio)
            .refine_style(&self.style)
            .children(self.children)
            .into_any_element()
    }
}

/// A painted "photo" placeholder — sky, sun and ridgelines — standing in for
/// real image content in the demos.
fn photo(dark: bool) -> impl IntoElement {
    canvas(
        // The canvas element has no intrinsic size; fill the ratio box so the
        // paint bounds reflect the real frame.
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let w = bounds.size.width;
            let h = bounds.size.height;
            let origin = bounds.origin;

            // Sky.
            let mut sky = PathBuilder::fill();
            sky.move_to(origin);
            sky.line_to(origin + point(w, px(0.)));
            sky.line_to(origin + point(w, h));
            sky.line_to(origin + point(px(0.), h));
            if let Ok(path) = sky.build() {
                window.paint_path(path, rgb(if dark { 0x1e2a38 } else { 0xcfe0ee }));
            }

            // Sun.
            let sun_r = w.min(h) * 0.11;
            let sun_c = origin + point(w * 0.74, h * 0.28);
            let mut sun = PathBuilder::fill();
            sun.move_to(sun_c + point(sun_r, px(0.)));
            sun.arc_to(
                point(sun_r, sun_r),
                px(0.),
                true,
                true,
                sun_c + point(-sun_r, px(0.)),
            );
            sun.arc_to(
                point(sun_r, sun_r),
                px(0.),
                true,
                true,
                sun_c + point(sun_r, px(0.)),
            );
            if let Ok(path) = sun.build() {
                window.paint_path(path, rgb(if dark { 0xe8c468 } else { 0xf5cf6e }));
            }

            // Far ridge.
            let mut far = PathBuilder::fill();
            far.move_to(origin + point(px(0.), h));
            far.line_to(origin + point(w * 0.18, h * 0.52));
            far.line_to(origin + point(w * 0.42, h * 0.78));
            far.line_to(origin + point(w * 0.6, h * 0.5));
            far.line_to(origin + point(w, h * 0.86));
            far.line_to(origin + point(w, h));
            if let Ok(path) = far.build() {
                window.paint_path(path, rgb(if dark { 0x33475c } else { 0x9db8cf }));
            }

            // Near ridge.
            let mut near = PathBuilder::fill();
            near.move_to(origin + point(px(0.), h));
            near.line_to(origin + point(w * 0.3, h * 0.62));
            near.line_to(origin + point(w * 0.55, h));
            near.line_to(origin + point(px(0.), h));
            if let Ok(path) = near.build() {
                window.paint_path(path, rgb(if dark { 0x243649 } else { 0x6f92ad }));
            }
        },
    )
    .size_full()
}

struct Palette {
    page: Hsla,
    chip_bg: Hsla,
    chip_fg: Hsla,
    border: Hsla,
    muted: Hsla,
}

fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            page: rgb(0x202020).into(),
            chip_bg: rgb(0x000000).opacity(0.45).into(),
            chip_fg: rgb(0xededed).into(),
            border: rgb(0x3b3b3b).into(),
            muted: rgb(0xa3a3a3).into(),
        }
    } else {
        Palette {
            page: rgb(0xf8f8f8).into(),
            chip_bg: rgb(0x171717).opacity(0.55).into(),
            chip_fg: rgb(0xffffff).into(),
            border: rgb(0xe4e4e4).into(),
            muted: rgb(0x737373).into(),
        }
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Landscape,
    Square,
    Portrait,
}

pub struct AspectRatioDemo {
    kind: DemoKind,
    dark: bool,
}

impl AspectRatioDemo {
    fn new(variant: &str, dark: bool, _: &mut Context<Self>) -> Self {
        let kind = match variant {
            "square" => DemoKind::Square,
            "portrait" => DemoKind::Portrait,
            _ => DemoKind::Landscape,
        };
        Self { kind, dark }
    }
}

impl Render for AspectRatioDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let colors = palette(self.dark);
        let (ratio, width, label) = match self.kind {
            DemoKind::Landscape => (16. / 9., px(460.), "16 : 9"),
            DemoKind::Square => (1., px(340.), "1 : 1"),
            DemoKind::Portrait => (9. / 16., px(215.), "9 : 16"),
        };

        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .p_6()
            .font_family(FONT)
            .bg(colors.page)
            .child(
                div()
                    .w(width)
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        AspectRatio::new(ratio)
                            .rounded_lg()
                            .overflow_hidden()
                            .border_1()
                            .border_color(colors.border)
                            .child(
                                div().size_full().child(photo(self.dark)).child(
                                    div()
                                        .absolute()
                                        .bottom_2()
                                        .left_2()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(colors.chip_bg)
                                        .text_xs()
                                        .text_color(colors.chip_fg)
                                        .child(label),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(colors.muted)
                            .child("Content keeps its shape as the parent resizes."),
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(560.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| AspectRatioDemo::new(&variant, dark, cx))
    })
    .expect("open aspect ratio gallery");
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

#[cfg(test)]
mod tests {
    use super::AspectRatio;
    use gpui_kit::{ParentElement, Styled};

    #[test]
    fn stores_the_ratio() {
        let ratio = AspectRatio::new(16. / 9.).ratio;
        assert!((ratio - 16. / 9.).abs() < f32::EPSILON);
    }

    #[test]
    fn accepts_children_and_style() {
        let boxed = AspectRatio::new(1.).rounded_md().child("content");
        assert_eq!(boxed.children.len(), 1);
    }
}
