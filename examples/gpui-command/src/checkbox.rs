//! shadcn/ui-style checkbox for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/checkbox (React Aria).
//! The box is a focusable entity — click or Space toggles it; an optional
//! label/description render beside it Field-style. Emits CheckboxChangeEvent.
use gpui_kit::{self as kit, *};
use kit::base::{Easing, Transition, transition};
use std::borrow::Cow;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

const FONT: &str = "Geist";
/// Check mark fade.
const MARK_FADE: Duration = Duration::from_millis(120);

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Tri-state value, mirroring React Aria (`isIndeterminate`).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CheckboxState {
    Unchecked,
    Checked,
    /// Some-but-not-all selected (table select-all).
    Indeterminate,
}

/// Emitted when the checkbox is toggled by click or Space.
#[derive(Clone, Debug, PartialEq)]
pub struct CheckboxChangeEvent {
    pub checked: bool,
}

/// One checkbox — a 16px box plus optional label + description.
/// `cx.new(|_| Checkbox::new().label("Accept terms"))`.
pub struct Checkbox {
    state: CheckboxState,
    label: SharedString,
    description: SharedString,
    disabled: bool,
    invalid: bool,
    dark: bool,
    /// Stable per-instance key for the check-mark transition state.
    key: SharedString,
    focus: Option<FocusHandle>,
}
impl EventEmitter<CheckboxChangeEvent> for Checkbox {}

impl Checkbox {
    pub fn new() -> Self {
        let id = NEXT_KEY.fetch_add(1, Ordering::Relaxed);
        Self {
            state: CheckboxState::Unchecked,
            label: "".into(),
            description: "".into(),
            disabled: false,
            invalid: false,
            dark: false,
            key: format!("cb-{id}").into(),
            focus: None,
        }
    }
    /// FieldLabel text rendered next to the box; clicking it toggles too.
    pub fn label(mut self, label: &str) -> Self {
        self.label = label.into();
        self
    }
    /// FieldDescription helper text under the label.
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.into();
        self
    }
    /// Initial state (`defaultSelected` equivalent). For controlled updates
    /// use [`Checkbox::set_state`].
    pub fn checked(mut self, checked: bool) -> Self {
        self.state = if checked {
            CheckboxState::Checked
        } else {
            CheckboxState::Unchecked
        };
        self
    }
    pub fn indeterminate(mut self) -> Self {
        self.state = CheckboxState::Indeterminate;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// `isInvalid` — red border + label.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// Controlled update — e.g. a table's select-all driving row checkboxes.
    pub fn set_state(&mut self, state: CheckboxState, cx: &mut Context<Self>) {
        if self.state != state {
            self.state = state;
            cx.notify();
        }
    }

    fn toggle(&mut self, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        // React Aria: clicking an indeterminate box selects it.
        self.state = match self.state {
            CheckboxState::Checked => CheckboxState::Unchecked,
            CheckboxState::Unchecked | CheckboxState::Indeterminate => CheckboxState::Checked,
        };
        cx.emit(CheckboxChangeEvent {
            checked: self.state == CheckboxState::Checked,
        });
        cx.notify();
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.key.as_str() != "space" || self.disabled {
            return;
        }
        self.toggle(cx);
        window.prevent_default();
        cx.stop_propagation();
    }
}

