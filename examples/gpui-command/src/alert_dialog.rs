//! shadcn/ui-style alert dialog for GPUI, built on the gpui-base primitives.
//! Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/alert-dialog
use gpui_kit::{self as kit, *};
use kit::base::{
    AlertDialog, AlertDialogAction, AlertDialogBackdrop, AlertDialogCancel, AlertDialogDescription,
    AlertDialogPopup, AlertDialogTitle, AlertDialogTrigger, DialogHandle, Easing, Transition,
    transition,
};
use std::{borrow::Cow, time::Duration};

const FONT: &str = "Geist";
const INTRO: Duration = Duration::from_millis(220);

/// Popup width preset, matching shadcn's `size` prop on AlertDialogContent.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertDialogSize {
    Default,
    Sm,
}

/// Icon shown in the media slot of an [`AlertDialogSpec`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlertDialogMedia {
    Plus,
    Bluetooth,
    Trash,
}

/// Content and behavior of one alert dialog instance. The host view keeps a
/// [`DialogHandle`] and renders `alert_dialog(&spec, &handle, ..)`; the dialog
/// opens through [`AlertDialogTrigger`] and closes through the Cancel/Action
/// buttons, Escape, or `handle.close(..)`.
#[derive(Clone)]
pub struct AlertDialogSpec {
    pub trigger: SharedString,
    pub title: SharedString,
    pub description: SharedString,
    pub cancel: SharedString,
    pub confirm: SharedString,
    pub size: AlertDialogSize,
    pub media: Option<AlertDialogMedia>,
    /// Paints the confirm button in the destructive treatment.
    pub destructive: bool,
    pub dark: bool,
}

impl Default for AlertDialogSpec {
    fn default() -> Self {
        Self {
            trigger: "Show Dialog".into(),
            title: "Are you absolutely sure?".into(),
            description: "This action cannot be undone. This will permanently delete your account and remove your data from our servers.".into(),
            cancel: "Cancel".into(),
            confirm: "Continue".into(),
            size: AlertDialogSize::Default,
            media: None,
            destructive: false,
            dark: false,
        }
    }
}

impl AlertDialogSpec {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn trigger(mut self, label: impl Into<SharedString>) -> Self {
        self.trigger = label.into();
        self
    }
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = description.into();
        self
    }
    pub fn cancel(mut self, label: impl Into<SharedString>) -> Self {
        self.cancel = label.into();
        self
    }
    pub fn confirm(mut self, label: impl Into<SharedString>) -> Self {
        self.confirm = label.into();
        self
    }
    pub fn size(mut self, size: AlertDialogSize) -> Self {
        self.size = size;
        self
    }
    pub fn media(mut self, media: AlertDialogMedia) -> Self {
        self.media = Some(media);
        self
    }
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

struct Palette {
    dark: bool,
    page: Hsla,
    surface: Hsla,
    border: Hsla,
    foreground: Hsla,
    muted: Hsla,
    scrim: Hsla,
    primary: Hsla,
    primary_foreground: Hsla,
}

fn palette(dark: bool) -> Palette {
    if dark {
        Palette {
            dark,
            page: rgb(0x202020).into(),
            surface: rgb(0x232323).into(),
            border: rgb(0x3b3b3b).into(),
            foreground: rgb(0xededed).into(),
            muted: rgb(0xa3a3a3).into(),
            scrim: rgb(0x000000).opacity(0.62).into(),
            primary: rgb(0xededed).into(),
            primary_foreground: rgb(0x171717).into(),
        }
    } else {
        Palette {
            dark,
            page: rgb(0xf8f8f8).into(),
            surface: rgb(0xffffff).into(),
            border: rgb(0xe4e4e4).into(),
            foreground: rgb(0x171717).into(),
            muted: rgb(0x737373).into(),
            scrim: rgb(0x171717).opacity(0.4).into(),
            primary: rgb(0x171717).into(),
            primary_foreground: rgb(0xffffff).into(),
        }
    }
}

