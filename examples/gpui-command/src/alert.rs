//! shadcn/ui-style alert callout for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/alert
use gpui_kit::{self as kit, *};
use kit::base::{Easing, Transition, transition};
use std::{borrow::Cow, time::Duration};

const FONT: &str = "Geist";
const INTRO: Duration = Duration::from_millis(260);

/// Emitted when the optional action button is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct AlertActionEvent {
    pub label: SharedString,
}

/// Line icon rendered at the start of an [`Alert`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertIcon {
    Info,
    Check,
    Warning,
    Error,
}

/// Base color treatment. `Destructive` tints the whole callout red; any
/// surface can be overridden with the `surface`/`edge`/`tint` builders.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertVariant {
    Default,
    Destructive,
}

/// A styled alert entity. The host keeps the entity on its view and renders it
/// with `.child(self.alert.clone())`; the optional action button emits
/// [`AlertActionEvent`].
pub struct Alert {
    title: SharedString,
    description: SharedString,
    icon: Option<AlertIcon>,
    action: Option<SharedString>,
    variant: AlertVariant,
    surface: Option<Hsla>,
    edge: Option<Hsla>,
    tint: Option<Hsla>,
    dark: bool,
}
impl EventEmitter<AlertActionEvent> for Alert {}

impl Alert {
    pub fn new() -> Self {
        Self {
            title: SharedString::default(),
            description: SharedString::default(),
            icon: None,
            action: None,
            variant: AlertVariant::Default,
            surface: None,
            edge: None,
            tint: None,
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
    pub fn icon(mut self, icon: AlertIcon) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Adds a compact action button on the trailing edge. Pressing it emits
    /// [`AlertActionEvent`] carrying the label.
    pub fn action(mut self, label: impl Into<SharedString>) -> Self {
        self.action = Some(label.into());
        self
    }
    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }
    /// Overrides the callout fill, e.g. the amber custom-color example.
    pub fn surface(mut self, surface: impl Into<Hsla>) -> Self {
        self.surface = Some(surface.into());
        self
    }
    /// Overrides the border color.
    pub fn edge(mut self, edge: impl Into<Hsla>) -> Self {
        self.edge = Some(edge.into());
        self
    }
    /// Overrides the icon color.
    pub fn tint(mut self, tint: impl Into<Hsla>) -> Self {
        self.tint = Some(tint.into());
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
    icon: Hsla,
}

fn palette(alert: &Alert) -> Palette {
    let dark = alert.dark;
    let (surface, border, foreground, muted, icon) = match alert.variant {
        AlertVariant::Default => {
            let surface = rgb(if dark { 0x232323 } else { 0xffffff });
            let border = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 });
            let foreground = rgb(if dark { 0xededed } else { 0x171717 });
            let muted = rgb(if dark { 0xa3a3a3 } else { 0x737373 });
            (surface, border, foreground, muted, foreground)
        }
        AlertVariant::Destructive => {
            let surface = rgb(if dark { 0x2a1f1f } else { 0xffffff });
            let border = rgb(if dark { 0x6b3434 } else { 0xe8b4ae });
            let destructive = rgb(if dark { 0xef8473 } else { 0xc9372c });
            (surface, border, destructive, destructive, destructive)
        }
    };
    Palette {
        surface: alert.surface.unwrap_or_else(|| surface.into()),
        border: alert.edge.unwrap_or_else(|| border.into()),
        foreground: foreground.into(),
        muted: muted.into(),
        icon: alert.tint.unwrap_or_else(|| icon.into()),
    }
}