/// The ✓ / — mark drawn inside the box.
fn mark(state: CheckboxState, color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.6 * k.max(0.7)));
            match state {
                CheckboxState::Checked => {
                    b.move_to(at(3.4, 8.4));
                    b.line_to(at(6.8, 11.8));
                    b.line_to(at(12.8, 4.6));
                }
                CheckboxState::Indeterminate => {
                    b.move_to(at(4., 8.));
                    b.line_to(at(12., 8.));
                }
                CheckboxState::Unchecked => return,
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

impl Render for Checkbox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let ink = rgb(if dark { 0xededed } else { 0x171717 });
        let muted = rgb(if dark { 0xa3a3a3 } else { 0x737373 });
        let danger = rgb(if dark { 0xf87171 } else { 0xdc2626 });
        let on_ink = rgb(if dark { 0x171717 } else { 0xffffff });

        let marked = self.state != CheckboxState::Unchecked;
        let mark_t = if cx.reduce_motion() {
            1f32
        } else {
            transition(
                ("cb-mark", self.key.clone()),
                if marked { 1f32 } else { 0f32 },
                Transition::new(MARK_FADE).easing(Easing::EaseOut),
                window,
                cx,
            )
        };

        // Box: bordered square; checked/indeterminate fills with primary ink.
        let edge: Hsla = if self.invalid {
            danger.into()
        } else {
            rgb(if dark { 0x525252 } else { 0xd4d4d4 }).into()
        };
        let mut box_el = div()
            .flex_none()
            .size(px(16.))
            .rounded(px(4.))
            .border_1()
            .border_color(edge)
            .bg(rgb(0).opacity(0.));
        if marked && mark_t > 0.01 {
            box_el = box_el.bg(ink).border_color(ink).child(
                div()
                    .size_full()
                    .opacity(mark_t)
                    .child(mark(self.state, on_ink.into())),
            );
        }

        // Label column (Field + FieldLabel + FieldDescription).
        let mut row = div().flex().items_start().gap_2p5().font_family(FONT);
        if !self.label.is_empty() || !self.description.is_empty() {
            let mut col = div()
                .flex()
                .flex_col()
                .gap_1()
                .pt(px(-1.))
                .text_sm()
                .line_height(px(20.));
            if !self.label.is_empty() {
                let label_color: Hsla = if self.invalid {
                    danger.into()
                } else {
                    ink.into()
                };
                col = col.child(
                    div()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(label_color)
                        .child(self.label.clone()),
                );
            }
            if !self.description.is_empty() {
                col = col.child(
                    div()
                        .text_xs()
                        .text_color(muted)
                        .child(self.description.clone()),
                );
            }
            row = row.child(box_el).child(col);
        } else {
            row = row.child(box_el);
        }

        if self.disabled {
            return row.opacity(0.5).into_any_element();
        }

        let focus = self.focus.get_or_insert_with(|| cx.focus_handle()).clone();
        // focus-visible semantics: the ring appears only for keyboard focus.
        let focused = focus.is_focused(window) && window.last_input_was_keyboard();
        let mut row = row.relative().id(self.key.clone());
        if focused {
            // focus-visible ring around the box.
            row = row.child(
                div()
                    .absolute()
                    .w(px(20.))
                    .h(px(20.))
                    .top(px(-2.))
                    .left(px(-2.))
                    .rounded(px(6.))
                    .border_2()
                    .border_color(rgb(0x0285f7).opacity(0.6)),
            );
        }
        let focus_for_click = focus.clone();
        row.track_focus(&focus.tab_index(0))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                focus_for_click.focus(window, cx);
                window.prevent_default();
            })
            .on_click(cx.listener(|this, _, _, cx| this.toggle(cx)))
            .on_key_down(cx.listener(Self::key_down))
            .into_any_element()
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

pub struct CheckboxDemo {
    boxes: Vec<Entity<Checkbox>>,
    header: Option<Entity<Checkbox>>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl CheckboxDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let d = |c: Checkbox| c.dark(dark);
        let mk = |cx: &mut Context<Self>, c: Checkbox| cx.new(|_| c);
        let (boxes, header) = match variant {
            "checked" => (
                vec![mk(
                    cx,
                    d(Checkbox::new()
                        .label("Enable notifications")
                        .description("You can enable or disable notifications at any time.")
                        .checked(true)),
                )],
                None,
            ),
            "invalid" => (
                vec![mk(
                    cx,
                    d(Checkbox::new()
                        .label("Accept terms and conditions")
                        .invalid(true)),
                )],
                None,
            ),
            "disabled" => (
                vec![
                    mk(
                        cx,
                        d(Checkbox::new().label("Enable notifications").disabled(true)),
                    ),
                    mk(
                        cx,
                        d(Checkbox::new()
                            .label("Enabled and checked")
                            .checked(true)
                            .disabled(true)),
                    ),
                ],
                None,
            ),
            "group" => (
                vec![
                    mk(cx, d(Checkbox::new().label("Hard disks"))),
                    mk(cx, d(Checkbox::new().label("External disks"))),
                    mk(cx, d(Checkbox::new().label("CDs, DVDs, and iPods"))),
                    mk(
                        cx,
                        d(Checkbox::new().label("Connected servers").checked(true)),
                    ),
                ],
                None,
            ),
            "table" => {
                let rows = vec![
                    mk(cx, d(Checkbox::new().checked(true))),
                    mk(cx, d(Checkbox::new())),
                    mk(cx, d(Checkbox::new())),
                    mk(cx, d(Checkbox::new())),
                ];
                let header = mk(cx, d(Checkbox::new().indeterminate()));
                (rows, Some(header))
            }
            _ => (
                vec![mk(
                    cx,
                    d(Checkbox::new()
                        .label("Accept terms and conditions")
                        .description("By clicking this checkbox, you agree to the terms.")),
                )],
                None,
            ),
        };

