//! shadcn/ui-style accordion for GPUI, composed from the unstyled
//! `gpui-base` primitives. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/accordion
use gpui_kit::{self as kit, *};
use kit::base::{
    Accordion as AccordionRoot, AccordionHeader, AccordionItem, AccordionPanel, AccordionTrigger,
    Easing, MotionReveal, Transition, transition,
};
use kit::prelude::FluentBuilder;
use std::{borrow::Cow, f32::consts::PI, rc::Rc, time::Duration};

const FONT: &str = "Geist";
const REVEAL: Duration = Duration::from_millis(220);

/// Emitted whenever an entry is expanded or collapsed.
#[derive(Clone, Debug, PartialEq)]
pub struct AccordionChange {
    pub value: SharedString,
    pub open: bool,
}

/// One row of an [`Accordion`]: a stable value, a heading and its panel copy.
#[derive(Clone, Debug)]
pub struct AccordionEntry {
    value: SharedString,
    title: SharedString,
    content: SharedString,
    disabled: bool,
}
impl AccordionEntry {
    pub fn new(value: impl Into<SharedString>, title: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            title: title.into(),
            content: SharedString::default(),
            disabled: false,
        }
    }
    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

struct Item {
    entry: AccordionEntry,
    open: bool,
}

/// A styled accordion entity. The host keeps the entity on its view and renders
/// it with `.child(self.accordion.clone())`; open state, keyboard navigation
/// and the reveal animation are owned by the component.
pub struct Accordion {
    items: Vec<Item>,
    multiple: bool,
    collapsible: bool,
    dark: bool,
    focuses: Rc<Vec<FocusHandle>>,
}
impl EventEmitter<AccordionChange> for Accordion {}

impl Accordion {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            multiple: false,
            collapsible: true,
            dark: false,
            focuses: Rc::new(Vec::new()),
        }
    }
    /// Appends one entry. Entry order is presentation order.
    pub fn entry(mut self, entry: AccordionEntry) -> Self {
        self.items.push(Item { entry, open: false });
        self
    }
    /// Expands the entry with this value. Repeatable when `multiple` is set.
    pub fn open(mut self, value: impl AsRef<str>) -> Self {
        let value = value.as_ref();
        if let Some(item) = self.items.iter_mut().find(|item| item.entry.value == value) {
            item.open = true;
        }
        self
    }
    /// Allows several entries to stay expanded at once.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }
    /// Allows the last open entry to be collapsed. Only meaningful when
    /// `multiple` is off; a `multiple` accordion can always collapse.
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    pub fn is_open(&self, value: &str) -> bool {
        self.items
            .iter()
            .any(|item| item.entry.value == value && item.open)
    }

    fn set_open(&mut self, index: usize, open: bool, cx: &mut Context<Self>) {
        if !apply_open(
            &mut self.items,
            self.multiple,
            self.collapsible,
            index,
            open,
        ) {
            return;
        }
        cx.emit(AccordionChange {
            value: self.items[index].entry.value.clone(),
            open,
        });
        cx.notify();
    }

    /// WAI-ARIA accordion navigation: Up/Down cycle between headers,
    /// Home/End jump to the edges. Disabled entries are skipped.
    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let enabled: Vec<bool> = self.items.iter().map(|item| !item.entry.disabled).collect();
        let current = window
            .focused(cx)
            .and_then(|handle| self.focuses.iter().position(|focus| *focus == handle));
        let Some(next) = navigate(&enabled, current, event.keystroke.key.as_str()) else {
            return;
        };
        self.focuses[next].focus(window, cx);
        window.prevent_default();
        cx.stop_propagation();
    }
}

/// Pure state transition behind [`Accordion::set_open`]: disabled entries are
/// inert, single mode closes siblings, and closing the last open entry needs
/// `collapsible` (a `multiple` accordion can always collapse).
fn apply_open(
    items: &mut [Item],
    multiple: bool,
    collapsible: bool,
    index: usize,
    open: bool,
) -> bool {
    let Some(item) = items.get(index) else {
        return false;
    };
    if item.entry.disabled {
        return false;
    }
    if open {
        if !multiple {
            items.iter_mut().for_each(|item| item.open = false);
        }
        items[index].open = true;
        true
    } else if multiple || collapsible {
        items[index].open = false;
        true
    } else {
        false
    }
}