/// Paints a 16 × 16 line icon: a circle, triangle or check built from arcs and
/// strokes, matching the lucide glyph shapes on the reference page.
fn icon(kind: AlertIcon, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let origin = bounds.origin;
            let stroke = px(1.4);
            let mut builder = PathBuilder::stroke(stroke);
            match kind {
                AlertIcon::Info | AlertIcon::Check | AlertIcon::Error => {
                    circle_at(&mut builder, origin + point(px(8.), px(8.)), px(7.2));
                    match kind {
                        AlertIcon::Info => {
                            stroke_line(&mut builder, origin, px(8.), px(8.), px(8.), px(11.6));
                            circle_at(&mut builder, origin + point(px(8.), px(4.8)), px(0.9));
                        }
                        AlertIcon::Check => {
                            polyline(&mut builder, origin, &[(5., 8.4), (7.3, 10.7), (11., 5.6)]);
                        }
                        _ => {
                            stroke_line(&mut builder, origin, px(8.), px(4.6), px(8.), px(9.8));
                            circle_at(&mut builder, origin + point(px(8.), px(11.6)), px(0.9));
                        }
                    }
                }
                AlertIcon::Warning => {
                    polyline(
                        &mut builder,
                        origin,
                        &[(8., 2.4), (14.4, 12.4), (1.6, 12.4), (8., 2.4)],
                    );
                    stroke_line(&mut builder, origin, px(8.), px(6.4), px(8.), px(9.4));
                    circle_at(&mut builder, origin + point(px(8.), px(11.)), px(0.9));
                }
            }
            if let Ok(path) = builder.build() {
                window.paint_path(path, color);
            }
        },
    )
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

fn stroke_line(
    builder: &mut PathBuilder,
    origin: Point<Pixels>,
    x1: Pixels,
    y1: Pixels,
    x2: Pixels,
    y2: Pixels,
) {
    builder.move_to(origin + point(x1, y1));
    builder.line_to(origin + point(x2, y2));
}

fn polyline(builder: &mut PathBuilder, origin: Point<Pixels>, points: &[(f32, f32)]) {
    for (index, (x, y)) in points.iter().enumerate() {
        let vertex = origin + point(px(*x), px(*y));
        if index == 0 {
            builder.move_to(vertex);
        } else {
            builder.line_to(vertex);
        }
    }
}

impl Render for Alert {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = palette(self);
        let title = self.title.clone();
        let description = self.description.clone();
        let action = self.action.clone();
        let icon = self.icon;

        let progress = transition(
            ("alert-intro", title.clone()),
            1.,
            Transition::new(INTRO).easing(Easing::EaseOut),
            window,
            cx,
        );
        let focus = window
            .use_keyed_state("alert-action", cx, |_, cx| cx.focus_handle())
            .read(cx)
            .clone();
        let alert = cx.entity().downgrade();

        let mut content = div().flex().flex_col().gap_1().flex_1().min_w_0().child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(colors.foreground)
                .child(title),
        );
        if !description.is_empty() {
            content = content.child(
                div()
                    .text_sm()
                    .line_height(relative(1.55))
                    .text_color(colors.muted)
                    .child(description),
            );
        }

        let mut row = div()
            .w_full()
            .flex()
            .items_start()
            .gap_3()
            .rounded_lg()
            .border_1()
            .border_color(colors.border)
            .bg(colors.surface)
            .px_4()
            .py(px(14.))
            .font_family(FONT)
            .opacity(progress);
        if let Some(icon) = icon {
            row = row.child(
                div()
                    .w(px(16.))
                    .h(px(16.))
                    .flex_none()
                    .mt(px(1.))
                    .child(self::icon(icon, colors.icon)),
            );
        }
        row = row.child(content);
        if let Some(label) = action {
            let emitted = label.clone();
            row = row.child(
                div()
                    .id("alert-action")
                    .track_focus(&focus.tab_index(0))
                    .flex_none()
                    .cursor_pointer()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.border)
                    .px_3()
                    .py_1()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(colors.foreground)
                    .hover(|style| style.bg(colors.border.opacity(0.35)))
                    .on_click(move |_, _, cx| {
                        if let Some(alert) = alert.upgrade() {
                            let _ = alert.update(cx, |_, cx| {
                                cx.emit(AlertActionEvent {
                                    label: emitted.clone(),
                                });
                                cx.notify();
                            });
                        }
                    })
                    .child(label),
            );
        }
        row
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Destructive,
    Action,
    Custom,
}

