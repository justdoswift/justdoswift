//! shadcn/ui-style button group for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/button-group
use crate::button::{Button, ButtonIcon, ButtonJoin, ButtonSize, ButtonVariant, IconPos};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

/// A `role="group"` container joining related buttons. Children opt into the
/// merged look with `Button::join(..)` / `Button::vjoin(..)`; spacers and
/// nested groups keep their own gap.
#[derive(IntoElement)]
pub struct ButtonGroup {
    vertical: bool,
    gap: Pixels,
    dark: bool,
    children: Vec<AnyElement>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            vertical: false,
            gap: px(8.),
            dark: false,
            children: Vec::new(),
        }
    }
    /// `orientation="vertical"` — stacks children and stretches them to the
    /// same width.
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
    /// Spacing between children that are NOT joined (nested groups, split
    /// buttons keep `gap`, joined buttons use `join`/`vjoin` instead).
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

impl ParentElement for ButtonGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .when(!self.vertical, |style| {
                style.flex_row().items_center().gap(self.gap)
            })
            .when(self.vertical, |style| {
                style.flex_col().items_start().gap(self.gap)
            })
            .children(self.children)
    }
}

/// `ButtonGroupSeparator` — a 1px divider, vertical by default and stretched
/// to the group's full height (`self-stretch`). In a vertical group pass
/// `vertical(true)` for a horizontal rule.
pub fn group_separator(vertical: bool, dark: bool) -> impl IntoElement {
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xd4d4d4 }).into();
    div()
        .flex_none()
        .self_stretch()
        .bg(edge)
        .when(!vertical, |style| style.w(px(1.)))
        .when(vertical, |style| style.h(px(1.)).w_full())
}

/// `ButtonGroupText` — a non-interactive label cell dressed like a button.
pub fn group_text(text: impl Into<SharedString>, dark: bool) -> impl IntoElement {
    let surface: Hsla = rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    div()
        .flex_none()
        .flex()
        .items_center()
        .h(px(36.))
        .px_4()
        .rounded(px(8.))
        .bg(surface)
        .border_1()
        .border_color(edge)
        .font_family(FONT)
        .text_sm()
        .font_weight(FontWeight::MEDIUM)
        .text_color(ink)
        .whitespace_nowrap()
        .child(text.into())
}

