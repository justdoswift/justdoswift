//! shadcn/ui-style card for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/card
use crate::button::{Button, ButtonIcon, ButtonSize, ButtonVariant, IconPos};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";
/// Default `--card-spacing` (shadcn's 24px inset/gap).
pub const CARD_SPACING: f32 = 24.;
/// Small-size `--card-spacing` (16px).
pub const CARD_SPACING_SM: f32 = 16.;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CardSize {
    Default,
    Sm,
}

/// Root card container — vertical stack with `gap = spacing` and
/// `padding-y = spacing`; sections inset themselves horizontally.
/// `size(Sm)` switches the spacing scale, `.spacing(..)` overrides it
/// (shadcn's `--card-spacing` variable).
#[derive(IntoElement)]
pub struct Card {
    spacing: f32,
    dark: bool,
    width: f32,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            spacing: CARD_SPACING,
            dark: false,
            width: 384.,
            children: Vec::new(),
        }
    }
    pub fn size(mut self, size: CardSize) -> Self {
        self.spacing = match size {
            CardSize::Default => CARD_SPACING,
            CardSize::Sm => CARD_SPACING_SM,
        };
        self
    }
    /// Explicit `--card-spacing` override (takes precedence over `.size`).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn w(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    /// Append any element (header/content/footer helpers or free content
    /// like `card_cover`). Children render in call order.
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.children.push(el.into_any_element());
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let surface: Hsla = rgb(if self.dark { 0x232323 } else { 0xffffff }).into();
        let edge: Hsla = rgb(if self.dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
        let sp = self.spacing;
        div()
            .flex_none()
            .flex()
            .flex_col()
            .w(px(self.width))
            .gap(px(sp))
            .py(px(sp))
            .bg(surface)
            .border_1()
            .border_color(edge)
            .rounded_xl()
            .shadow_sm()
            .overflow_hidden()
            .font_family(FONT)
            .children(self.children)
    }
}

/// Spec collected inside `card_header`'s closure.
#[derive(Default)]
pub struct CardHeaderSpec {
    title: Option<SharedString>,
    description: Option<SharedString>,
    action: Option<AnyElement>,
    children: Vec<AnyElement>,
    bordered: bool,
}

impl CardHeaderSpec {
    pub fn title(&mut self, title: impl Into<SharedString>) -> &mut Self {
        self.title = Some(title.into());
        self
    }
    pub fn description(&mut self, description: impl Into<SharedString>) -> &mut Self {
        self.description = Some(description.into());
        self
    }
    /// Top-right slot (shadcn's CardAction — button, badge, icon…).
    pub fn action(&mut self, el: impl IntoElement) -> &mut Self {
        self.action = Some(el.into_any_element());
        self
    }
    /// Extra row below the title/description block.
    pub fn child(&mut self, el: impl IntoElement) -> &mut Self {
        self.children.push(el.into_any_element());
        self
    }
    /// Bottom border + padding (shadcn's `[.border-b]:pb-6`).
    pub fn bordered(&mut self, bordered: bool) -> &mut Self {
        self.bordered = bordered;
        self
    }
}

/// Header section: title + description column, optional top-right action.
pub fn card_header(
    spacing: f32,
    dark: bool,
    build: impl FnOnce(&mut CardHeaderSpec),
) -> AnyElement {
    let mut spec = CardHeaderSpec::default();
    build(&mut spec);
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let sp = px(spacing);

    let mut text_col = div().flex_1().min_w_0().flex().flex_col().gap(px(6.));
    if let Some(title) = spec.title {
        text_col = text_col.child(
            div()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(ink)
                .whitespace_normal()
                .child(title),
        );
    }
    if let Some(description) = spec.description {
        text_col = text_col.child(
            div()
                .text_sm()
                .text_color(muted)
                .whitespace_normal()
                .child(description),
        );
    }

    let header = div()
        .flex()
        .flex_col()
        .px(sp)
        .when(spec.bordered, |s| s.pb(sp).border_b_1().border_color(edge));
    let mut row = div().flex().flex_row().items_start().w_full();
    if spec.action.is_some() {
        row = row
            .child(text_col)
            .child(spec.action.unwrap().into_any_element());
    } else {
        row = row.child(text_col);
    }
    header.child(row).children(spec.children).into_any_element()
}

