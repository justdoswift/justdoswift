//! shadcn/ui-style combobox for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/combobox (React Aria).
//! One entity owns the query input, the filtered collection, selection and the
//! popup. Keyboard semantics come from gpui-base's `Combobox` root — its
//! "Combobox" key context binds up/down/enter/escape, and single-line inputs
//! don't register MoveUp/MoveDown handlers so the arrows bubble through.
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use kit::base::Combobox as ComboboxRoot;
use kit::base::actions::{SelectDown, SelectUp};
use kit::base::input::{
    Enter as InputEnter, Escape as InputEscape, Input, InputBase, InputEditorStyle, InputEvent,
    InputState, MoveDown as InputMoveDown, MoveUp as InputMoveUp,
};
use std::borrow::Cow;
use std::sync::atomic::{AtomicUsize, Ordering};

const FONT: &str = "Geist";

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Emitted whenever the committed selection changes.
#[derive(Clone, Debug, PartialEq)]
pub struct ComboboxChangeEvent {
    /// Newest committed value (single select) or `None` for multi toggles.
    pub value: Option<SharedString>,
    /// The full selection after the change.
    pub selected: Vec<SharedString>,
}

/// One option in the collection (`ComboboxItem`).
#[derive(Clone, Debug)]
pub struct ComboboxItem {
    pub value: SharedString,
    pub label: SharedString,
    /// Group header — a label row is rendered when the group changes.
    pub group: Option<SharedString>,
    /// Quiet trailing text (custom items).
    pub hint: Option<SharedString>,
    pub disabled: bool,
}

