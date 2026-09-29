//! shadcn/ui-style chat bubble for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/bubble
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

/// Emitted when a pressable (`render`-style button) bubble is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct BubblePressEvent {
    pub label: SharedString,
}

/// Visual treatment, mirroring shadcn's `variant` prop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BubbleVariant {
    /// Strong primary bubble, usually for the current user.
    Default,
    /// Standard neutral bubble.
    Secondary,
    /// Lower-emphasis bubble for quiet supporting content.
    Muted,
    /// Subtle primary-tinted bubble.
    Tinted,
    /// Bordered bubble for secondary or rich content.
    Outline,
    /// Unframed full-width content for assistant text.
    Ghost,
    /// Destructive bubble for error or failed actions.
    Destructive,
}

/// Which side of the conversation the bubble hugs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BubbleAlign {
    Start,
    End,
}

/// A chat bubble entity. Keep it on the host view and render with
/// `.child(self.bubble.clone())`.
pub struct Bubble {
    text: SharedString,
    variant: BubbleVariant,
    align: BubbleAlign,
    reactions: Vec<SharedString>,
    reactions_side: BubbleAlign,
    pressable: bool,
    /// Long content collapses to `collapsed_lines` worth of preview
    /// text with a Show more/less toggle.
    collapsible: bool,
    expanded: bool,
    preview: Option<SharedString>,
    dark: bool,
}
impl EventEmitter<BubblePressEvent> for Bubble {}

impl Bubble {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            variant: BubbleVariant::Secondary,
            align: BubbleAlign::Start,
            reactions: Vec::new(),
            reactions_side: BubbleAlign::Start,
            pressable: false,
            collapsible: false,
            expanded: false,
            preview: None,
            dark: false,
        }
    }
    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }
    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }
    /// Reaction chips anchored on the bubble's bottom edge.
    pub fn reactions(mut self, chips: impl Into<Vec<SharedString>>, side: BubbleAlign) -> Self {
        self.reactions = chips.into();
        self.reactions_side = side;
        self
    }
    /// Button-style bubble: focusable, pressable, emits
    /// [`BubblePressEvent`].
    pub fn pressable(mut self, pressable: bool) -> Self {
        self.pressable = pressable;
        self
    }
    /// Long text: starts collapsed to `preview`, a Show more/less
    /// control toggles the full content.
    pub fn collapsible(mut self, preview: impl Into<SharedString>) -> Self {
        self.collapsible = true;
        self.preview = Some(preview.into());
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

fn bubble_colors(variant: BubbleVariant, dark: bool) -> (Hsla, Hsla, Hsla) {
    let (bg, ink, edge) = match (variant, dark) {
        (BubbleVariant::Default, false) => (0x171717, 0xffffff, 0x171717),
        (BubbleVariant::Default, true) => (0xededed, 0x171717, 0xededed),
        (BubbleVariant::Secondary, false) => (0xececec, 0x171717, 0xececec),
        (BubbleVariant::Secondary, true) => (0x333333, 0xededed, 0x333333),
        (BubbleVariant::Muted, false) => (0xf4f4f4, 0x525252, 0xf4f4f4),
        (BubbleVariant::Muted, true) => (0x292929, 0xa3a3a3, 0x292929),
        (BubbleVariant::Tinted, false) => (0xdfe7ff, 0x1d4ed8, 0xdfe7ff),
        (BubbleVariant::Tinted, true) => (0x1e2a4a, 0x93b4ff, 0x1e2a4a),
        (BubbleVariant::Outline, false) => (0xffffff, 0x171717, 0xe4e4e4),
        (BubbleVariant::Outline, true) => (0x232323, 0xededed, 0x3b3b3b),
        (BubbleVariant::Ghost, false) => (0xffffff, 0x404040, 0xffffff),
        (BubbleVariant::Ghost, true) => (0x202020, 0xd4d4d4, 0x202020),
        (BubbleVariant::Destructive, false) => (0xffe6e9, 0xff0000, 0xffe6e9),
        (BubbleVariant::Destructive, true) => (0x402126, 0xff858d, 0x402126),
    };
    (rgb(bg).into(), rgb(ink).into(), rgb(edge).into())
}

fn reaction_ink_center(line: &ShapedLine, window: &Window) -> Pixels {
    let mut extent: Option<(Pixels, Pixels)> = None;
    for run in &line.runs {
        for glyph in &run.glyphs {
            let Some(character) = line.text[glyph.index..].chars().next() else {
                continue;
            };
            let Ok(bounds) =
                window
                    .text_system()
                    .typographic_bounds(run.font_id, line.font_size, character)
            else {
                return line.width() / 2.;
            };
            if bounds.size.width <= px(0.) {
                continue;
            }
            let left = glyph.position.x + bounds.origin.x;
            let right = left + bounds.size.width;
            extent = Some(match extent {
                Some((min, max)) => (min.min(left), max.max(right)),
                None => (left, right),
            });
        }
    }
    extent.map_or(line.width() / 2., |(left, right)| (left + right) / 2.)
}

// Reactions are pictograms: center their visible ink, not the font's advance
// box (which includes side bearings). Apply the same measurement to counters.
fn reaction_label(text: SharedString, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, window, _| {
            window.text_system().shape_line(
                text.clone(),
                px(14.),
                &[TextRun {
                    len: text.len(),
                    font: Font {
                        family: FONT.into(),
                        ..Default::default()
                    },
                    color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                }],
                None,
            )
        },
        move |bounds, line, window, cx| {
            let origin = point(
                bounds.origin.x + bounds.size.width / 2. - reaction_ink_center(&line, window),
                bounds.origin.y,
            );
            let _ = line.paint(
                origin,
                bounds.size.height,
                TextAlign::Left,
                None,
                window,
                cx,
            );
        },
    )
    .w(px(18.))
    .h(px(24.))
    .flex_none()
}

