//! shadcn/ui-style attachment card for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/attachment
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

/// Emitted when the card's remove action is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct AttachmentRemoveEvent {
    pub name: SharedString,
}

/// Emitted when the whole card's trigger overlay is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct AttachmentOpenEvent {
    pub name: SharedString,
}

/// Media slot content: a line icon for generic files, or a painted image.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttachmentMedia {
    FileText,
    FileCode,
    Image,
    Photo,
}

/// Upload lifecycle. `Uploading` renders a progress bar, `Processing` dims the
/// title, `Error` switches to the destructive treatment.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AttachmentState {
    Idle,
    Uploading { progress: f32 },
    Processing,
    Error,
    Done,
}

/// Card density. `Xs` shows media + title only.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttachmentSize {
    Default,
    Sm,
    Xs,
}

/// `Vertical` stacks the media box above the content (image attachments).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttachmentOrientation {
    Horizontal,
    Vertical,
}

/// A styled attachment card entity. The host keeps the entity on its view and
/// renders it with `.child(self.attachment.clone())`.
pub struct Attachment {
    title: SharedString,
    description: SharedString,
    media: AttachmentMedia,
    state: AttachmentState,
    size: AttachmentSize,
    orientation: AttachmentOrientation,
    removable: bool,
    openable: bool,
    dark: bool,
}
impl EventEmitter<AttachmentRemoveEvent> for Attachment {}
impl EventEmitter<AttachmentOpenEvent> for Attachment {}

impl Attachment {
    pub fn new() -> Self {
        Self {
            title: SharedString::default(),
            description: SharedString::default(),
            media: AttachmentMedia::FileText,
            state: AttachmentState::Done,
            size: AttachmentSize::Default,
            orientation: AttachmentOrientation::Horizontal,
            removable: true,
            openable: false,
            dark: false,
        }
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }
    pub fn media(mut self, media: AttachmentMedia) -> Self {
        self.media = media;
        self
    }
    pub fn state(mut self, state: AttachmentState) -> Self {
        self.state = state;
        self
    }
    pub fn size(mut self, size: AttachmentSize) -> Self {
        self.size = size;
        self
    }
    /// Stacks the media box above the content, for image attachments.
    pub fn vertical(mut self) -> Self {
        self.orientation = AttachmentOrientation::Vertical;
        self
    }
    /// Shows the × remove action (default on).
    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }
    /// Adds a full-card trigger behind the actions; pressing it emits
    /// [`AttachmentOpenEvent`]. The remove action stays independently
    /// clickable, like shadcn's `AttachmentTrigger`.
    pub fn openable(mut self, openable: bool) -> Self {
        self.openable = openable;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

struct Palette {
    surface: Hsla,
    border: Hsla,
    foreground: Hsla,
    muted: Hsla,
    primary: Hsla,
    destructive: Hsla,
    media_bg: Hsla,
}

fn palette(dark: bool) -> Palette {
    Palette {
        surface: rgb(if dark { 0x232323 } else { 0xffffff }).into(),
        border: rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(),
        foreground: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        primary: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        destructive: rgb(if dark { 0xef8473 } else { 0xc9372c }).into(),
        media_bg: rgb(if dark { 0x2c2c2c } else { 0xf5f5f5 }).into(),
    }
}

/// Paints one of the file-type line icons at the given size.
fn file_icon(kind: AttachmentMedia, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let origin = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.; // icon art is authored on a 16 × 16 grid
            let pxk = |v: f32| px(v * k);
            let mut builder = PathBuilder::stroke(px(1.3 * k.max(0.8)));
            let at = |x: f32, y: f32| origin + point(pxk(x), pxk(y));
            match kind {
                AttachmentMedia::FileText | AttachmentMedia::Photo => {
                    // Document outline with a folded corner.
                    polyline(
                        &mut builder,
                        &[
                            at(4., 1.6),
                            at(9.6, 1.6),
                            at(12.4, 4.4),
                            at(12.4, 14.4),
                            at(4., 14.4),
                            at(4., 1.6),
                        ],
                    );
                    polyline(&mut builder, &[at(9.6, 1.6), at(9.6, 4.4), at(12.4, 4.4)]);
                    stroke_line(&mut builder, at(5.6, 8.), at(10.8, 8.));
                    stroke_line(&mut builder, at(5.6, 10.8), at(10.8, 10.8));
                }
                AttachmentMedia::FileCode => {
                    polyline(
                        &mut builder,
                        &[
                            at(4., 1.6),
                            at(9.6, 1.6),
                            at(12.4, 4.4),
                            at(12.4, 14.4),
                            at(4., 14.4),
                            at(4., 1.6),
                        ],
                    );
                    polyline(&mut builder, &[at(9.6, 1.6), at(9.6, 4.4), at(12.4, 4.4)]);
                    // </> glyph.
                    polyline(&mut builder, &[at(7.8, 8.2), at(6., 9.6), at(7.8, 11.)]);
                    polyline(&mut builder, &[at(9.4, 8.2), at(11.2, 9.6), at(9.4, 11.)]);
                }
                AttachmentMedia::Image => {
                    // Frame + sun + mountain, like lucide image.
                    rounded_rect(
                        &mut builder,
                        origin + point(pxk(2.), pxk(2.)),
                        pxk(12.),
                        pxk(12.),
                        pxk(2.),
                    );
                    circle_at(&mut builder, at(6., 6.), pxk(1.4));
                    polyline(
                        &mut builder,
                        &[
                            at(2.6, 12.6),
                            at(6.4, 8.8),
                            at(9.6, 11.6),
                            at(11.4, 9.8),
                            at(13.8, 12.6),
                        ],
                    );
                }
            }
            if let Ok(path) = builder.build() {
                window.paint_path(path, color);
            }
        },
    )
}