impl ComboboxItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label: SharedString = label.into();
        Self {
            value: label.clone(),
            label,
            group: None,
            hint: None,
            disabled: false,
        }
    }
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }
    pub fn group(mut self, group: impl Into<SharedString>) -> Self {
        self.group = Some(group.into());
        self
    }
    pub fn hint(mut self, hint: impl Into<SharedString>) -> Self {
        self.hint = Some(hint.into());
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// A combobox — text input + filtered popup list. Create the `InputState`
/// first (it needs a `Window`), then `cx.new(|cx| Combobox::new(input, cx))`.
pub struct Combobox {
    input: Entity<InputState>,
    items: Vec<ComboboxItem>,
    /// Indices into `items` matching the current query, in display order.
    filtered: Vec<usize>,
    /// Highlighted position inside `filtered`; `None` before the user picks.
    active: Option<usize>,
    /// Committed selection — single uses `selected`, multi uses `picked`.
    selected: Option<usize>,
    picked: Vec<usize>,
    open: bool,
    multiple: bool,
    clearable: bool,
    invalid: bool,
    disabled: bool,
    dark: bool,
    empty_text: SharedString,
    focus: FocusHandle,
    scroll: ScrollHandle,
    id: usize,
    _subscriptions: Vec<Subscription>,
    _interceptor: Option<Subscription>,
}
impl EventEmitter<ComboboxChangeEvent> for Combobox {}

impl Combobox {
    pub fn new(input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let id = NEXT_KEY.fetch_add(1, Ordering::Relaxed);
        let input_sub =
            cx.subscribe(
                &input,
                |this: &mut Self, _, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.refilter_with(cx);
                        this.open = true;
                        this.active = if this.filtered.is_empty() {
                            None
                        } else {
                            Some(0)
                        };
                        cx.notify();
                    }
                    InputEvent::Blur => {
                        this.open = false;
                        cx.notify();
                    }
                    _ => {}
                },
            );
        // Backspace on an empty input removes the last chip — the input itself
        // consumes Backspace, so intercept the keystroke before dispatch.
        let weak = cx.entity().downgrade();
        let interceptor = cx.intercept_keystrokes(move |event, _window, cx| {
            if event.keystroke.key != "backspace"
                || event.keystroke.modifiers.control
                || event.keystroke.modifiers.alt
                || event.keystroke.modifiers.platform
            {
                return;
            }
            let Some(this) = weak.upgrade() else { return };
            let can_pop = this.read(cx).multiple
                && this.read(cx).input.read(cx).value().is_empty()
                && !this.read(cx).picked.is_empty()
                && this.read(cx).input.focus_handle(cx).is_focused(_window);
            if !can_pop {
                return;
            }
            this.update(cx, |this, cx| {
                this.picked.pop();
                this.emit_change(cx);
            });
            cx.stop_propagation();
        });
        Self {
            input,
            items: Vec::new(),
            filtered: Vec::new(),
            active: None,
            selected: None,
            picked: Vec::new(),
            open: false,
            multiple: false,
            clearable: false,
            invalid: false,
            disabled: false,
            dark: false,
            empty_text: "No results found.".into(),
            focus: cx.focus_handle(),
            scroll: ScrollHandle::new(),
            id,
            _subscriptions: vec![input_sub],
            _interceptor: Some(interceptor),
        }
    }

    /// The suggestion collection (`ComboboxItem` list).
    pub fn items(mut self, items: Vec<ComboboxItem>) -> Self {
        self.filtered = (0..items.len()).filter(|i| !items[*i].disabled).collect();
        self.items = items;
        self
    }
    /// Multi-select — selections render as chips inside the field.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }
    /// Show an × button that clears the query while it has text.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }
    /// Invalid styling — destructive border and ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    /// `ComboboxEmpty` content — shown when the query matches nothing.
    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }
    /// Initial selection by value (`defaultValue` equivalent). For controlled
    /// updates use [`Combobox::set_value`].
    pub fn default_value(mut self, value: &str) -> Self {
        if let Some(i) = self.items.iter().position(|item| item.value == *value) {
            self.selected = Some(i);
        }
        self
    }
    /// Initial multi-selection by values.
    pub fn selected_values(mut self, values: &[&str]) -> Self {
        self.picked = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| values.iter().any(|v| *v == item.value.as_ref()))
            .map(|(i, _)| i)
            .collect();
        self
    }

    pub fn selected_value(&self) -> Option<SharedString> {
        self.selected.map(|i| self.items[i].value.clone())
    }
    pub fn selected_values_vec(&self) -> Vec<SharedString> {
        self.picked
            .iter()
            .map(|&i| self.items[i].value.clone())
            .collect()
    }
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Controlled update — single-select `value` / `onValueChange` equivalent.
    pub fn set_value(&mut self, value: Option<&str>, window: &mut Window, cx: &mut Context<Self>) {
        self.selected = value.and_then(|v| self.items.iter().position(|i| i.value == *v));
        let text = self
            .selected
            .map(|i| self.items[i].label.clone())
            .unwrap_or_default();
        let input = self.input.clone();
        input.update(cx, |input, cx| input.set_value(text, window, cx));
        self.refilter_with(cx);
        cx.notify();
    }

    /// Controlled update for multi-select (`selected` equivalent).
    pub fn set_selected(&mut self, values: &[&str], cx: &mut Context<Self>) {
        self.picked = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| values.iter().any(|v| *v == item.value.as_ref()))
            .map(|(i, _)| i)
            .collect();
        cx.notify();
    }

    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        if self.open != open {
            self.open = open;
            if open {
                self.active = None;
            }
            cx.notify();
        }
    }

    /// Recompute `filtered` from the query. A query that is exactly the
    /// committed label shows the full collection — the list is not reduced
    /// to just the selected row after it is filled in.
    fn emit_change(&self, cx: &mut Context<Self>) {
        let selected: Vec<SharedString> = if self.multiple {
            self.picked
                .iter()
                .map(|&i| self.items[i].value.clone())
                .collect()
        } else {
            self.selected
                .iter()
                .map(|&i| self.items[i].value.clone())
                .collect()
        };
        cx.emit(ComboboxChangeEvent {
            value: selected.last().cloned(),
            selected,
        });
    }

    fn commit_active(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Fall back to the first row when nothing was arrow-keyed to.
        let position = self.active.or(if self.open && !self.filtered.is_empty() {
            Some(0)
        } else {
            None
        });
        if let Some(position) = position {
            if let Some(&index) = self.filtered.get(position) {
                self.commit_index(index, window, cx);
            }
        }
    }

    /// Escape while the popup is open: close and restore the committed label.
    fn dismiss(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.multiple {
            let text = self
                .selected
                .map(|i| self.items[i].label.clone())
                .unwrap_or_default();
            let input = self.input.clone();
            input.update(cx, |input, cx| input.set_value(text, window, cx));
            self.refilter_with(cx);
        }
        self.set_open(false, cx);
    }

    fn commit_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if self.items[index].disabled {
            return;
        }
        if self.multiple {
            if let Some(pos) = self.picked.iter().position(|&i| i == index) {
                self.picked.remove(pos);
            } else {
                self.picked.push(index);
            }
            let input = self.input.clone();
            input.update(cx, |input, cx| {
                input.set_value("", window, cx);
                input.focus(window, cx);
            });
            self.refilter_with(cx);
            self.active = if self.filtered.is_empty() {
                None
            } else {
                Some(0)
            };
            self.emit_change(cx);
            cx.notify();
        } else {
            self.selected = Some(index);
            let label = self.items[index].label.clone();
            let input = self.input.clone();
            input.update(cx, |input, cx| {
                input.set_value(label, window, cx);
                input.select_all(window, cx);
                input.focus(window, cx);
            });
            self.open = false;
            self.emit_change(cx);
            cx.notify();
        }
    }

    /// Post-input refilter that can read the query through `cx`.
    fn refilter_with(&mut self, cx: &App) {
        let query = self.input.read(cx).value().to_string();
        let committed_label = self
            .selected
            .filter(|_| !self.multiple)
            .map(|i| self.items[i].label.to_lowercase());
        self.filtered = filter_items(&self.items, &query, committed_label);
    }

    fn move_active(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.filtered.is_empty() {
            return;
        }
        let len = self.filtered.len() as isize;
        self.active = Some(match self.active {
            None => 0,
            Some(i) => ((i as isize + delta + len) % len) as usize,
        });
        if let Some(i) = self.active {
            self.scroll.scroll_to_item(i);
        }
        cx.notify();
    }

    fn remove_picked(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(pos) = self.picked.iter().position(|&i| i == index) {
            self.picked.remove(pos);
            self.emit_change(cx);
            cx.notify();
        }
    }
}

