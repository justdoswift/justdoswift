//! shadcn/ui-style avatar for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/avatar
//!
//! Demo photos are bundled JPEGs decoded into `RenderImage` frames, so the
//! `img` element exercises the real load/fallback path without a network.
use gpui_kit::{self as kit, *};
use std::borrow::Cow;
use std::sync::Arc;

const FONT: &str = "Geist";

/// Avatar picture: one of the bundled photos, or a broken source to
/// exercise the fallback path.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AvatarImage {
    /// Bundled 96×96 photo keyed on a seed (four photos included).
    Photo(u8),
    /// A resource that cannot be decoded — the initials fallback shows.
    Broken,
}

/// Diameter presets matching shadcn's size prop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AvatarSize {
    Sm,
    Default,
    Lg,
}

impl AvatarSize {
    fn side(self) -> Pixels {
        match self {
            Self::Sm => px(24.),
            Self::Default => px(32.),
            Self::Lg => px(40.),
        }
    }
}

/// Bottom-right status badge.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AvatarBadge {
    Dot,
    /// Dot plus an icon glyph inside it.
    Icon,
}

/// Renders a circular avatar element: image → initials fallback, optional
/// bottom-right badge, size presets and group overlap handled by the caller.
pub struct Avatar {
    image: Option<AvatarImage>,
    initials: SharedString,
    size: AvatarSize,
    badge: Option<AvatarBadge>,
    dark: bool,
    surface: Option<Hsla>,
}

impl Avatar {
    pub fn new() -> Self {
        Self {
            image: None,
            initials: SharedString::from("CN"),
            size: AvatarSize::Default,
            badge: None,
            dark: false,
            surface: None,
        }
    }
    /// Shows a picture generated from the seed.
    pub fn image(mut self, image: AvatarImage) -> Self {
        self.image = Some(image);
        self
    }
    /// Initials rendered when no picture is set or the picture fails.
    pub fn initials(mut self, initials: impl Into<SharedString>) -> Self {
        self.initials = initials.into();
        self
    }
    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
    /// Bottom-right status dot; `Icon` draws a plus glyph inside it.
    pub fn badge(mut self, badge: AvatarBadge) -> Self {
        self.badge = Some(badge);
        self
    }
    /// Ring color used behind the badge (page/row background).
    pub fn ring(mut self, color: Hsla) -> Self {
        self.surface = Some(color);
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

fn palette(dark: bool) -> (Hsla, Hsla, Hsla) {
    (
        rgb(if dark { 0x2c2c2c } else { 0xececed }).into(), // fallback bg
        rgb(if dark { 0xd4d4d4 } else { 0x525252 }).into(), // fallback ink
        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into(), // page surface
    )
}

/// Decodes a bundled JPEG photo into a cached `RenderImage`. Decoding once
/// per photo keeps render-time `img` construction cheap.
fn photo_image(seed: u8) -> ImageSource {
    const PHOTOS: [&[u8]; 4] = [
        include_bytes!("../assets/avatar-1.jpg"),
        include_bytes!("../assets/avatar-12.jpg"),
        include_bytes!("../assets/avatar-32.jpg"),
        include_bytes!("../assets/avatar-47.jpg"),
    ];
    thread_local! {
        static CACHE: std::cell::RefCell<
            std::collections::HashMap<u8, Arc<RenderImage>>,
        > = std::cell::RefCell::new(std::collections::HashMap::new());
    }
    let index = seed % PHOTOS.len() as u8;
    ImageSource::Render(CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry(index)
            .or_insert_with(|| {
                let pixels = image::load_from_memory(PHOTOS[index as usize])
                    .expect("decode bundled avatar photo")
                    .to_rgba8();
                Arc::new(RenderImage::new(vec![image::Frame::new(pixels)]))
            })
            .clone()
    }))
}

impl IntoElement for Avatar {
    type Element = gpui::Div;