/// Spec collected inside `card_content`'s closure.
#[derive(Default)]
pub struct CardContentSpec {
    children: Vec<AnyElement>,
    bleed: bool,
}

impl CardContentSpec {
    pub fn child(&mut self, el: impl IntoElement) -> &mut Self {
        self.children.push(el.into_any_element());
        self
    }
    /// Edge-to-edge content (`-mx-(--card-spacing)` in shadcn).
    pub fn bleed(&mut self, bleed: bool) -> &mut Self {
        self.bleed = bleed;
        self
    }
}

/// Main body section.
pub fn card_content(
    spacing: f32,
    dark: bool,
    build: impl FnOnce(&mut CardContentSpec),
) -> AnyElement {
    let mut spec = CardContentSpec::default();
    build(&mut spec);
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    let sp = px(spacing);
    let mut el = div()
        .flex()
        .flex_col()
        .gap(px(8.))
        .text_sm()
        .text_color(ink)
        .font_family(FONT)
        .children(spec.children);
    if spec.bleed {
        el = el.mx(-sp);
    } else {
        el = el.px(sp);
    }
    el.into_any_element()
}

/// Spec collected inside `card_footer`'s closure.
#[derive(Default)]
pub struct CardFooterSpec {
    children: Vec<AnyElement>,
    bordered: bool,
    column: bool,
    justify_end: bool,
}

impl CardFooterSpec {
    pub fn child(&mut self, el: impl IntoElement) -> &mut Self {
        self.children.push(el.into_any_element());
        self
    }
    /// Top border + padding (shadcn's `[.border-t]:pt-6`).
    pub fn bordered(&mut self, bordered: bool) -> &mut Self {
        self.bordered = bordered;
        self
    }
    /// Stack children vertically instead of a row.
    pub fn column(&mut self, column: bool) -> &mut Self {
        self.column = column;
        self
    }
    /// Push children to the trailing edge.
    pub fn justify_end(&mut self, end: bool) -> &mut Self {
        self.justify_end = end;
        self
    }
}

/// Footer section: action row (or column) at the bottom.
pub fn card_footer(
    spacing: f32,
    dark: bool,
    build: impl FnOnce(&mut CardFooterSpec),
) -> AnyElement {
    let mut spec = CardFooterSpec::default();
    build(&mut spec);
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let sp = px(spacing);
    div()
        .flex()
        .w_full()
        .px(sp)
        .gap(px(8.))
        .when(spec.column, |s| s.flex_col())
        .when(!spec.column, |s| {
            s.flex_row()
                .items_center()
                .when(spec.justify_end, |s| s.justify_end())
        })
        .when(spec.bordered, |s| s.pt(sp).border_t_1().border_color(edge))
        .children(spec.children)
        .into_any_element()
}

/// Edge-to-edge media child that sits flush against the card's top —
/// wraps a full-width element and eats the card's top padding
/// (`margin-top: -spacing`). Place it as the first `child`.
pub fn card_cover(el: impl IntoElement, spacing: f32) -> AnyElement {
    div()
        .mt(px(-spacing))
        .w_full()
        // Clip media to the card's rounded top corners — content masks are
        // rectangular, so the child element must carry the radii itself.
        .rounded_tl(px(12.))
        .rounded_tr(px(12.))
        .overflow_hidden()
        .child(el.into_any_element())
        .into_any_element()
}

// ── Demo gallery ──────────────────────────────────────────────────────────

fn demo_label(text: &str, dark: bool) -> AnyElement {
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    div()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(ink)
        .child(text.to_string())
        .into_any_element()
}