/// Case-insensitive substring match over label, value and group. An empty
/// query — or one that equals the committed label — returns every row, so
/// reopening a filled input still lists the whole collection.
fn filter_items(
    items: &[ComboboxItem],
    query: &str,
    committed_label: Option<String>,
) -> Vec<usize> {
    let query = query.trim().to_lowercase();
    let show_all = query.is_empty() || Some(query.clone()) == committed_label;
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            show_all
                || item.label.to_lowercase().contains(&query)
                || item.value.to_lowercase().contains(&query)
                || item
                    .group
                    .as_ref()
                    .is_some_and(|g| g.to_lowercase().contains(&query))
        })
        .map(|(i, _)| i)
        .collect()
}

/// Check glyph drawn in the selection slot of a list row.
fn check_glyph(color: Hsla) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 14.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.6 * k.max(0.8)));
            b.move_to(at(3., 7.2));
            b.line_to(at(6., 10.4));
            b.line_to(at(11., 3.6));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size_full()
}

/// Chevron-down glyph for the popup trigger button.
fn chevron_down(color: Hsla) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let center = bounds.center();
            let mut b = PathBuilder::stroke(px(1.5));
            b.move_to(center + point(px(-4.5), px(-1.6)));
            b.line_to(center + point(px(0.), px(2.4)));
            b.line_to(center + point(px(4.5), px(-1.6)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size_full()
}

/// × glyph for the clear button and chip remove.
fn x_glyph(color: Hsla) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _| {
            let c = bounds.center();
            let mut b = PathBuilder::stroke(px(1.4));
            b.move_to(c + point(px(-3.4), px(-3.4)));
            b.line_to(c + point(px(3.4), px(3.4)));
            b.move_to(c + point(px(3.4), px(-3.4)));
            b.line_to(c + point(px(-3.4), px(3.4)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size_full()
}

/// One display row in the open list — group headers interleave items.
enum ListRow {
    Group(SharedString),
    Item(usize),
    Separator,
}

impl Combobox {
    /// Flatten `filtered` into rows, inserting a group label when the item's
    /// group changes and a separator between groups.
    fn rows(&self) -> Vec<ListRow> {
        let mut rows = Vec::new();
        let mut last_group: Option<&SharedString> = None;
        for &index in &self.filtered {
            let group = self.items[index].group.as_ref();
            if group != last_group {
                if last_group.is_some() {
                    rows.push(ListRow::Separator);
                }
                if let Some(g) = group {
                    rows.push(ListRow::Group(g.clone()));
                }
                last_group = group;
            }
            rows.push(ListRow::Item(index));
        }
        rows
    }
}

impl Render for Combobox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
        let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
        let field_bg: Hsla = rgb(if dark { 0x1c1c1c } else { 0xffffff }).into();
        let popover: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();
        let accent_bg: Hsla = rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into();
        let chip_bg: Hsla = rgb(if dark { 0x2c2c2c } else { 0xececec }).into();
        let destructive: Hsla = rgb(if dark { 0xef4444 } else { 0xdc2626 }).into();
        let border = if self.invalid { destructive } else { edge };

        let input_focus = self.input.focus_handle(cx);
        let has_query = !self.input.read(cx).value().is_empty();

        // ── Field ────────────────────────────────────────────────────────
        // Chips + input + controls inside one bordered row.
        let focused = input_focus.is_focused(window);
        let mut field = InputBase::new(("combobox-field", self.id))
            .focused(focused)
            .styles(|s| {
                s.focused(|f| {
                    f.border_color(if self.invalid {
                        destructive
                    } else {
                        rgb(if dark { 0x9c9c9c } else { 0xa3a3a3 }).into()
                    })
                })
            })
            .flex()
            .items_center()
            .flex_wrap()
            .gap_1p5()
            .w_full()
            .min_h(px(38.))
            .px_3()
            .py(px(5.))
            .rounded_md()
            .border_1()
            .border_color(border)
            .bg(field_bg)
            .text_sm()
            .text_color(ink)
            .font_family(FONT);

        if self.disabled {
            field = field.opacity(0.5);
        } else {
            let input = self.input.clone();
            field = field
                .cursor_text()
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    input.update(cx, |input, cx| input.focus(window, cx));
                });
        }

        if self.multiple {
            for &index in &self.picked {
                let label = self.items[index].label.clone();
                let mut chip = div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    .pl(px(10.))
                    .pr(px(6.))
                    .h(px(24.))
                    .rounded_md()
                    .bg(chip_bg)
                    .text_xs()
                    .text_color(ink)
                    .child(label);
                if !self.disabled {
                    chip = chip.child(
                        div()
                            .id(("combobox-chip", index))
                            .flex_none()
                            .size(px(12.))
                            .cursor_pointer()
                            .hover(|s| s.text_color(destructive))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                    this.remove_picked(index, cx);
                                }),
                            )
                            .child(x_glyph(muted)),
                    );
                }
                field = field.child(chip);
            }
        }

        field = field.child(div().flex_1().min_w(px(96.)).child(Input::new(&self.input)));

        if self.clearable && has_query && !self.disabled {
            field = field.child(
                div()
                    .id(("combobox-clear", self.id))
                    .flex_none()
                    .size(px(18.))
                    .rounded_sm()
                    .cursor_pointer()
                    .hover(|s| s.bg(accent_bg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            let input = this.input.clone();
                            input.update(cx, |input, cx| {
                                input.set_value("", window, cx);
                                input.focus(window, cx);
                            });
                        }),
                    )
                    .child(x_glyph(muted)),
            );
        }

        if !self.disabled {
            field = field.child(
                div()
                    .id(("combobox-toggle", self.id))
                    .flex_none()
                    .size(px(18.))
                    .rounded_sm()
                    .cursor_pointer()
                    .hover(|s| s.bg(accent_bg))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            this.set_open(!this.open, cx);
                            let input = this.input.clone();
                            input.update(cx, |input, cx| input.focus(window, cx));
                        }),
                    )
                    .child(chevron_down(muted)),
            );
        }

        // ── Popup (ComboboxContent → ComboboxList) ──────────────────────
        let mut popup = div();
        if self.open {
            let mut list = div().flex().flex_col();
            if self.filtered.is_empty() {
                list = list.child(
                    div()
                        .py_5()
                        .text_center()
                        .text_sm()
                        .text_color(muted)
                        .child(self.empty_text.clone()),
                );
            } else {
                for row in self.rows() {
                    match row {
                        ListRow::Group(g) => {
                            list = list.child(
                                div()
                                    .px_2()
                                    .pt_2()
                                    .pb_1()
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(muted)
                                    .child(g),
                            );
                        }
                        ListRow::Separator => {
                            list = list.child(div().h(px(1.)).bg(edge).mx(px(-4.)).my_1());
                        }
                        ListRow::Item(index) => {
                            let item = &self.items[index];
                            let position =
                                self.filtered.iter().position(|&i| i == index).unwrap_or(0);
                            let is_active = self.active == Some(position);
                            let is_selected = if self.multiple {
                                self.picked.contains(&index)
                            } else {
                                self.selected == Some(index)
                            };
                            let item_row = div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .px_2()
                                .py(px(6.))
                                .mx_1()
                                .rounded_sm()
                                .text_sm()
                                .text_color(ink)
                                .child(div().flex_1().min_w_0().child(item.label.clone()))
                                .when_some(item.hint.clone(), |row, hint| {
                                    row.child(
                                        div().flex_none().text_xs().text_color(muted).child(hint),
                                    )
                                })
                                .child(div().flex_none().size(px(14.)).child(if is_selected {
                                    check_glyph(ink).into_any_element()
                                } else {
                                    div().into_any_element()
                                }));
                            list = list.child(if item.disabled {
                                item_row.opacity(0.5).into_any_element()
                            } else {
                                item_row
                                    .id(("combobox-item", index))
                                    .cursor_pointer()
                                    .when(is_active, |row| row.bg(accent_bg))
                                    .hover(move |s| s.bg(accent_bg))
                                    .on_mouse_move(cx.listener(move |this, _, _, cx| {
                                        if this.active != Some(position) {
                                            this.active = Some(position);
                                            cx.notify();
                                        }
                                    }))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _, window, cx| {
                                            window.prevent_default();
                                            cx.stop_propagation();
                                            this.commit_index(index, window, cx);
                                        }),
                                    )
                                    .into_any_element()
                            });
                        }
                    }
                }
            }
            // deferred() paints the popup in the overlay pass, after the
            // field/card borders — otherwise those borders draw on top of it.
            popup = popup.child(deferred(
                div()
                    .absolute()
                    .top(relative(1.))
                    .mt_1()
                    .left_0()
                    .w_full()
                    .rounded_md()
                    .border_1()
                    .border_color(edge)
                    .bg(popover)
                    .shadow_md()
                    .py_1()
                    .child(
                        div()
                            .id(("combobox-list", self.id))
                            .max_h(px(196.))
                            .overflow_y_scroll()
                            .track_scroll(&self.scroll)
                            .child(list),
                    ),
            ));
        }

        let weak = cx.entity().downgrade();
        div()
            .w_full()
            .font_family(FONT)
            // Two action paths drive the same semantics:
            // - SelectDown/SelectUp: the "Combobox" key context when focus is
            //   on the trigger shell itself (input not focused).
            // - input::MoveDown/MoveUp: the deeper "Input" context wins the
            //   binding, so these bubble up from the focused single-line
            //   input, which registers no vertical-move handlers.
            .on_action(cx.listener(|this, _: &SelectDown, _, cx| this.move_active(1, cx)))
            .on_action(cx.listener(|this, _: &SelectUp, _, cx| this.move_active(-1, cx)))
            .on_action(cx.listener(|this, _: &InputMoveDown, window, cx| {
                if this.disabled {
                    cx.propagate();
                    return;
                }
                let _ = window;
                if !this.open {
                    this.set_open(true, cx);
                }
                this.move_active(1, cx);
            }))
            .on_action(cx.listener(|this, _: &InputMoveUp, window, cx| {
                if this.disabled {
                    cx.propagate();
                    return;
                }
                let _ = window;
                if !this.open {
                    this.set_open(true, cx);
                }
                this.move_active(-1, cx);
            }))
            .on_action(cx.listener(|this, _: &InputEnter, window, cx| {
                if this.disabled {
                    cx.propagate();
                    return;
                }
                this.commit_active(window, cx);
            }))
            .on_action(cx.listener(|this, _: &InputEscape, window, cx| {
                if !this.open {
                    cx.propagate();
                    return;
                }
                this.dismiss(window, cx);
            }))
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.set_open(false, cx);
            }))
            .child(
                div().relative().w_full().child(
                    ComboboxRoot::new(("combobox", self.id))
                        .open(self.open)
                        .disabled(self.disabled)
                        .focus_handle(&self.focus)
                        .content_focus_handle(&input_focus)
                        .on_open_change({
                            let weak = weak.clone();
                            move |open, _window, cx| {
                                let _ = weak.update_in(cx, |this, _window, cx| {
                                    this.set_open(open, cx);
                                });
                            }
                        })
                        .on_confirm({
                            let weak = weak.clone();
                            move |_window, cx| {
                                let _ = weak.update_in(cx, |this, window, cx| {
                                    this.commit_active(window, cx);
                                });
                            }
                        })
                        .on_dismiss({
                            move |_window, cx| {
                                let _ = weak.update_in(cx, |this, window, cx| {
                                    // Restore the committed label when the
                                    // popup is dismissed mid-edit.
                                    if !this.multiple {
                                        let text = this
                                            .selected
                                            .map(|i| this.items[i].label.clone())
                                            .unwrap_or_default();
                                        let input = this.input.clone();
                                        input.update(cx, |input, cx| {
                                            input.set_value(text, window, cx)
                                        });
                                        this.refilter_with(cx);
                                    }
                                });
                            }
                        })
                        .child(field)
                        .child(popup),
                ),
            )
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