/// A non-editable input cell for the Input demo — renders like `Input`,
/// paired with a joined button.
fn fake_input(placeholder: &str, dark: bool) -> impl IntoElement {
    let ink: Hsla = rgb(if dark { 0x737373 } else { 0xa3a3a3 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
    let surface: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();
    div()
        .flex_none()
        .flex()
        .items_center()
        .h(px(36.))
        .w(px(180.))
        .px_3()
        .rounded_tl(px(8.))
        .rounded_bl(px(8.))
        .bg(surface)
        .border_1()
        .border_color(edge)
        .font_family(FONT)
        .text_sm()
        .text_color(ink)
        .child(placeholder.to_string())
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Orientation,
    Sizes,
    Nested,
    Separator,
    Split,
    Input,
}

pub struct ButtonGroupDemo {
    variant: String,
    dark: bool,
    outcome: Option<SharedString>,
}

impl ButtonGroupDemo {
    fn new(variant: &str, dark: bool, _cx: &mut Context<Self>) -> Self {
        Self {
            variant: variant.to_string(),
            dark,
            outcome: None,
        }
    }

    fn press(&self, id: &str, label: &str, cx: &mut Context<Self>) -> Button {
        let weak = cx.entity().downgrade();
        let name: SharedString = label.to_string().into();
        Button::new(id.to_string())
            .label(name.clone())
            .dark(self.dark)
            .on_click(move |_, _, cx| {
                if let Some(demo) = weak.upgrade() {
                    let _ = demo.update(cx, |demo, cx| {
                        demo.outcome = Some(format!("Clicked {name}").into());
                        cx.notify();
                    });
                }
            })
    }

    fn press_icon(&self, id: &str, a11y: &str, cx: &mut Context<Self>) -> Button {
        let weak = cx.entity().downgrade();
        let name: SharedString = a11y.to_string().into();
        Button::new(id.to_string())
            .a11y_label(name.clone())
            .dark(self.dark)
            .on_click(move |_, _, cx| {
                if let Some(demo) = weak.upgrade() {
                    let _ = demo.update(cx, |demo, cx| {
                        demo.outcome = Some(format!("Clicked {name}").into());
                        cx.notify();
                    });
                }
            })
    }
}

impl Render for ButtonGroupDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let dark = self.dark;
        let kind = match self.variant.as_str() {
            "orientation" => DemoKind::Orientation,
            "sizes" => DemoKind::Sizes,
            "nested" => DemoKind::Nested,
            "separator" => DemoKind::Separator,
            "split" => DemoKind::Split,
            "input" => DemoKind::Input,
            _ => DemoKind::Basic,
        };

        let content: AnyElement = match kind {
            // Archive | Report joined, Snooze separate — matches the hero.
            DemoKind::Basic => ButtonGroup::new()
                .dark(dark)
                .gap(px(0.))
                .child(
                    self.press("b-archive", "Archive", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::Start),
                )
                .child(
                    self.press("b-report", "Report", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::End),
                )
                .child(
                    div().pl(px(8.)).child(
                        self.press("b-snooze", "Snooze", cx)
                            .variant(ButtonVariant::Outline),
                    ),
                )
                .into_any_element(),
            // Vertical group of icon buttons joined top-to-bottom.
            DemoKind::Orientation => ButtonGroup::new()
                .dark(dark)
                .vertical(true)
                .gap(px(0.))
                .child(
                    self.press_icon("o-plus", "Increase", cx)
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .icon(ButtonIcon::Plus, IconPos::Start)
                        .vjoin(ButtonJoin::Start),
                )
                .child(
                    self.press_icon("o-minus", "Decrease", cx)
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Icon)
                        .icon(ButtonIcon::Minus, IconPos::Start)
                        .vjoin(ButtonJoin::End),
                )
                .into_any_element(),
            DemoKind::Sizes => ButtonGroup::new()
                .dark(dark)
                .vertical(true)
                .gap(px(12.))
                .children(
                    [
                        (ButtonSize::Sm, ButtonSize::IconSm, "s-sm"),
                        (ButtonSize::Default, ButtonSize::Icon, "s-df"),
                        (ButtonSize::Lg, ButtonSize::IconLg, "s-lg"),
                    ]
                    .into_iter()
                    .map(|(size, icon_size, id)| {
                        ButtonGroup::new()
                            .dark(dark)
                            .gap(px(0.))
                            .child(
                                self.press(&format!("{id}-label"), "Button", cx)
                                    .variant(ButtonVariant::Outline)
                                    .size(size)
                                    .join(ButtonJoin::Start),
                            )
                            .child(
                                self.press_icon(&format!("{id}-icon"), "Add", cx)
                                    .variant(ButtonVariant::Outline)
                                    .size(icon_size)
                                    .icon(ButtonIcon::Plus, IconPos::Start)
                                    .join(ButtonJoin::End),
                            )
                            .into_any_element()
                    }),
                )
                .into_any_element(),
            // Two joined groups separated by the group's own gap.
            DemoKind::Nested => ButtonGroup::new()
                .dark(dark)
                .gap(px(16.))
                .child(
                    ButtonGroup::new()
                        .dark(dark)
                        .gap(px(0.))
                        .child(
                            self.press("n-1", "1", cx)
                                .variant(ButtonVariant::Secondary)
                                .size(ButtonSize::Sm)
                                .join(ButtonJoin::Start),
                        )
                        .child(
                            self.press("n-2", "2", cx)
                                .variant(ButtonVariant::Secondary)
                                .size(ButtonSize::Sm)
                                .join(ButtonJoin::Middle),
                        )
                        .child(
                            self.press("n-3", "3", cx)
                                .variant(ButtonVariant::Secondary)
                                .size(ButtonSize::Sm)
                                .join(ButtonJoin::End),
                        ),
                )
                .child(
                    ButtonGroup::new()
                        .dark(dark)
                        .gap(px(0.))
                        .child(
                            self.press_icon("n-prev", "Previous", cx)
                                .variant(ButtonVariant::Secondary)
                                .size(ButtonSize::IconSm)
                                .icon(ButtonIcon::ChevronLeft, IconPos::Start)
                                .join(ButtonJoin::Start),
                        )
                        .child(
                            self.press_icon("n-next", "Next", cx)
                                .variant(ButtonVariant::Secondary)
                                .size(ButtonSize::IconSm)
                                .icon(ButtonIcon::ChevronRight, IconPos::Start)
                                .join(ButtonJoin::End),
                        ),
                )
                .into_any_element(),
            // Default-variant buttons need a separator since they share a
            // surface color (outline buttons already have visible borders).
            DemoKind::Separator => ButtonGroup::new()
                .dark(dark)
                .gap(px(0.))
                .child(self.press("sep-copy", "Copy", cx).join(ButtonJoin::Start))
                .child(group_separator(false, dark).into_any_element())
                .child(self.press("sep-paste", "Paste", cx).join(ButtonJoin::End))
                .into_any_element(),
            // Split button: label + separator + icon-only trigger.
            DemoKind::Split => ButtonGroup::new()
                .dark(dark)
                .gap(px(0.))
                .child(
                    self.press("sp-main", "Publish", cx)
                        .variant(ButtonVariant::Secondary)
                        .join(ButtonJoin::Start),
                )
                .child(group_separator(false, dark).into_any_element())
                .child(
                    self.press_icon("sp-more", "More options", cx)
                        .variant(ButtonVariant::Secondary)
                        .size(ButtonSize::Icon)
                        .icon(ButtonIcon::ChevronDown, IconPos::Start)
                        .join(ButtonJoin::End),
                )
                .into_any_element(),
            // Input cell joined to a trailing button.
            DemoKind::Input => ButtonGroup::new()
                .dark(dark)
                .gap(px(0.))
                .child(fake_input("Search...", dark).into_any_element())
                .child(
                    self.press("in-search", "Search", cx)
                        .variant(ButtonVariant::Outline)
                        .join(ButtonJoin::End),
                )
                .into_any_element(),
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
                    .child(content)
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(400.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| ButtonGroupDemo::new(&variant, dark, cx))
    })
    .expect("open button group gallery");
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
    cx.new(|cx| ButtonGroupDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::ButtonGroup;
    use gpui_kit::ParentElement;

    #[test]
    fn builders_store_content() {
        let mut group = ButtonGroup::new()
            .vertical(true)
            .gap(gpui_kit::px(12.))
            .dark(true);
        assert!(group.vertical && group.dark);
        assert_eq!(group.gap, gpui_kit::px(12.));
        group.extend([]);
    }
}