/// Painted thumbnail for `AttachmentMedia::Photo`.
fn photo_tile(dark: bool) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let origin = bounds.origin;
            let w = bounds.size.width;
            let h = bounds.size.height;
            let fill = |points: &[(f32, f32)], color: u32, window: &mut Window| {
                let mut b = PathBuilder::fill();
                for (i, (x, y)) in points.iter().enumerate() {
                    let v = origin + point(w * *x / 100., h * *y / 100.);
                    if i == 0 {
                        b.move_to(v);
                    } else {
                        b.line_to(v);
                    }
                }
                if let Ok(path) = b.build() {
                    window.paint_path(path, rgb(color));
                }
            };
            fill(
                &[(0., 0.), (100., 0.), (100., 100.), (0., 100.)],
                if dark { 0x1e2a38 } else { 0xcfe0ee },
                window,
            );
            circle_fill(
                window,
                origin + point(w * 0.72, h * 0.3),
                w.min(h) * 0.12,
                rgb(if dark { 0xe8c468 } else { 0xf5cf6e }).into(),
            );
            fill(
                &[
                    (0., 100.),
                    (20., 52.),
                    (42., 80.),
                    (62., 48.),
                    (100., 88.),
                    (100., 100.),
                ],
                if dark { 0x33475c } else { 0x9db8cf },
                window,
            );
            fill(
                &[(0., 100.), (32., 60.), (58., 100.)],
                if dark { 0x243649 } else { 0x6f92ad },
                window,
            );
        },
    )
    .size_full()
}

fn circle_fill(window: &mut Window, center: Point<Pixels>, radius: Pixels, color: Hsla) {
    let mut b = PathBuilder::fill();
    b.move_to(center + point(radius, px(0.)));
    b.arc_to(
        point(radius, radius),
        px(0.),
        true,
        true,
        center + point(-radius, px(0.)),
    );
    b.arc_to(
        point(radius, radius),
        px(0.),
        true,
        true,
        center + point(radius, px(0.)),
    );
    if let Ok(path) = b.build() {
        window.paint_path(path, color);
    }
}

fn circle_at(builder: &mut PathBuilder, center: Point<Pixels>, radius: Pixels) {
    builder.move_to(center + point(radius, px(0.)));
    builder.arc_to(
        point(radius, radius),
        px(0.),
        true,
        true,
        center + point(-radius, px(0.)),
    );
    builder.arc_to(
        point(radius, radius),
        px(0.),
        true,
        true,
        center + point(radius, px(0.)),
    );
}

fn stroke_line(builder: &mut PathBuilder, a: Point<Pixels>, b: Point<Pixels>) {
    builder.move_to(a);
    builder.line_to(b);
}

fn polyline(builder: &mut PathBuilder, points: &[Point<Pixels>]) {
    for (index, vertex) in points.iter().enumerate() {
        if index == 0 {
            builder.move_to(*vertex);
        } else {
            builder.line_to(*vertex);
        }
    }
}