fn demo_alerts(kind: DemoKind, dark: bool) -> Vec<Alert> {
    match kind {
        DemoKind::Basic => vec![
            Alert::new()
                .icon(AlertIcon::Check)
                .title("Payment successful")
                .description("Your payment of $29.99 has been processed. A receipt has been sent to your email address.")
                .dark(dark),
            Alert::new()
                .icon(AlertIcon::Info)
                .title("New feature available")
                .description("We've added dark mode support. You can enable it in your account settings.")
                .dark(dark),
        ],
        DemoKind::Destructive => vec![Alert::new()
            .variant(AlertVariant::Destructive)
            .icon(AlertIcon::Error)
            .title("Payment failed")
            .description("Your payment could not be processed. Please check your payment method and try again.")
            .dark(dark)],
        DemoKind::Action => vec![Alert::new()
            .icon(AlertIcon::Info)
            .title("Dark mode is now available")
            .description("Enable it under your profile settings to get started.")
            .action("Enable")
            .dark(dark)],
        DemoKind::Custom => vec![Alert::new()
            .icon(AlertIcon::Warning)
            .surface(rgb(if dark { 0x33270f } else { 0xfffbec }))
            .edge(rgb(if dark { 0x7a5a16 } else { 0xf3d889 }))
            .tint(rgb(if dark { 0xf0b429 } else { 0xa15c07 }))
            .title("Your subscription will expire in 3 days.")
            .description("Renew now to avoid service interruption or upgrade to a paid plan to continue using the service.")
            .dark(dark)],
    }
}

struct AlertDemo {
    alerts: Vec<Entity<Alert>>,
    dark: bool,
    _subscriptions: Vec<Subscription>,
}
impl AlertDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "destructive" => DemoKind::Destructive,
            "action" => DemoKind::Action,
            "custom" => DemoKind::Custom,
            _ => DemoKind::Basic,
        };
        let mut subscriptions = Vec::new();
        let alerts = demo_alerts(kind, dark)
            .into_iter()
            .map(|alert| {
                let entity = cx.new(|_| alert);
                subscriptions.push(cx.subscribe(&entity, |_, _, event: &AlertActionEvent, _| {
                    println!("alert action pressed: {}", event.label);
                }));
                entity
            })
            .collect();
        Self {
            alerts,
            dark,
            _subscriptions: subscriptions,
        }
    }
}
impl Render for AlertDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
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
                    .w_full()
                    .max_w(px(460.))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .children(self.alerts.iter().cloned()),
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(520.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| AlertDemo::new(&variant, dark, cx))
    })
    .expect("open alert gallery");
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
    use super::{Alert, AlertIcon, AlertVariant, palette};
    use gpui_kit::rgb;

    #[test]
    fn builders_store_content() {
        let alert = Alert::new()
            .icon(AlertIcon::Check)
            .title("Done")
            .description("All good")
            .action("Undo")
            .variant(AlertVariant::Destructive);
        assert_eq!(alert.title.as_ref(), "Done");
        assert_eq!(alert.description.as_ref(), "All good");
        assert_eq!(alert.action.as_deref(), Some("Undo"));
        assert_eq!(alert.variant, AlertVariant::Destructive);
    }

    #[test]
    fn custom_surfaces_override_the_variant() {
        let custom = rgb(0xfffbec);
        let alert = Alert::new().surface(custom).edge(rgb(0xf3d889));
        let colors = palette(&alert);
        assert_eq!(colors.surface, custom.into());
        assert_eq!(colors.border, rgb(0xf3d889).into());
        // Foreground still comes from the variant.
        assert_eq!(colors.foreground, rgb(0x171717).into());
    }

    #[test]
    fn destructive_uses_destructive_foreground() {
        let alert = Alert::new().variant(AlertVariant::Destructive);
        let colors = palette(&alert);
        assert_eq!(colors.foreground, colors.icon);
        assert_eq!(colors.foreground, rgb(0xc9372c).into());
    }
}