fn palette(dark: bool) -> (Hsla, Hsla, Hsla, Hsla, Hsla) {
    (
        rgb(if dark { 0xededed } else { 0x171717 }).into(), // ink
        rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(), // muted
        rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(), // edge
        rgb(if dark { 0x232323 } else { 0xffffff }).into(), // card
        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into(), // surface
    )
}

fn editor_style(dark: bool) -> InputEditorStyle {
    InputEditorStyle {
        foreground: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted_foreground: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        caret: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        selection: rgba(if dark { 0xffffff2e } else { 0x1717171f }).into(),
        ..Default::default()
    }
}

/// Create the shared query input for a demo combobox.
fn query_input(
    placeholder: &str,
    dark: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState> {
    cx.new(|cx| {
        let mut state = InputState::new(window, cx).placeholder(placeholder.to_string());
        state.set_editor_style(editor_style(dark));
        state
    })
}

fn frameworks() -> Vec<ComboboxItem> {
    ["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"]
        .into_iter()
        .map(ComboboxItem::new)
        .collect()
}

fn grouped_items() -> Vec<ComboboxItem> {
    [
        ComboboxItem::new("useMediaQuery").group("Hooks"),
        ComboboxItem::new("useDebounce").group("Hooks"),
        ComboboxItem::new("useLocalStorage").group("Hooks"),
        ComboboxItem::new("cn").group("Utils"),
        ComboboxItem::new("formatDate").group("Utils"),
        ComboboxItem::new("capitalize").group("Utils"),
        ComboboxItem::new("slugify").group("Utils"),
    ]
    .into()
}

fn assignees() -> Vec<ComboboxItem> {
    [
        ("shadcn", "@shadcn"),
        ("maxleiter", "@maxleiter"),
        ("evilrabbit", "@evilrabbit"),
        ("paco", "@paco"),
        ("leerob", "@leerob"),
    ]
    .iter()
    .map(|(label, hint)| ComboboxItem::new(*label).hint(*hint))
    .collect()
}

pub struct ComboboxDemo {
    box_: Entity<Combobox>,
    dark: bool,
    variant: SharedString,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl ComboboxDemo {
    fn new(variant: &str, dark: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut subs: Vec<Subscription> = Vec::new();
        let box_: Entity<Combobox> = match variant {
            "multiple" => {
                let input = query_input("Select frameworks…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(frameworks())
                        .multiple(true)
                        .selected_values(&["Next.js"])
                        .empty_text("No framework found.")
                        .dark(dark)
                })
            }
            "clearable" => {
                let input = query_input("Select framework…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(frameworks())
                        .clearable(true)
                        .empty_text("No framework found.")
                        .dark(dark)
                })
            }
            "groups" => {
                let input = query_input("Search utilities…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(grouped_items())
                        .empty_text("No utility found.")
                        .dark(dark)
                })
            }
            "custom" => {
                let input = query_input("Assign to…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(assignees())
                        .empty_text("No assignee found.")
                        .dark(dark)
                })
            }
            "invalid" => {
                let input = query_input("Select framework…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(frameworks())
                        .invalid(true)
                        .empty_text("No framework found.")
                        .dark(dark)
                })
            }
            "disabled" => {
                let input = query_input("Select framework…", dark, window, cx);
                input.update(cx, |input, cx| input.set_disabled(true, cx));
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(frameworks())
                        .default_value("SvelteKit")
                        .disabled(true)
                        .dark(dark)
                })
            }
            "empty" => {
                let input = query_input("Pick a number…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(Vec::new())
                        .empty_text("This collection is empty.")
                        .dark(dark)
                })
            }
            _ => {
                let input = query_input("Select framework…", dark, window, cx);
                cx.new(|cx| {
                    Combobox::new(input, cx)
                        .items(frameworks())
                        .empty_text("No framework found.")
                        .dark(dark)
                })
            }
        };
        subs.push(
            cx.subscribe(&box_, |demo, _, ev: &ComboboxChangeEvent, cx| {
                demo.outcome = Some(
                    if ev.selected.is_empty() {
                        "Selection cleared".to_string()
                    } else {
                        format!("Selected: {}", ev.selected.join(", "))
                    }
                    .into(),
                );
                cx.notify();
            }),
        );
        Self {
            box_,
            dark,
            variant: variant.to_string().into(),
            outcome: None,
            _subscriptions: subs,
        }
    }
}