fn navigate(enabled: &[bool], current: Option<usize>, key: &str) -> Option<usize> {
    let count = enabled.len();
    if count == 0 || enabled.iter().all(|enabled| !enabled) {
        return None;
    }
    match key {
        "home" => enabled.iter().position(|enabled| *enabled),
        "end" => enabled.iter().rposition(|enabled| *enabled),
        "up" | "down" => {
            let step: isize = if key == "down" { 1 } else { -1 };
            let mut index = match current {
                Some(index) => (index as isize + step).rem_euclid(count as isize) as usize,
                None if step > 0 => 0,
                None => count - 1,
            };
            for _ in 0..count {
                if enabled[index] {
                    return Some(index);
                }
                index = (index as isize + step).rem_euclid(count as isize) as usize;
            }
            None
        }
        _ => None,
    }
}

fn chevron(progress: f32, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let center = bounds.center();
            let (sin, cos) = (PI * progress).sin_cos();
            let points = [
                point(px(-4.5), px(-1.6)),
                point(px(0.), px(2.4)),
                point(px(4.5), px(-1.6)),
            ];
            let mut builder = PathBuilder::stroke(px(1.5));
            for (index, vertex) in points.iter().enumerate() {
                let rotated = center
                    + point(
                        px(f32::from(vertex.x) * cos - f32::from(vertex.y) * sin),
                        px(f32::from(vertex.x) * sin + f32::from(vertex.y) * cos),
                    );
                if index == 0 {
                    builder.move_to(rotated);
                } else {
                    builder.line_to(rotated);
                }
            }
            if let Ok(path) = builder.build() {
                window.paint_path(path, color);
            }
        },
    )
}

impl Render for Accordion {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let foreground: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
        let border: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe8e8e8 }).into();
        let focus_bg: Hsla = rgb(if dark { 0x2c2c2c } else { 0xefefef }).into();

        let focuses: Vec<FocusHandle> = (0..self.items.len())
            .map(|index| {
                window
                    .use_keyed_state(("accordion-focus", index), cx, |_, cx| cx.focus_handle())
                    .read(cx)
                    .clone()
            })
            .collect();
        self.focuses = Rc::new(focuses.clone());
        let policy = Transition::new(REVEAL).easing(Easing::EaseOut);
        let accordion = cx.entity().downgrade();
        let last = self.items.len().saturating_sub(1);

        AccordionRoot::new("accordion")
            .w_full()
            .font_family(FONT)
            .on_key_down(cx.listener(Self::key_down))
            .children(self.items.iter().enumerate().map(|(index, item)| {
                let progress = transition(
                    ("accordion-item", item.entry.value.clone()),
                    if item.open { 1. } else { 0. },
                    policy.clone(),
                    window,
                    cx,
                );
                let disabled = item.entry.disabled;
                let accordion = accordion.clone();
                let trigger = AccordionTrigger::new(("accordion-trigger", index))
                    .track_focus(&focuses[index].clone().tab_index(0))
                    .w_full()
                    .flex()
                    .items_start()
                    .justify_between()
                    .gap_3()
                    .px_1()
                    .py_4()
                    .text_left()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(foreground)
                    .cursor_pointer()
                    .hover(|style| style.underline())
                    .focus_visible(|style| style.bg(focus_bg))
                    .when(disabled, |style| style.cursor_default().opacity(0.4))
                    .on_change(move |open, _, _, cx| {
                        let _ = accordion.update(cx, |this, cx| this.set_open(index, open, cx));
                    })
                    .child(div().flex_1().min_w_0().child(item.entry.title.clone()))
                    .child(div().flex_none().pt(px(1.)).child(chevron(progress, muted)));
                let panel = AccordionPanel::new()
                    .id(("accordion-panel", index))
                    .keep_mounted(progress > 0.001)
                    .child(
                        MotionReveal::new(
                            ("accordion-reveal", index),
                            progress,
                            div()
                                .pb_4()
                                .px_1()
                                .text_sm()
                                .line_height(relative(1.6))
                                .text_color(muted)
                                .child(item.entry.content.clone())
                                .into_any_element(),
                        )
                        .into_any_element(),
                    );
                AccordionItem::new()
                    .open(item.open)
                    .disabled(disabled)
                    .w_full()
                    .when(index != last, |item| item.border_b_1().border_color(border))
                    .header(AccordionHeader::new(trigger).id(("accordion-header", index)))
                    .panel(panel)
                    .into_any_element()
            }))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DemoKind {
    Basic,
    Multiple,
    Disabled,
    Card,
}
impl DemoKind {
    fn card(self) -> bool {
        self == Self::Card
    }
}