impl Render for Bubble {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let (surface, ink, edge) = bubble_colors(self.variant, dark);
        let ghost = self.variant == BubbleVariant::Ghost;
        // Reactions mirror shadcn: `bg-muted` pill with a `ring-3 ring-card`
        // halo — the card-colored ring is what carves the notch, no shadow.
        let page: Hsla = rgb(if dark { 0x202020 } else { 0xffffff }).into();
        let chip_bg: Hsla = rgb(if dark { 0x2a2a2a } else { 0xf5f5f5 }).into();
        let chip_ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let toggle_ink: Hsla = rgb(if dark { 0x93c5fd } else { 0x2563eb }).into();

        let mut frame = div()
            .w_full()
            .flex()
            .flex_col()
            .items_start()
            .when(self.align == BubbleAlign::End, |style| style.items_end());

        // The rounded surface and reaction overlay are siblings. Keeping the
        // overlay outside the painted surface lets its halo cross the edge.
        let styled = || {
            div()
                .when(!ghost, |style| {
                    style
                        .rounded(px(24.))
                        .px(px(12.))
                        .py(px(10.))
                        .bg(surface)
                        .border_1()
                        .border_color(edge)
                })
                .when(ghost, |style| style.py(px(2.)))
                .relative()
                .font_family(FONT)
                .text_sm()
                .text_color(ink)
        };

        let shown = if self.collapsible && !self.expanded {
            self.preview.clone().unwrap_or_else(|| self.text.clone())
        } else {
            self.text.clone()
        };
        let mut content = div().child(shown);

        if self.collapsible {
            let entity = cx.entity().downgrade();
            let toggle = div()
                .id("bubble-toggle")
                .mt_1()
                .flex_none()
                .flex()
                .items_center()
                .font_family(FONT)
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(toggle_ink)
                .cursor_pointer()
                .on_click(move |_, _, cx| {
                    if let Some(bubble) = entity.upgrade() {
                        let _ = bubble.update(cx, |bubble, cx| {
                            bubble.expanded = !bubble.expanded;
                            cx.notify();
                        });
                    }
                })
                .child(if self.expanded {
                    "Show less"
                } else {
                    "Show more"
                });
            content = content.child(toggle);
        }