fn demo_input(placeholder: &str, dark: bool) -> AnyElement {
    let ink: Hsla = rgb(if dark { 0x737373 } else { 0xa3a3a3 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let surface: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();
    div()
        .flex()
        .items_center()
        .h(px(36.))
        .w_full()
        .px_3()
        .rounded_md()
        .bg(surface)
        .border_1()
        .border_color(edge)
        .text_sm()
        .text_color(ink)
        .child(placeholder.to_string())
        .into_any_element()
}

fn demo_link(text: &str, dark: bool, on: &'static str, weak: WeakEntity<CardDemo>) -> AnyElement {
    Button::new(SharedString::from(format!("link-{text}")))
        .label(text)
        .variant(ButtonVariant::Link)
        .size(ButtonSize::Xs)
        .dark(dark)
        .on_click(move |_, _, cx| {
            if let Some(demo) = weak.upgrade() {
                let _ = demo.update(cx, |this, cx| {
                    this.outcome = Some(format!("Clicked {on}").into());
                    cx.notify();
                });
            }
        })
        .into_any_element()
}

fn demo_button(
    id: &str,
    label: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    dark: bool,
    full: bool,
    weak: WeakEntity<CardDemo>,
) -> AnyElement {
    let weak = weak.clone();
    let label_owned = label.to_string();
    let b = Button::new(SharedString::from(id.to_string()))
        .label(label)
        .variant(variant)
        .size(size)
        .dark(dark)
        .on_click(move |_, _, cx| {
            if let Some(demo) = weak.upgrade() {
                let _ = demo.update(cx, |this, cx| {
                    this.outcome = Some(format!("Clicked {label_owned}").into());
                    cx.notify();
                });
            }
        });
    if full {
        div().w_full().child(b).into_any_element()
    } else {
        b.into_any_element()
    }
}

fn login_card(dark: bool, spacing: f32, weak: &WeakEntity<CardDemo>) -> Card {
    Card::new()
        .spacing(spacing)
        .dark(dark)
        .child(card_header(spacing, dark, |h| {
            h.title("Login to your account")
                .description("Enter your email below to login to your account")
                .action(
                    Button::new("signup")
                        .label("Sign Up")
                        .variant(ButtonVariant::Link)
                        .size(ButtonSize::Xs)
                        .dark(dark)
                        .on_click({
                            let weak = weak.clone();
                            move |_, _, cx| {
                                if let Some(demo) = weak.upgrade() {
                                    let _ = demo.update(cx, |this, cx| {
                                        this.outcome = Some("Clicked Sign Up".into());
                                        cx.notify();
                                    });
                                }
                            }
                        }),
                );
        }))
        .child(card_content(spacing, dark, |c| {
            c.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(demo_label("Email", dark))
                    .child(demo_input("m@example.com", dark)),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .w_full()
                            .child(demo_label("Password", dark))
                            .child(demo_link(
                                "Forgot your password?",
                                dark,
                                "Forgot your password?",
                                weak.clone(),
                            )),
                    )
                    .child(demo_input("••••••••", dark)),
            );
        }))
        .child(card_footer(spacing, dark, |f| {
            f.column(true)
                .child(demo_button(
                    "login",
                    "Login",
                    ButtonVariant::Default,
                    ButtonSize::Default,
                    dark,
                    true,
                    weak.clone(),
                ))
                .child(demo_button(
                    "google",
                    "Login with Google",
                    ButtonVariant::Outline,
                    ButtonSize::Default,
                    dark,
                    true,
                    weak.clone(),
                ));
        }))
}

fn reports_card(dark: bool, weak: &WeakEntity<CardDemo>) -> Card {
    let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
    let bullet = |text: &str| {
        div()
            .flex()
            .flex_row()
            .gap(px(8.))
            .child(div().text_color(muted).child("•"))
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(if dark { 0xd4d4d4 } else { 0x404040 }))
                    .child(text.to_string()),
            )
            .into_any_element()
    };
    Card::new()
        .size(CardSize::Sm)
        .dark(dark)
        .child(card_header(CARD_SPACING_SM, dark, |h| {
            h.title("Scheduled reports")
                .description("Weekly snapshots. No more manual exports.")
                .action(
                    Button::new("open-report")
                        .a11y_label("Open reports")
                        .variant(ButtonVariant::Ghost)
                        .size(ButtonSize::IconXs)
                        .icon(ButtonIcon::ChevronRight, IconPos::Start)
                        .dark(dark),
                );
        }))
        .child(card_content(CARD_SPACING_SM, dark, |c| {
            c.child(bullet("Choose a schedule (daily, or weekly)."))
                .child(bullet("Send to channels or specific teammates."))
                .child(bullet("Include charts, tables, and key metrics."));
        }))
        .child(card_footer(CARD_SPACING_SM, dark, |f| {
            f.child(demo_button(
                "setup",
                "Set up scheduled reports",
                ButtonVariant::Secondary,
                ButtonSize::Sm,
                dark,
                false,
                weak.clone(),
            ))
            .child(
                Button::new("whats-new")
                    .label("See what's new")
                    .icon(ButtonIcon::ChevronRight, IconPos::End)
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Sm)
                    .dark(dark),
            );
        }))
}