    fn into_element(self) -> Self::Element {
        let (fallback_bg, fallback_ink, page) = palette(self.dark);
        let ring = self.surface.unwrap_or(page);
        let side = self.size.side();
        let badge_side = match self.size {
            AvatarSize::Sm => px(10.),
            AvatarSize::Default => px(12.),
            AvatarSize::Lg => px(14.),
        };
        let text_px = match self.size {
            AvatarSize::Sm => px(10.),
            AvatarSize::Default => px(12.),
            AvatarSize::Lg => px(14.),
        };

        let initials_text = self.initials.clone();
        let initials_el = move || {
            div()
                .size_full()
                .rounded(px(9999.))
                .flex()
                .items_center()
                .justify_center()
                .bg(fallback_bg)
                .font_family(FONT)
                .text_size(text_px)
                .font_weight(FontWeight::MEDIUM)
                .text_color(fallback_ink)
                .child(initials_text.clone())
                .into_any_element()
        };

        let mut circle = div()
            .flex_none()
            .size(side)
            .rounded(px(9999.))
            .overflow_hidden()
            .bg(fallback_bg);
        match self.image {
            Some(AvatarImage::Photo(seed)) => {
                circle = circle.child(
                    img(photo_image(seed))
                        .size_full()
                        .rounded(px(9999.))
                        .object_fit(ObjectFit::Cover)
                        .with_fallback(initials_el.clone()),
                );
            }
            Some(AvatarImage::Broken) => {
                circle = circle.child(
                    img("avatar/missing.png")
                        .size_full()
                        .rounded(px(9999.))
                        .object_fit(ObjectFit::Cover)
                        .with_fallback(initials_el),
                );
            }
            None => {
                circle = circle.child(initials_el());
            }
        }

        let mut root = div().relative().flex_none().size(side).child(circle);
        if let Some(badge) = self.badge {
            let mut dot = div()
                .absolute()
                .bottom(px(-1.))
                .right(px(-1.))
                .size(badge_side)
                .rounded(px(9999.))
                .border_2()
                .border_color(ring)
                .bg(rgb(0x2ea44f));
            if badge == AvatarBadge::Icon {
                dot = dot
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().size(px(7.)).child(plus_icon(rgb(0xffffff).into())));
            }
            root = root.child(dot);
        }
        root
    }
}

/// Small plus glyph for the badge icon.
fn plus_icon(color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let w = bounds.size.width;
            let mut b = PathBuilder::stroke(px(1.4));
            b.move_to(o + point(w * 0.5, w * 0.18));
            b.line_to(o + point(w * 0.5, w * 0.82));
            b.move_to(o + point(w * 0.18, w * 0.5));
            b.line_to(o + point(w * 0.82, w * 0.5));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Badge,
    BadgeIcon,
    Group,
    GroupCount,
    GroupIcon,
    Sizes,
}

fn avatar_row(kind: DemoKind, dark: bool) -> Div {
    let (_, _, page) = palette(dark);
    let row = div().flex().items_center().gap_4();
    match kind {
        DemoKind::Basic => row
            .child(
                Avatar::new()
                    .dark(dark)
                    .image(AvatarImage::Photo(0))
                    .initials("CN"),
            )
            .child(Avatar::new().dark(dark).initials("LR"))
            .child(
                Avatar::new()
                    .dark(dark)
                    .image(AvatarImage::Broken)
                    .initials("ER"),
            ),
        DemoKind::Badge => row.child(
            Avatar::new()
                .dark(dark)
                .image(AvatarImage::Photo(1))
                .initials("CN")
                .badge(AvatarBadge::Dot)
                .ring(page),
        ),
        DemoKind::BadgeIcon => row.child(
            Avatar::new()
                .dark(dark)
                .image(AvatarImage::Photo(2))
                .initials("PP")
                .badge(AvatarBadge::Icon)
                .ring(page),
        ),
        DemoKind::Group => group_row(
            dark,
            [0u8, 1, 2]
                .into_iter()
                .map(|s| ("", Some(s), false))
                .collect(),
            None,
        ),
        DemoKind::GroupCount => group_row(
            dark,
            [0u8, 1, 2]
                .into_iter()
                .map(|s| ("", Some(s), false))
                .collect(),
            Some("+3"),
        ),
        DemoKind::GroupIcon => group_row(
            dark,
            [0u8, 1, 2]
                .into_iter()
                .map(|s| ("", Some(s), false))
                .collect(),
            None,
        )
        .child(
            div()
                .ml(px(-10.))
                .size(px(32.))
                .rounded(px(9999.))
                .border_2()
                .border_color(page)
                .bg(rgb(if dark { 0x3a3a3a } else { 0xe4e4e4 }))
                .flex()
                .items_center()
                .justify_center()
                .child(div().size(px(10.)).child(plus_icon(
                    rgb(if dark { 0xd4d4d4 } else { 0x525252 }).into(),
                ))),
        ),
        DemoKind::Sizes => row
            .child(
                Avatar::new()
                    .dark(dark)
                    .size(AvatarSize::Sm)
                    .image(AvatarImage::Photo(3))
                    .initials("CN"),
            )
            .child(
                Avatar::new()
                    .dark(dark)
                    .image(AvatarImage::Photo(3))
                    .initials("CN"),
            )
            .child(
                Avatar::new()
                    .dark(dark)
                    .size(AvatarSize::Lg)
                    .image(AvatarImage::Photo(3))
                    .initials("CN"),
            ),
    }
}