fn rounded_rect(builder: &mut PathBuilder, origin: Point<Pixels>, w: Pixels, h: Pixels, r: Pixels) {
    builder.move_to(origin + point(r, px(0.)));
    builder.line_to(origin + point(w - r, px(0.)));
    builder.arc_to(point(r, r), px(0.), true, true, origin + point(w, r));
    builder.line_to(origin + point(w, h - r));
    builder.arc_to(point(r, r), px(0.), true, true, origin + point(w - r, h));
    builder.line_to(origin + point(r, h));
    builder.arc_to(
        point(r, r),
        px(0.),
        true,
        true,
        origin + point(px(0.), h - r),
    );
    builder.line_to(origin + point(px(0.), r));
    builder.arc_to(point(r, r), px(0.), true, true, origin + point(r, px(0.)));
}

/// × glyph used by the remove action.
fn x_icon(color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let mut b = PathBuilder::stroke(px(1.3));
            b.move_to(o + point(px(4.), px(4.)));
            b.line_to(o + point(px(12.), px(12.)));
            b.move_to(o + point(px(12.), px(4.)));
            b.line_to(o + point(px(4.), px(12.)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

impl Render for Attachment {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = palette(self.dark);
        let compact = self.size == AttachmentSize::Xs;
        let vertical = self.orientation == AttachmentOrientation::Vertical;
        let error = self.state == AttachmentState::Error;
        let processing = self.state == AttachmentState::Processing;

        let media_side = match self.size {
            AttachmentSize::Default => px(40.),
            AttachmentSize::Sm => px(32.),
            AttachmentSize::Xs => px(24.),
        };
        let icon_side = media_side * 0.45;
        let title_color = if error {
            colors.destructive
        } else if processing {
            colors.muted
        } else {
            colors.foreground
        };

        // Media box.
        let mut media = div()
            .flex_none()
            .rounded_md()
            .border_1()
            .border_color(colors.border)
            .bg(colors.media_bg)
            .overflow_hidden();
        if vertical {
            media = media.w_full().h(px(72.));
        } else {
            media = media.w(media_side).h(media_side);
        }
        if self.media == AttachmentMedia::Photo {
            media = media.child(photo_tile(self.dark));
        } else {
            media =
                media
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().size(icon_side).child(file_icon(
                        self.media,
                        if error {
                            colors.destructive
                        } else {
                            colors.muted
                        },
                    )));
        }

        // Content: title + description (+ progress while uploading).
        let mut content = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(px(2.))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(title_color)
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(self.title.clone()),
            );
        if !compact {
            content = content.child(
                div()
                    .text_xs()
                    .text_color(if error {
                        colors.destructive
                    } else {
                        colors.muted
                    })
                    .overflow_hidden()
                    .whitespace_nowrap()
                    .child(self.description.clone()),
            );
        }
        if let AttachmentState::Uploading { progress } = self.state {
            content = content.child(
                div()
                    .mt(px(2.))
                    .h(px(3.))
                    .w_full()
                    .rounded_full()
                    .bg(colors.border)
                    .child(
                        div()
                            .h_full()
                            .w(relative(progress.clamp(0., 1.)))
                            .rounded_full()
                            .bg(colors.primary),
                    ),
            );
        }

        // Remove action (ghost icon button, independently clickable/focusable).
        let mut action = div();
        if self.removable {
            let focus = window
                .use_keyed_state(self.title.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone();
            let entity = cx.entity().downgrade();
            let emitted = self.title.clone();
            action = action.child(
                div()
                    .id(SharedString::from(format!(
                        "attachment-remove-{}",
                        self.title
                    )))
                    .track_focus(&focus.tab_index(0))
                    .flex_none()
                    .size(px(24.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .cursor_pointer()
                    .text_color(colors.muted)
                    .hover(|style| {
                        style
                            .bg(colors.border.opacity(0.4))
                            .text_color(colors.foreground)
                    })
                    .focus_visible(|style| style.border_1().border_color(colors.primary))
                    .on_click(move |_, _, cx| {
                        if let Some(attachment) = entity.upgrade() {
                            let _ = attachment.update(cx, |_, cx| {
                                cx.emit(AttachmentRemoveEvent {
                                    name: emitted.clone(),
                                });
                                cx.notify();
                            });
                        }
                    })
                    .child(div().size(px(12.)).child(x_icon(colors.muted))),
            );
        }

        let mut card = div()
            .relative()
            .w_full()
            .flex()
            .rounded_lg()
            .border_1()
            .border_color(if error {
                colors.destructive.opacity(0.6)
            } else {
                colors.border
            })
            .bg(colors.surface)
            .font_family(FONT)
            .gap_3()
            .when(!vertical, |this| this.items_center())
            .when(vertical, |this| this.flex_col().gap_2())
            .map(|this| match self.size {
                AttachmentSize::Default => this.p(px(10.)),
                AttachmentSize::Sm => this.p_2(),
                AttachmentSize::Xs => this.p_1p5(),
            })
            .child(media)
            .child(content);

        // Full-card trigger. GPUI paints absolute children above in-flow
        // siblings, so the overlay stops short of the remove-action column —
        // the action stays independently clickable like shadcn's layout.
        if self.openable {
            let entity = cx.entity().downgrade();
            let emitted = self.title.clone();
            let right_edge = if self.removable { px(40.) } else { px(0.) };
            card = card.child(
                div()
                    .id(SharedString::from(format!(
                        "attachment-trigger-{}",
                        self.title
                    )))
                    .absolute()
                    .top_0()
                    .bottom_0()
                    .left_0()
                    .right(right_edge)
                    .cursor_pointer()
                    .on_click(move |_, _, cx| {
                        if let Some(attachment) = entity.upgrade() {
                            let name = emitted.clone();
                            let _ = attachment.update(cx, |_, cx| {
                                cx.emit(AttachmentOpenEvent { name });
                                cx.notify();
                            });
                        }
                    }),
            );
        }
        if vertical {
            // Pin the remove action to the card's top-right corner.
            card = card.child(div().absolute().top(px(6.)).right(px(6.)).child(action));
        } else {
            card = card.child(action);
        }
        card
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Image,
    States,
    Sizes,
    Group,
    Trigger,
}

fn demo_attachments(kind: DemoKind, dark: bool) -> Vec<Attachment> {
    let dark_spec = |a: Attachment| a.dark(dark);
    match kind {
        DemoKind::Basic => vec![
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::Image)
                    .title("workspace.png")
                    .description("PNG · 820 KB"),
            ),
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::Image)
                    .title("desk-reference.jpg")
                    .description("JPG · 1.1 MB"),
            ),
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::FileCode)
                    .title("message-renderer.tsx")
                    .description("TypeScript · 12 KB"),
            ),
        ],
        DemoKind::Image => vec![
            dark_spec(
                Attachment::new()
                    .vertical()
                    .media(AttachmentMedia::Photo)
                    .title("workspace.png")
                    .description("PNG · 820 KB"),
            ),
            dark_spec(
                Attachment::new()
                    .vertical()
                    .media(AttachmentMedia::Photo)
                    .title("office-reference.jpg")
                    .description("JPG · 940 KB"),
            ),
        ],
        DemoKind::States => vec![
            dark_spec(
                Attachment::new()
                    .state(AttachmentState::Idle)
                    .title("selected-file.pdf")
                    .description("Ready to upload"),
            ),
            dark_spec(
                Attachment::new()
                    .state(AttachmentState::Uploading { progress: 0.64 })
                    .title("design-system.zip")
                    .description("Uploading · 64%"),
            ),
            dark_spec(
                Attachment::new()
                    .state(AttachmentState::Processing)
                    .title("market-research.pdf")
                    .description("Processing document"),
            ),
            dark_spec(
                Attachment::new()
                    .state(AttachmentState::Error)
                    .title("financial-model.xlsx")
                    .description("Upload failed. Try again."),
            ),
            dark_spec(
                Attachment::new()
                    .state(AttachmentState::Done)
                    .title("uploaded-report.pdf")
                    .description("Uploaded · 1.8 MB"),
            ),
        ],
        DemoKind::Sizes => vec![
            dark_spec(
                Attachment::new()
                    .title("Default attachment")
                    .description("PDF · 2.4 MB"),
            ),
            dark_spec(
                Attachment::new()
                    .size(AttachmentSize::Sm)
                    .title("Small attachment")
                    .description("PDF · 2.4 MB"),
            ),
            dark_spec(
                Attachment::new()
                    .size(AttachmentSize::Xs)
                    .title("Extra small attachment"),
            ),
        ],
        DemoKind::Group => vec![
            dark_spec(
                Attachment::new()
                    .title("briefing-notes.pdf")
                    .description("PDF · 1.4 MB"),
            ),
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::Image)
                    .title("workspace.png")
                    .description("PNG · 820 KB"),
            ),
            dark_spec(
                Attachment::new()
                    .title("customers.csv")
                    .description("CSV · 18 KB"),
            ),
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::FileCode)
                    .title("renderer.tsx")
                    .description("TSX · 12 KB"),
            ),
            dark_spec(
                Attachment::new()
                    .media(AttachmentMedia::Photo)
                    .title("office-reference.jpg")
                    .description("JPG · 940 KB"),
            ),
        ],
        DemoKind::Trigger => vec![dark_spec(
            Attachment::new()
                .title("research-summary.pdf")
                .description("PDF · 1.4 MB — click the card")
                .openable(true),
        )],
    }
}