impl Render for ComboboxDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, edge, card, surface) = palette(self.dark);
        let wide = self.variant == "multiple";

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_3()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(
                div()
                    .w(if wide { px(360.) } else { px(320.) })
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .w_full()
                            .rounded_lg()
                            .border_1()
                            .border_color(edge)
                            .bg(card)
                            .p_4()
                            .child(self.box_.clone()),
                    )
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(480.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |window, cx| {
        cx.new(|cx| ComboboxDemo::new(&variant, dark, window, cx))
    })
    .expect("open combobox gallery");
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
    // `gpui` gives the test macro's expansion a real crate path; a glob
    // gpui_kit import would shadow #[test] with gpui::test.
    use super::{Combobox, ComboboxItem, filter_items, frameworks, grouped_items};
    use gpui_kit::base::input::InputState;
    use gpui_kit::gpui;
    use gpui_kit::gpui::{AppContext, Context, Entity, TestAppContext, VisualTestContext};

    fn combobox(
        cx: &mut TestAppContext,
        build: impl FnOnce(Entity<InputState>, &mut Context<Combobox>) -> Combobox,
    ) -> (Entity<Combobox>, &mut VisualTestContext) {
        cx.update(gpui_kit::base::init);
        cx.add_window_view(move |window, cx| {
            let input = cx.new(|cx| InputState::new(window, cx));
            build(input, cx)
        })
    }

    #[gpui::test]
    fn typing_filters_and_enter_commits(cx: &mut TestAppContext) {
        let (entity, cx) = combobox(cx, |input, cx| Combobox::new(input, cx).items(frameworks()));
        // Focus the input and type — Change event filters + opens the list.
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                this.input.update(cx, |input, cx| input.focus(window, cx))
            });
        });
        cx.simulate_keystrokes("s v");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, cx| {
                assert!(this.open);
                assert_eq!(this.input.read(cx).value().as_ref(), "sv");
                assert_eq!(
                    this.filtered
                        .iter()
                        .map(|&i| this.items[i].label.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["SvelteKit"]
                );
            });
        });
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, cx| {
                assert!(!this.open);
                assert_eq!(this.selected_value().as_deref(), Some("SvelteKit"));
                assert_eq!(this.input.read(cx).value().as_ref(), "SvelteKit");
            });
        });
    }

    #[gpui::test]
    fn arrows_navigate_and_escape_closes(cx: &mut TestAppContext) {
        let (entity, cx) = combobox(cx, |input, cx| Combobox::new(input, cx).items(frameworks()));
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                this.input.update(cx, |input, cx| input.focus(window, cx))
            });
        });
        cx.simulate_keystrokes("down down");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert!(this.open);
                assert_eq!(this.active, Some(1));
            });
        });
        cx.simulate_keystrokes("escape");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| assert!(!this.open));
        });
    }

    #[gpui::test]
    fn multiple_toggles_and_backspace_pops(cx: &mut TestAppContext) {
        let (entity, cx) = combobox(cx, |input, cx| {
            Combobox::new(input, cx)
                .items(frameworks())
                .multiple(true)
                .selected_values(&["Astro"])
        });
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                this.input.update(cx, |input, cx| input.focus(window, cx))
            });
        });
        cx.simulate_keystrokes("down enter");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert_eq!(
                    this.selected_values_vec(),
                    vec!["Astro".to_string(), "Next.js".to_string()]
                );
            });
        });
        // Backspace on the empty input removes the newest chip.
        cx.simulate_keystrokes("backspace");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert_eq!(this.selected_values_vec(), vec!["Astro".to_string()]);
            });
        });
    }

    #[test]
    fn filter_matches_label_value_and_group() {
        let mut items = grouped_items();
        items.push(ComboboxItem::new("Shown Label").value("hidden-value"));
        assert_eq!(filter_items(&items, "debounce", None), vec![1]);
        assert_eq!(filter_items(&items, "utils", None), vec![3, 4, 5, 6]);
        assert_eq!(filter_items(&items, "hidden-value", None), vec![7]);
        assert!(filter_items(&items, "nope", None).is_empty());
        // Empty query and the committed label both show the full collection.
        assert_eq!(filter_items(&items, "", None).len(), items.len());
        assert_eq!(
            filter_items(&items, "cn", Some("cn".to_string())).len(),
            items.len()
        );
    }
}