fn terms_card(dark: bool, weak: &WeakEntity<CardDemo>) -> Card {
    let ink: Hsla = rgb(if dark { 0xd4d4d4 } else { 0x404040 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let para = |text: &str| {
        div()
            .text_sm()
            .text_color(ink)
            .child(text.to_string())
            .into_any_element()
    };
    let sp = 20.;
    Card::new()
        .spacing(sp)
        .dark(dark)
        .child(card_header(sp, dark, |h| {
            h.title("Terms of Service")
                .description("Review the terms before accepting the agreement.");
        }))
        .child(card_content(sp, dark, |c| {
            c.child(para("These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools."))
                // Edge-to-edge divider — bleeds through the card inset.
                .child(div().h(px(1.)).w_full().mx(px(-sp)).bg(edge))
                .child(para("You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it."))
                .child(para("By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies."));
        }))
        .child(card_footer(sp, dark, |f| {
            f.bordered(true)
                .justify_end(true)
                .child(demo_button("decline", "Decline", ButtonVariant::Outline, ButtonSize::Sm, dark, false, weak.clone()))
                .child(demo_button("accept", "Accept", ButtonVariant::Default, ButtonSize::Sm, dark, false, weak.clone()));
        }))
}

fn meetup_card(dark: bool, weak: &WeakEntity<CardDemo>) -> Card {
    let ink: Hsla = rgb(if dark { 0xd4d4d4 } else { 0x525252 }).into();
    // Painted event cover — gradient wash plus a date chip, clipped by the
    // card's rounded corners (overflow-hidden).
    let cover = div()
        .h(px(140.))
        .w_full()
        .relative()
        .rounded_tl(px(11.))
        .rounded_tr(px(11.))
        .bg(linear_gradient(
            135.,
            linear_color_stop(rgb(0x6366f1), 0.),
            linear_color_stop(rgb(0xa855f7), 1.),
        ))
        .child(
            div().absolute().top(px(12.)).left(px(12.)).child(
                div()
                    .px_2()
                    .py(px(2.))
                    .rounded_full()
                    .bg(rgb(0xffffff))
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(rgb(0x171717))
                    .child("Featured"),
            ),
        )
        .child(
            div()
                .absolute()
                .bottom(px(12.))
                .left(px(12.))
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xffffff))
                .child("MAR 24 · SAN FRANCISCO"),
        );
    Card::new()
        .dark(dark)
        .child(card_cover(cover, CARD_SPACING))
        .child(card_header(CARD_SPACING, dark, |h| {
            h.title("Design systems meetup").description(
                "A practical talk on component APIs, accessibility, and shipping faster.",
            );
        }))
        .child(card_content(CARD_SPACING, dark, |c| {
            c.child(
                div()
                    .text_sm()
                    .text_color(ink)
                    .child("Hosted by the GPUI community — doors at 18:30."),
            );
        }))
        .child(card_footer(CARD_SPACING, dark, |f| {
            f.justify_end(true).child(demo_button(
                "view-event",
                "View Event",
                ButtonVariant::Secondary,
                ButtonSize::Sm,
                dark,
                false,
                weak.clone(),
            ));
        }))
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Sm,
    Spacing,
    Image,
}

pub struct CardDemo {
    variant: String,
    dark: bool,
    outcome: Option<SharedString>,
}

impl CardDemo {
    fn new(variant: &str, dark: bool, _cx: &mut Context<Self>) -> Self {
        Self {
            variant: variant.to_string(),
            dark,
            outcome: None,
        }
    }
}

impl Render for CardDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let weak = cx.entity().downgrade();
        let kind = match self.variant.as_str() {
            "sm" => DemoKind::Sm,
            "spacing" => DemoKind::Spacing,
            "image" => DemoKind::Image,
            _ => DemoKind::Basic,
        };
        let card: AnyElement = match kind {
            DemoKind::Basic => login_card(self.dark, CARD_SPACING, &weak).into_any_element(),
            DemoKind::Sm => reports_card(self.dark, &weak).into_any_element(),
            DemoKind::Spacing => terms_card(self.dark, &weak).into_any_element(),
            DemoKind::Image => meetup_card(self.dark, &weak).into_any_element(),
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
                    .child(card)
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
        window_bounds: Some(WindowBounds::centered(size(px(560.), px(560.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| CardDemo::new(&variant, dark, cx))
    })
    .expect("open card gallery");
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