pub struct AttachmentDemo {
    attachments: Vec<Entity<Attachment>>,
    kind: DemoKind,
    dark: bool,
    outcome: Option<SharedString>,
    scroll: ScrollHandle,
    _subscriptions: Vec<Subscription>,
}

impl AttachmentDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "image" => DemoKind::Image,
            "states" => DemoKind::States,
            "sizes" => DemoKind::Sizes,
            "group" => DemoKind::Group,
            "trigger" => DemoKind::Trigger,
            _ => DemoKind::Basic,
        };
        let mut subscriptions = Vec::new();
        let attachments = demo_attachments(kind, dark)
            .into_iter()
            .map(|attachment| {
                let entity = cx.new(|_| attachment);
                subscriptions.push(cx.subscribe(
                    &entity,
                    |demo, _, event: &AttachmentRemoveEvent, cx| {
                        demo.outcome = Some(format!("Removed {}", event.name).into());
                        cx.notify();
                    },
                ));
                subscriptions.push(cx.subscribe(
                    &entity,
                    |demo, _, event: &AttachmentOpenEvent, cx| {
                        demo.outcome = Some(format!("Opened {}", event.name).into());
                        cx.notify();
                    },
                ));
                entity
            })
            .collect();
        Self {
            attachments,
            kind,
            dark,
            outcome: None,
            scroll: ScrollHandle::new(),
            _subscriptions: subscriptions,
        }
    }
}