fn demo_entries(kind: DemoKind) -> Vec<AccordionEntry> {
    match kind {
        DemoKind::Basic => vec![
            AccordionEntry::new("shipping", "What are your shipping options?").content(
                "We offer standard (5-7 days), express (2-3 days), and overnight shipping. \
                 Free shipping on international orders.",
            ),
            AccordionEntry::new("returns", "What is your return policy?").content(
                "Returns are accepted within 30 days of purchase. Items must be unworn, \
                 unwashed, and in their original packaging.",
            ),
            AccordionEntry::new("support", "How can I contact customer support?").content(
                "Reach us at support@example.com or through the live chat. We reply within \
                 one business day.",
            ),
        ],
        DemoKind::Multiple => vec![
            AccordionEntry::new("notifications", "Notification Settings").content(
                "Manage how you receive notifications. You can enable email alerts for \
                 updates or push notifications for mobile devices.",
            ),
            AccordionEntry::new("privacy", "Privacy & Security").content(
                "Control who can see your activity, download your data, and manage \
                 two-factor authentication from one place.",
            ),
            AccordionEntry::new("billing", "Billing & Subscription").content(
                "Update your payment method, switch between monthly and annual billing, \
                 or download past invoices.",
            ),
        ],
        DemoKind::Disabled => vec![
            AccordionEntry::new("history", "Can I access my account history?").content(
                "Sign in and open Settings → History to browse sign-ins, purchases and \
                 changes to your account.",
            ),
            AccordionEntry::new("premium", "Premium feature information")
                .content("Available on the Professional and Enterprise plans.")
                .disabled(true),
            AccordionEntry::new("email", "How do I update my email address?").content(
                "Open Settings → Profile, edit the email field, and confirm the change \
                 from the verification link we send you.",
            ),
        ],
        DemoKind::Card => vec![
            AccordionEntry::new("plans", "What subscription plans do you offer?").content(
                "We offer three subscription tiers: Starter ($9/month), Professional \
                 ($29/month), and Enterprise ($99/month). Each plan includes increasing \
                 storage limits, API access, priority support, and team collaboration \
                 features.",
            ),
            AccordionEntry::new("billing", "How does billing work?").content(
                "Billing is charged at the beginning of each cycle. You can cancel \
                 anytime — the plan stays active until the end of the paid period.",
            ),
            AccordionEntry::new("cancel", "How do I cancel my subscription?").content(
                "Open Settings → Billing and choose Cancel plan. Your data is kept for \
                 30 days in case you change your mind.",
            ),
        ],
    }
}