/// Overlapping avatars sharing a page-colored ring.
fn group_row(dark: bool, items: Vec<(&str, Option<u8>, bool)>, count: Option<&'static str>) -> Div {
    let (_, ink, page) = palette(dark);
    let mut row = div().flex().items_center();
    for (index, (initials, seed, broken)) in items.into_iter().enumerate() {
        let mut spec = Avatar::new().dark(dark).ring(page);
        if let Some(seed) = seed {
            spec = spec.image(if broken {
                AvatarImage::Broken
            } else {
                AvatarImage::Photo(seed)
            });
        }
        spec = spec.initials(if initials.is_empty() {
            ["CN", "LR", "ER"][index % 3]
        } else {
            initials
        });
        let avatar: Div = if index == 0 {
            div().child(spec)
        } else {
            div().ml(px(-10.)).child(spec)
        };
        row = row.child(avatar);
    }
    if let Some(count) = count {
        row = row.child(
            div()
                .ml(px(-10.))
                .size(px(32.))
                .rounded(px(9999.))
                .border_2()
                .border_color(page)
                .bg(rgb(if dark { 0x3a3a3a } else { 0xe4e4e4 }))
                .flex()
                .items_center()
                .justify_center()
                .font_family(FONT)
                .text_size(px(11.))
                .font_weight(FontWeight::MEDIUM)
                .text_color(ink)
                .child(count),
        );
    }
    row
}

pub struct AvatarDemo {
    kind: DemoKind,
    dark: bool,
}

impl Render for AvatarDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let label = match self.kind {
            DemoKind::Basic => "image · initials · broken image → fallback",
            DemoKind::Badge => "AvatarBadge — status dot",
            DemoKind::BadgeIcon => "AvatarBadge — icon inside",
            DemoKind::Group => "AvatarGroup — overlapping",
            DemoKind::GroupCount => "AvatarGroupCount — +3",
            DemoKind::GroupIcon => "AvatarGroupCount — icon inside",
            DemoKind::Sizes => "sm · default · lg",
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
                    .child(avatar_row(self.kind, self.dark))
                    .child(div().text_xs().text_color(muted).child(label)),
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(480.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|_| AvatarDemo {
            kind: match variant.as_str() {
                "badge" => DemoKind::Badge,
                "badge-icon" => DemoKind::BadgeIcon,
                "group" => DemoKind::Group,
                "group-count" => DemoKind::GroupCount,
                "group-icon" => DemoKind::GroupIcon,
                "sizes" => DemoKind::Sizes,
                _ => DemoKind::Basic,
            },
            dark,
        })
    })
    .expect("open avatar gallery");
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
    use super::{Avatar, AvatarBadge, AvatarImage, AvatarSize};

    #[test]
    fn builders_store_content() {
        let avatar = Avatar::new()
            .image(AvatarImage::Photo(4))
            .initials("PP")
            .size(AvatarSize::Lg)
            .badge(AvatarBadge::Icon)
            .dark(true);
        assert_eq!(avatar.image, Some(AvatarImage::Photo(4)));
        assert_eq!(avatar.initials.as_ref(), "PP");
        assert_eq!(avatar.size, AvatarSize::Lg);
        assert_eq!(avatar.badge, Some(AvatarBadge::Icon));
        assert!(avatar.dark);
    }

    #[test]
    fn sizes_map_to_pixels() {
        assert_eq!(f32::from(AvatarSize::Sm.side()), 24.);
        assert_eq!(f32::from(AvatarSize::Default.side()), 32.);
        assert_eq!(f32::from(AvatarSize::Lg.side()), 40.);
    }
}