        let mut subscriptions = Vec::new();
        for (i, entity) in boxes.iter().enumerate() {
            let header = header.clone();
            let rows = boxes.clone();
            subscriptions.push(cx.subscribe(
                entity,
                move |demo, _, ev: &CheckboxChangeEvent, cx| {
                    demo.outcome = Some(format!("Row {} → {}", i + 1, ev.checked).into());
                    if let Some(header) = &header {
                        let states: Vec<CheckboxState> =
                            rows.iter().map(|r| r.read(cx).state).collect();
                        let all = states.iter().all(|s| *s == CheckboxState::Checked);
                        let none = states.iter().all(|s| *s == CheckboxState::Unchecked);
                        let st = if all {
                            CheckboxState::Checked
                        } else if none {
                            CheckboxState::Unchecked
                        } else {
                            CheckboxState::Indeterminate
                        };
                        let _ = header.update(cx, |h, cx| h.set_state(st, cx));
                    }
                    cx.notify();
                },
            ));
        }
        if let Some(header) = &header {
            let rows = boxes.clone();
            subscriptions.push(
                cx.subscribe(header, move |_, _, ev: &CheckboxChangeEvent, cx| {
                    let st = if ev.checked {
                        CheckboxState::Checked
                    } else {
                        CheckboxState::Unchecked
                    };
                    for r in &rows {
                        let _ = r.update(cx, |c, cx| c.set_state(st, cx));
                    }
                }),
            );
        }
        Self {
            boxes,
            header,
            dark,
            outcome: None,
            _subscriptions: subscriptions,
        }
    }
}

const PEOPLE: [(&str, &str, &str); 4] = [
    ("Sarah Chen", "sarah.chen@example.com", "Admin"),
    ("Marcus Rodriguez", "marcus.rodriguez@example.com", "User"),
    ("Priya Patel", "priya.patel@example.com", "User"),
    ("David Kim", "david.kim@example.com", "Editor"),
];

impl Render for CheckboxDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let d = self.dark;
        let surface = rgb(if d { 0x202020 } else { 0xf8f8f8 });
        let card = rgb(if d { 0x232323 } else { 0xffffff });
        let ink = rgb(if d { 0xededed } else { 0x171717 });
        let muted = rgb(if d { 0xa3a3a3 } else { 0x737373 });
        let edge = rgb(if d { 0x3b3b3b } else { 0xe4e4e4 });

        let content: AnyElement = if let Some(header) = &self.header {
            // ── Table: header select-all + one box per row ────────────────
            let cell = |w: f32| div().flex().w(px(w)).flex_none();
            let head_cell = |text: &str, w: f32| {
                cell(w)
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(muted)
                    .child(SharedString::from(text.to_string()))
            };
            let mut table = div()
                .w(px(520.))
                .rounded_lg()
                .border_1()
                .border_color(edge)
                .bg(card)
                .overflow_hidden()
                .flex()
                .flex_col()
                .font_family(FONT);
            table = table.child(
                div()
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_2p5()
                    .border_b_1()
                    .border_color(edge)
                    .child(header.clone())
                    .child(head_cell("Name", 160.))
                    .child(head_cell("Email", 220.))
                    .child(head_cell("Role", 80.)),
            );
            for (i, (name, email, role)) in PEOPLE.iter().enumerate() {
                let mut row = div().flex().items_center().gap_3().px_4().py_3();
                if i > 0 {
                    row = row.border_t_1().border_color(edge);
                }
                table = table.child(
                    row.child(self.boxes[i].clone())
                        .child(
                            cell(160.)
                                .text_sm()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(ink)
                                .child(SharedString::from(name.to_string())),
                        )
                        .child(
                            cell(220.)
                                .text_sm()
                                .text_color(muted)
                                .child(SharedString::from(email.to_string())),
                        )
                        .child(
                            cell(80.)
                                .text_sm()
                                .text_color(muted)
                                .child(SharedString::from(role.to_string())),
                        ),
                );
            }
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_3()
                .child(table)
                .child(
                    div()
                        .h(px(18.))
                        .text_xs()
                        .text_color(muted)
                        .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
                )
                .into_any_element()
        } else {
            let mut col = div()
                .flex()
                .flex_col()
                .gap_4()
                .w_full()
                .max_w(px(340.))
                .font_family(FONT);
            for (i, b) in self.boxes.iter().enumerate() {
                let mut cell = div();
                if i > 0 {
                    cell = cell.pt_3().border_t_1().border_color(edge);
                }
                col = col.child(cell.child(b.clone()));
            }
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_3()
                .w_full()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .rounded_lg()
                        .border_1()
                        .border_color(edge)
                        .bg(card)
                        .p_5()
                        .w(px(340.))
                        .child(col),
                )
                .child(
                    div()
                        .h(px(18.))
                        .text_xs()
                        .text_color(muted)
                        .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
                )
                .into_any_element()
        };
        let _ = cx;
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(content)
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
        cx.new(|cx| CheckboxDemo::new(&variant, dark, cx))
    })
    .expect("open checkbox gallery");
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
    cx.new(|cx| CheckboxDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{Checkbox, CheckboxState};

    #[test]
    fn builders_store_state() {
        let c = Checkbox::new()
            .label("Accept")
            .description("desc")
            .checked(true)
            .disabled(true)
            .invalid(true)
            .dark(true);
        assert_eq!(c.state, CheckboxState::Checked);
        assert_eq!(c.label.as_ref(), "Accept");
        assert!(c.disabled && c.invalid && c.dark);
    }

    #[test]
    fn indeterminate_builder() {
        assert_eq!(
            Checkbox::new().indeterminate().state,
            CheckboxState::Indeterminate
        );
    }
}