fn destructive_ink(dark: bool) -> Hsla {
    rgb(if dark { 0xef8473 } else { 0xc9372c }).into()
}

/// Paints a 16 × 16 stroke icon inside the media box.
fn media_icon(kind: AlertDialogMedia, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let origin = bounds.origin;
            let mut builder = PathBuilder::stroke(px(1.4));
            match kind {
                AlertDialogMedia::Plus => {
                    circle_at(&mut builder, origin + point(px(8.), px(8.)), px(6.8));
                    stroke_line(&mut builder, origin, px(8.), px(5.2), px(8.), px(10.8));
                    stroke_line(&mut builder, origin, px(5.2), px(8.), px(10.8), px(8.));
                }
                AlertDialogMedia::Bluetooth => {
                    // Two stacked triangles sharing the right-pointing vertex.
                    polyline(&mut builder, origin, &[(6.5, 2.2), (10.4, 5.6), (6.5, 9.)]);
                    polyline(
                        &mut builder,
                        origin,
                        &[(6.5, 9.), (10.4, 12.4), (6.5, 15.8)],
                    );
                    stroke_line(&mut builder, origin, px(10.4), px(5.6), px(4.2), px(10.6));
                    stroke_line(&mut builder, origin, px(10.4), px(12.4), px(4.2), px(7.4));
                }
                AlertDialogMedia::Trash => {
                    polyline(
                        &mut builder,
                        origin,
                        &[(4.4, 4.4), (5., 13.2), (11., 13.2), (11.6, 4.4)],
                    );
                    stroke_line(&mut builder, origin, px(2.8), px(4.4), px(13.2), px(4.4));
                    stroke_line(&mut builder, origin, px(6.4), px(4.4), px(6.9), px(2.6));
                    stroke_line(&mut builder, origin, px(9.1), px(2.6), px(9.6), px(4.4));
                    stroke_line(&mut builder, origin, px(6.7), px(6.8), px(6.7), px(10.8));
                    stroke_line(&mut builder, origin, px(9.3), px(6.8), px(9.3), px(10.8));
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

fn button_face(colors: &Palette, primary: bool, destructive: bool, wide: bool) -> Div {
    let mut face = div()
        .flex()
        .items_center()
        .justify_center()
        .h_9()
        .px_4()
        .rounded_md()
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .cursor_pointer();
    if wide {
        face = face.w_full();
    }
    if primary {
        let fill = if destructive {
            destructive_ink(colors.dark)
        } else {
            colors.primary
        };
        face.bg(fill)
            .text_color(colors.primary_foreground)
            .hover(|style| style.opacity(0.88))
    } else {
        face.border_1()
            .border_color(colors.border)
            .text_color(colors.foreground)
            .hover(|style| style.bg(colors.border.opacity(0.35)))
    }
}

/// The composed alert dialog: dimmed backdrop + centered popup with title,
/// description and a Cancel/Action footer. `on_done` runs after a button
/// resolves the dialog (`true` for confirm, `false` for cancel).
pub fn alert_dialog(
    spec: &AlertDialogSpec,
    handle: &DialogHandle,
    intro: f32,
    on_done: impl Fn(bool, &mut Window, &mut App) + 'static,
    cx: &mut App,
) -> AlertDialog {
    let colors = palette(spec.dark);
    let small = spec.size == AlertDialogSize::Sm;
    let destructive = spec.destructive;
    let on_done = std::rc::Rc::new(on_done);
    let confirm_cb = on_done.clone();
    let cancel_cb = on_done;

    let popup_width = if small { px(320.) } else { px(440.) };
    let mut popup = AlertDialogPopup::new()
        .w(popup_width)
        .rounded_xl()
        .border_1()
        .border_color(colors.border)
        .bg(colors.surface)
        .p_6()
        .shadow_lg()
        .flex()
        .flex_col()
        .gap_2()
        .font_family(FONT)
        .opacity(intro)
        .mt(px((1. - intro) * 10.));

    let mut heading = div().flex().flex_col().gap_2();
    if let Some(media) = spec.media {
        heading = heading.child(
            div()
                .w(px(36.))
                .h(px(36.))
                .rounded_md()
                .border_1()
                .border_color(colors.border)
                .bg(colors.border.opacity(0.3))
                .flex()
                .items_center()
                .justify_center()
                .child(div().size(px(16.)).child(media_icon(
                    media,
                    if destructive {
                        destructive_ink(spec.dark)
                    } else {
                        colors.foreground
                    },
                ))),
        );
    }
    heading = heading
        .child(
            AlertDialogTitle::new()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(colors.foreground)
                .child(spec.title.clone()),
        )
        .child(
            AlertDialogDescription::new()
                .text_sm()
                .line_height(relative(1.55))
                .text_color(colors.muted)
                .child(spec.description.clone()),
        );
    popup = popup.child(heading);

    let cancel_face = button_face(&colors, false, false, small).child(spec.cancel.clone());
    let action_face = button_face(&colors, true, destructive, small).child(spec.confirm.clone());
    let mut footer = div().mt_4().flex().gap_2();
    if small {
        footer = footer.flex_col_reverse();
    } else {
        footer = footer.justify_end();
    }
    popup = popup.child(
        footer
            .child(AlertDialogCancel::new().child(cancel_face))
            .child(AlertDialogAction::new().child(action_face)),
    );

    AlertDialog::new(cx)
        .handle(handle.clone())
        .backdrop(
            AlertDialogBackdrop::new()
                .absolute()
                .inset_0()
                .bg(colors.scrim)
                .opacity(intro),
        )
        .popup(
            div()
                .absolute()
                .inset_0()
                .flex()
                .items_center()
                .justify_center()
                .child(popup),
        )
        .on_ok(move |_, window, cx| {
            confirm_cb(true, window, cx);
            true
        })
        .on_cancel(move |_, window, cx| {
            cancel_cb(false, window, cx);
            true
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoKind {
    Basic,
    Small,
    Media,
    SmallMedia,
    Destructive,
}

fn demo_spec(kind: DemoKind, dark: bool) -> AlertDialogSpec {
    let spec = AlertDialogSpec::new().dark(dark);
    match kind {
        DemoKind::Basic => spec,
        DemoKind::Small => spec
            .size(AlertDialogSize::Sm)
            .title("Allow app to access your location?")
            .description("This lets the app show nearby places and directions."),
        DemoKind::Media => spec
            .trigger("Share Project")
            .media(AlertDialogMedia::Plus)
            .title("Share this project?")
            .description("Anyone with the link can view this project. Members get edit access."),
        DemoKind::SmallMedia => spec
            .trigger("Pair Device")
            .size(AlertDialogSize::Sm)
            .media(AlertDialogMedia::Bluetooth)
            .title("Pair with this device?")
            .description("Headphones X2 want to connect."),
        DemoKind::Destructive => spec
            .trigger("Delete Chat")
            .media(AlertDialogMedia::Trash)
            .destructive(true)
            .title("Delete this chat?")
            .description("This will permanently delete the conversation and all of its messages.")
            .confirm("Delete"),
    }
}

pub struct AlertDialogDemo {
    handle: DialogHandle,
    dialog_focus: FocusHandle,
    trigger_focus: FocusHandle,
    kind: DemoKind,
    dark: bool,
    opened: u32,
    outcome: Option<&'static str>,
}

impl AlertDialogDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "sm" | "small" => DemoKind::Small,
            "media" => DemoKind::Media,
            "sm-media" | "small-media" => DemoKind::SmallMedia,
            "destructive" => DemoKind::Destructive,
            _ => DemoKind::Basic,
        };
        Self {
            handle: DialogHandle::new(false),
            dialog_focus: cx.focus_handle(),
            trigger_focus: cx.focus_handle(),
            kind,
            dark,
            opened: 0,
            outcome: None,
        }
    }
}

impl Render for AlertDialogDemo {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = palette(self.dark);
        let spec = demo_spec(self.kind, self.dark);
        let intro = transition(
            ("alert-dialog", self.opened.to_string()),
            1.,
            Transition::new(INTRO).easing(Easing::EaseOut),
            window,
            cx,
        );
        let demo = cx.entity().downgrade();

        let dialog = alert_dialog(
            &spec,
            &self.handle,
            intro,
            move |confirmed, _, cx| {
                if let Some(demo) = demo.upgrade() {
                    let _ = demo.update(cx, |demo, cx| {
                        demo.outcome = Some(if confirmed {
                            "Confirmed."
                        } else {
                            "Cancelled."
                        });
                        cx.notify();
                    });
                }
            },
            cx,
        );

        // Bump the intro counter on open so the fade replays, and move focus
        // into the popup — Cancel/Confirm actions and the Escape/Enter
        // bindings dispatch from the focused element, so the dialog must own
        // focus for them to land. Closing hands focus back to the trigger.
        let opener = cx.entity().downgrade();
        let dialog_focus = self.dialog_focus.clone();
        let trigger_focus = self.trigger_focus.clone();
        let dialog = dialog
            .focus_handle(self.dialog_focus.clone())
            .on_open_change(move |open, _, window, cx| {
                if open {
                    dialog_focus.focus(window, cx);
                    if let Some(demo) = opener.upgrade() {
                        let _ = demo.update(cx, |demo, cx| {
                            demo.opened += 1;
                            demo.outcome = None;
                            cx.notify();
                        });
                    }
                } else {
                    trigger_focus.focus(window, cx);
                }
            });

        // The dialog only exists while open; keep focus pinned inside it so
        // clicks elsewhere cannot strand focus outside its dispatch path.
        if self.handle.is_open() {
            self.dialog_focus.focus(window, cx);
        }

        let open_handle = self.handle.clone();
        let trigger = AlertDialogTrigger::new(
            button_face(&colors, false, false, false)
                .id("alert-dialog-trigger")
                .track_focus(&self.trigger_focus.clone().tab_index(0))
                .focus_visible(|style| style.border_color(colors.primary))
                .on_key_down(move |event: &KeyDownEvent, window, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        open_handle.open(window, cx);
                        cx.stop_propagation();
                    }
                })
                .child(spec.trigger.clone()),
        )
        .handle(self.handle.clone());

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
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(trigger)
                    .child(
                        div()
                            .h(px(18.))
                            .text_xs()
                            .text_color(colors.muted)
                            .child(self.outcome.unwrap_or(" ")),
                    ),
            )
            .child(dialog)
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
        cx.new(|cx| AlertDialogDemo::new(&variant, dark, cx))
    })
    .expect("open alert dialog gallery");
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
    cx.new(|cx| AlertDialogDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{AlertDialogMedia, AlertDialogSize, AlertDialogSpec};

    #[test]
    fn spec_defaults_match_the_basic_example() {
        let spec = AlertDialogSpec::new();
        assert_eq!(spec.trigger.as_ref(), "Show Dialog");
        assert_eq!(spec.size, AlertDialogSize::Default);
        assert!(spec.media.is_none());
        assert!(!spec.destructive);
    }

    #[test]
    fn builder_overrides_stick() {
        let spec = AlertDialogSpec::new()
            .trigger("Delete Chat")
            .title("Delete this chat?")
            .size(AlertDialogSize::Sm)
            .media(AlertDialogMedia::Trash)
            .destructive(true)
            .dark(true);
        assert_eq!(spec.trigger.as_ref(), "Delete Chat");
        assert_eq!(spec.size, AlertDialogSize::Sm);
        assert_eq!(spec.media, Some(AlertDialogMedia::Trash));
        assert!(spec.destructive && spec.dark);
    }
}