        // The unpainted wrapper positions the chips against the bubble edge.
        let chips = if self.reactions.is_empty() {
            None
        } else {
            // `ring-3 ring-card`: 3px page-colored halo around a muted pill,
            // sunk 3/4 of its height below the bubble edge.
            let single = self.reactions.len() == 1;
            let mut pill = div()
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(9999.))
                .h(px(24.))
                .when(single, |style| style.w(px(24.)))
                .when(!single, |style| style.px(px(4.)).gap(px(2.)))
                .bg(chip_bg);
            for chip in &self.reactions {
                pill = pill.child(reaction_label(chip.clone(), chip_ink));
            }
            let mut row = div()
                .absolute()
                .bottom(px(-20.))
                .rounded(px(9999.))
                .p(px(3.))
                .bg(page)
                .child(pill);
            row = match self.reactions_side {
                // Hug the corner like shadcn's `left-3`/`right-3`: the ring
                // bites through the corner curve, carving the socket.
                BubbleAlign::Start => row.left(px(10.)),
                BubbleAlign::End => row.right(px(10.)),
            };
            Some(row)
        };

        let body: AnyElement = if self.pressable {
            let focus = window
                .use_keyed_state(format!("bubble-{}", self.text), cx, |_, cx| {
                    cx.focus_handle()
                })
                .read(cx)
                .clone();
            let entity = cx.entity().downgrade();
            let text = self.text.clone();
            styled()
                .id(format!("bubble-{}", self.text))
                .track_focus(&focus.tab_index(0))
                .cursor_pointer()
                .focus_visible(|style| style.border_color(ink))
                .on_click(move |_, _, cx| {
                    if let Some(bubble) = entity.upgrade() {
                        let label = text.clone();
                        let _ = bubble.update(cx, |_, cx| {
                            cx.emit(BubblePressEvent { label });
                            cx.notify();
                        });
                    }
                })
                .child(content)
                .into_any_element()
        } else {
            styled().child(content).into_any_element()
        };
        let anchored = div()
            .relative()
            .when(!ghost, |style| style.max_w(relative(0.8)))
            .when(ghost, |style| style.w_full())
            .child(body)
            .children(chips);
        // Reserve the complete overhang before laying out the next bubble.
        frame = frame
            .child(anchored)
            .when(!self.reactions.is_empty(), |f| f.mb_5());
        frame
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Variants,
    Alignment,
    Group,
    Reactions,
    Buttons,
    Collapsible,
}