struct AccordionDemo {
    accordion: Entity<Accordion>,
    dark: bool,
    card: bool,
}
impl AccordionDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "multiple" => DemoKind::Multiple,
            "disabled" => DemoKind::Disabled,
            "card" => DemoKind::Card,
            _ => DemoKind::Basic,
        };
        let entries = demo_entries(kind);
        let first = entries[0].value.clone();
        let accordion = entries
            .into_iter()
            .fold(
                Accordion::new()
                    .multiple(kind == DemoKind::Multiple)
                    .dark(dark),
                Accordion::entry,
            )
            .open(first);
        Self {
            accordion: cx.new(|_| accordion),
            dark,
            card: kind.card(),
        }
    }
}
impl Render for AccordionDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let card_bg = rgb(if self.dark { 0x262626 } else { 0xffffff });
        let border = rgb(if self.dark { 0x3b3b3b } else { 0xe8e8e8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let foreground = rgb(if self.dark { 0xededed } else { 0x171717 });
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
                    .when(self.card, |card| {
                        card.rounded_xl()
                            .border_1()
                            .border_color(border)
                            .bg(card_bg)
                            .px_6()
                            .py_5()
                            .shadow_lg()
                            .child(
                                div()
                                    .pb_3()
                                    .child(
                                        div()
                                            .text_base()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(foreground)
                                            .child("Subscription & Billing"),
                                    )
                                    .child(div().pt_1().text_sm().text_color(muted).child(
                                        "Common questions about your account, plans, \
                                                 payments and cancellations.",
                                    )),
                            )
                    })
                    .child(self.accordion.clone()),
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
        cx.new(|cx| AccordionDemo::new(&variant, dark, cx))
    })
    .expect("open accordion gallery");
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
    use super::{Accordion, AccordionEntry, Item, apply_open, navigate};

    fn items(values: &[&str], disabled: &[usize]) -> Vec<Item> {
        values
            .iter()
            .enumerate()
            .map(|(index, value)| Item {
                entry: AccordionEntry::new(*value, *value).disabled(disabled.contains(&index)),
                open: false,
            })
            .collect()
    }
    fn open_flags(items: &[Item]) -> Vec<bool> {
        items.iter().map(|item| item.open).collect()
    }

    #[test]
    fn single_mode_swaps_and_collapsible_controls_closing() {
        let mut items = items(&["a", "b"], &[]);
        apply_open(&mut items, false, true, 0, true);
        apply_open(&mut items, false, true, 1, true);
        assert_eq!(open_flags(&items), [false, true]);
        // Collapsible by default: the open entry can close again.
        apply_open(&mut items, false, true, 1, false);
        assert_eq!(open_flags(&items), [false, false]);
        // Non-collapsible keeps the last open entry.
        apply_open(&mut items, false, false, 0, true);
        assert!(!apply_open(&mut items, false, false, 0, false));
        assert_eq!(open_flags(&items), [true, false]);
    }

    #[test]
    fn multiple_mode_keeps_siblings_open_and_disabled_never_changes() {
        let mut items = items(&["a", "b", "c"], &[1]);
        apply_open(&mut items, true, true, 0, true);
        apply_open(&mut items, true, true, 2, true);
        assert_eq!(open_flags(&items), [true, false, true]);
        // Disabled entries ignore requests.
        assert!(!apply_open(&mut items, true, true, 1, true));
        apply_open(&mut items, true, true, 0, false);
        assert_eq!(open_flags(&items), [false, false, true]);
    }

    #[test]
    fn builder_marks_entries_open_and_disabled() {
        let accordion = Accordion::new()
            .entry(AccordionEntry::new("a", "A"))
            .entry(AccordionEntry::new("b", "B").disabled(true))
            .open("a")
            .open("missing");
        assert!(accordion.is_open("a"));
        assert!(!accordion.is_open("b"));
        assert!(!accordion.is_open("missing"));
    }

    #[test]
    fn keyboard_navigation_wraps_and_skips_disabled() {
        let enabled = [true, false, true];
        assert_eq!(navigate(&enabled, Some(0), "down"), Some(2));
        assert_eq!(navigate(&enabled, Some(2), "down"), Some(0));
        assert_eq!(navigate(&enabled, Some(0), "up"), Some(2));
        assert_eq!(navigate(&enabled, None, "down"), Some(0));
        assert_eq!(navigate(&enabled, None, "up"), Some(2));
        assert_eq!(navigate(&enabled, Some(1), "home"), Some(0));
        assert_eq!(navigate(&enabled, Some(0), "end"), Some(2));
        assert_eq!(navigate(&enabled, Some(0), "left"), None);
        assert_eq!(navigate(&[false, false], Some(0), "down"), None);
        assert_eq!(navigate(&[], None, "down"), None);
    }
}