impl Render for AttachmentDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });

        let mut list = div().w(px(460.)).flex().flex_col().gap_2();
        match self.kind {
            DemoKind::Image => {
                list = list.child(
                    div().flex().gap_3().children(
                        self.attachments
                            .iter()
                            .cloned()
                            .map(|a| div().flex_1().min_w_0().child(a)),
                    ),
                );
            }
            DemoKind::Group => {
                list = list.child(
                    div()
                        .id("attachment-group")
                        .flex()
                        .gap_3()
                        .overflow_x_scroll()
                        .track_scroll(&self.scroll)
                        .pb_2()
                        .children(
                            self.attachments
                                .iter()
                                .cloned()
                                .map(|a| div().flex_none().w(px(220.)).child(a)),
                        ),
                );
            }
            _ => {
                for attachment in &self.attachments {
                    list = list.child(attachment.clone());
                }
            }
        }

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
                    .child(list)
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(560.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| AttachmentDemo::new(&variant, dark, cx))
    })
    .expect("open attachment gallery");
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
    use super::{
        Attachment, AttachmentMedia, AttachmentOrientation, AttachmentSize, AttachmentState,
    };

    #[test]
    fn builders_store_content() {
        let attachment = Attachment::new()
            .title("report.pdf")
            .description("PDF · 2.4 MB")
            .media(AttachmentMedia::FileCode)
            .state(AttachmentState::Uploading { progress: 0.4 })
            .size(AttachmentSize::Sm)
            .vertical()
            .openable(true);
        assert_eq!(attachment.title.as_ref(), "report.pdf");
        assert_eq!(
            attachment.state,
            AttachmentState::Uploading { progress: 0.4 }
        );
        assert_eq!(attachment.size, AttachmentSize::Sm);
        assert_eq!(attachment.orientation, AttachmentOrientation::Vertical);
        assert!(attachment.openable);
    }

    #[test]
    fn progress_clamps_for_uploading() {
        let clamped = (1.4f32).clamp(0., 1.);
        assert_eq!(clamped, 1.);
    }
}