pub struct BubbleDemo {
    bubbles: Vec<Entity<Bubble>>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl BubbleDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "alignment" => DemoKind::Alignment,
            "group" => DemoKind::Group,
            "reactions" => DemoKind::Reactions,
            "buttons" => DemoKind::Buttons,
            "collapsible" => DemoKind::Collapsible,
            _ => DemoKind::Variants,
        };
        let d = |b: Bubble| b.dark(dark);
        let specs: Vec<Bubble> = match kind {
            DemoKind::Variants => vec![
                d(Bubble::new("This is the default primary bubble.").variant(BubbleVariant::Default)),
                d(Bubble::new("This is the secondary variant.")),
                d(Bubble::new("This one is muted — lower emphasis.").variant(BubbleVariant::Muted)),
                d(Bubble::new("This one is tinted with primary color.").variant(BubbleVariant::Tinted)),
                d(Bubble::new("An outlined variant.").variant(BubbleVariant::Outline)),
                d(Bubble::new("Or a destructive variant with a reaction.")
                    .variant(BubbleVariant::Destructive)
                    .reactions(vec!["🔥".into()], BubbleAlign::End)),
                d(Bubble::new("Ghost bubbles work for assistant text and rich content — unframed, up to the full row width.")
                    .variant(BubbleVariant::Ghost)),
            ],
            DemoKind::Alignment => vec![
                d(Bubble::new("This bubble is aligned to the start.")),
                d(Bubble::new("Aligned to the end — user messages.")
                    .variant(BubbleVariant::Default)
                    .align(BubbleAlign::End)),
                d(Bubble::new("Back to the start again.")),
            ],
            DemoKind::Group => vec![
                d(Bubble::new("Can you tell me what's the issue?")),
                d(Bubble::new("You tell me!")),
                d(Bubble::new("It worked yesterday. You broke it!")),
                d(Bubble::new("Find the bug and fix it.")
                    .reactions(vec!["👀".into()], BubbleAlign::Start)),
                d(Bubble::new("Want me to diff yesterday's you against today's you?")
                    .variant(BubbleVariant::Default)
                    .align(BubbleAlign::End)),
            ],
            DemoKind::Reactions => vec![
                d(Bubble::new("I don't need tests, I know my code works.")
                    .reactions(vec!["👍".into(), "😮".into()], BubbleAlign::Start)),
                d(Bubble::new("Bold. I'll add some tests and let you know.")
                    .variant(BubbleVariant::Default)
                    .align(BubbleAlign::End)
                    .reactions(vec!["👀".into(), "🚀".into(), "+2".into()], BubbleAlign::End)),
                d(Bubble::new("Tests passed on the first try. All 142 of them.")
                    .reactions(vec!["🎉".into(), "👏".into()], BubbleAlign::Start)),
            ],
            DemoKind::Buttons => vec![
                d(Bubble::new("How can I help you today?")),
                d(Bubble::new("I forgot my password")
                    .variant(BubbleVariant::Outline)
                    .pressable(true)),
                d(Bubble::new("I need help with my subscription")
                    .variant(BubbleVariant::Outline)
                    .pressable(true)),
                d(Bubble::new("Something else. Talk to a human.")
                    .variant(BubbleVariant::Outline)
                    .pressable(true)),
            ],
            DemoKind::Collapsible => vec![
                d(Bubble::new("How can I help you today?")),
                d(Bubble::new("The accessibility review found two focus states that were visually too subtle in dark mode. I checked the dialog, menu, and drawer paths because each one renders focusable controls differently — the ring contrast, the offset, and whether the outline survives a busy gradient background. The fix keeps the 2px ring but swaps the ring color to the accent token and raises the offset so the halo stays visible on both themes.")
                    .variant(BubbleVariant::Secondary)
                    .collapsible("The accessibility review found two focus states that were visually too subtle in dark mode. I checked the dialog, menu, and drawer paths…")),
            ],
        };

        let mut subscriptions = Vec::new();
        let bubbles = specs
            .into_iter()
            .map(|spec| {
                let entity = cx.new(|_| spec);
                subscriptions.push(cx.subscribe(
                    &entity,
                    |demo, _, event: &BubblePressEvent, cx| {
                        demo.outcome = Some(format!("Clicked \"{}\"", event.label).into());
                        cx.notify();
                    },
                ));
                entity
            })
            .collect();
        Self {
            bubbles,
            dark,
            outcome: None,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for BubbleDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xffffff });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(
                div()
                    .w_full()
                    .max_w(px(520.))
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap_2()
                    .children(self.bubbles.iter().cloned().map(IntoElement::into_element)),
            )
            .child(
                div()
                    .mt_2()
                    .h(px(18.))
                    .text_xs()
                    .text_color(muted)
                    .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(460.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| BubbleDemo::new(&variant, dark, cx))
    })
    .expect("open bubble gallery");
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
    cx.new(|cx| BubbleDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{Bubble, BubbleAlign, BubbleVariant, SharedString};

    #[test]
    fn builders_store_content() {
        let bubble = Bubble::new("hello")
            .variant(BubbleVariant::Tinted)
            .align(BubbleAlign::End)
            .reactions(vec!["👍".into()], BubbleAlign::End)
            .pressable(true)
            .collapsible("hi")
            .dark(true);
        assert_eq!(bubble.text.as_ref(), "hello");
        assert_eq!(bubble.variant, BubbleVariant::Tinted);
        assert_eq!(bubble.align, BubbleAlign::End);
        assert_eq!(bubble.reactions, vec![SharedString::from("👍")]);
        assert_eq!(bubble.reactions_side, BubbleAlign::End);
        assert!(bubble.pressable && bubble.collapsible && bubble.dark);
        assert!(!bubble.expanded);
        assert_eq!(bubble.preview.as_deref(), Some("hi"));
    }
}
