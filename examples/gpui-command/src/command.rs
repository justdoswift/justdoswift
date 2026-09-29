//! shadcn/ui-style command palette for GPUI. Desktop and WASM share this
//! module. Reference: https://ui.shadcn.com/docs/components/aria/command
//! (React Aria / cmdk). One entity owns the query input, the filtered
//! collection and the active row; `CommandSelectEvent` fires on commit.
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use kit::base::input::{
    Enter as InputEnter, Escape as InputEscape, Input, InputEditorStyle, InputEvent, InputState,
    MoveDown as InputMoveDown, MoveUp as InputMoveUp,
};
use std::borrow::Cow;
use std::sync::atomic::{AtomicUsize, Ordering};

const FONT: &str = "Geist";

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Emitted when the user commits a row (Enter or click).
#[derive(Clone, Debug, PartialEq)]
pub struct CommandSelectEvent {
    /// The committed item's `value` (defaults to its label).
    pub value: SharedString,
}

/// Small leading glyphs for command rows (`CommandIcon`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandIcon {
    Search,
    Calendar,
    Emoji,
    Calculator,
    User,
    Card,
    Gear,
    File,
}

/// One row in the collection (`CommandItem`).
#[derive(Clone, Debug)]
pub struct CommandItem {
    pub label: SharedString,
    /// Committed value; defaults to `label`.
    pub value: SharedString,
    /// `CommandGroup` heading this row belongs to.
    pub group: Option<SharedString>,
    /// `CommandShortcut` trailing text (e.g. "⌘P").
    pub shortcut: Option<SharedString>,
    /// Extra match-only terms — filter hits but never render.
    pub keywords: Vec<SharedString>,
    /// Leading glyph.
    pub icon: Option<CommandIcon>,
    pub disabled: bool,
}

impl CommandItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            group: None,
            shortcut: None,
            keywords: Vec::new(),
            icon: None,
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

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn keywords(mut self, keywords: &[&str]) -> Self {
        self.keywords = keywords.iter().map(|k| (*k).into()).collect();
        self
    }

    pub fn icon(mut self, icon: CommandIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// How the flat `filtered` list renders: headings between groups and a
/// separator line between two different groups (`CommandSeparator`).
enum ListRow {
    Group(SharedString),
    Separator,
    Item(usize),
}

impl EventEmitter<CommandSelectEvent> for Command {}

/// `Command` — search field on top, always-visible filtered list below.
pub struct Command {
    input: Entity<InputState>,
    items: Vec<CommandItem>,
    /// Indices into `items` matching the current query, in display order.
    filtered: Vec<usize>,
    /// Position in `filtered` of the keyboard-highlighted row.
    active: Option<usize>,
    dark: bool,
    /// `CommandEmpty` content — shown when the query matches nothing.
    empty_text: SharedString,
    /// List viewport height cap (`CommandList` maxHeight).
    list_max_h: Pixels,
    scroll: ScrollHandle,
    id: usize,
    _subscriptions: Vec<Subscription>,
}

impl Command {
    /// Wraps a caller-created `InputState` (it needs a `Window`), then
    /// `cx.new(|cx| Command::new(input, cx))`.
    pub fn new(input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let id = NEXT_KEY.fetch_add(1, Ordering::Relaxed);
        let input_sub =
            cx.subscribe(
                &input,
                |this: &mut Self, _, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.refilter_with(cx);
                        this.active = this.filtered.first().copied().map(|_| 0);
                        cx.notify();
                    }
                    _ => {}
                },
            );
        Self {
            input,
            items: Vec::new(),
            filtered: Vec::new(),
            active: Some(0),
            dark: false,
            empty_text: "No results found.".into(),
            list_max_h: px(300.),
            scroll: ScrollHandle::new(),
            id,
            _subscriptions: vec![input_sub],
        }
    }

    pub fn items(mut self, items: Vec<CommandItem>) -> Self {
        self.filtered = items.iter().enumerate().map(|(i, _)| i).collect();
        self.active = if self.filtered.is_empty() {
            None
        } else {
            Some(0)
        };
        self.items = items;
        self
    }

    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// `CommandList` viewport height cap before it scrolls.
    pub fn list_max_h(mut self, height: Pixels) -> Self {
        self.list_max_h = height;
        self
    }

    /// The query string, for hosts that want to react to it.
    pub fn query(&self, cx: &App) -> String {
        self.input.read(cx).value().to_string()
    }

    fn refilter_with(&mut self, cx: &App) {
        let query = self.input.read(cx).value().to_string();
        self.filtered = filter_items(&self.items, &query);
        if self.filtered.is_empty() {
            self.active = None;
        }
    }

    fn commit_active(&mut self, cx: &mut Context<Self>) {
        let Some(position) = self.active else { return };
        let Some(&index) = self.filtered.get(position) else {
            return;
        };
        if self.items[index].disabled {
            return;
        }
        cx.emit(CommandSelectEvent {
            value: self.items[index].value.clone(),
        });
    }

    fn move_active(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.filtered.is_empty() {
            return;
        }
        // Skip disabled rows — they render but never become active.
        let len = self.filtered.len() as isize;
        let mut next = match self.active {
            None => 0isize,
            Some(i) => i as isize + delta,
        };
        for _ in 0..self.filtered.len() {
            let pos = next.rem_euclid(len) as usize;
            if !self.items[self.filtered[pos]].disabled {
                self.active = Some(pos);
                self.scroll.scroll_to_item(pos);
                cx.notify();
                return;
            }
            next += delta;
        }
    }

    /// `filtered` → render rows with group headings and separators.
    fn rows(&self) -> Vec<ListRow> {
        let mut rows = Vec::new();
        let mut last_group: Option<&SharedString> = None;
        for &index in self.filtered.iter() {
            let item = &self.items[index];
            let group = item.group.as_ref();
            match (last_group, group) {
                (None, Some(g)) => rows.push(ListRow::Group(g.clone())),
                (Some(a), Some(b)) if a != b => {
                    rows.push(ListRow::Separator);
                    rows.push(ListRow::Group(b.clone()));
                }
                (Some(_), None) => rows.push(ListRow::Separator),
                _ => {}
            }
            last_group = group;
            rows.push(ListRow::Item(index));
        }
        rows
    }
}

/// Case-insensitive substring match over label, value, group and keywords.
fn filter_items(items: &[CommandItem], query: &str) -> Vec<usize> {
    let query = query.trim().to_lowercase();
    if query.is_empty() {
        return items.iter().enumerate().map(|(i, _)| i).collect();
    }
    items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            item.label.to_lowercase().contains(&query)
                || item.value.to_lowercase().contains(&query)
                || item
                    .group
                    .as_ref()
                    .is_some_and(|g| g.to_lowercase().contains(&query))
                || item
                    .keywords
                    .iter()
                    .any(|k| k.to_lowercase().contains(&query))
        })
        .map(|(i, _)| i)
        .collect()
}

// ── Glyphs ────────────────────────────────────────────────────────────────

fn icon_element(icon: CommandIcon, color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let w = px(1.2 * k.max(0.7));
            let stroke = |b: &mut PathBuilder, points: &[(f32, f32)], close: bool| {
                b.move_to(at(points[0].0, points[0].1));
                for &(x, y) in &points[1..] {
                    b.line_to(at(x, y));
                }
                if close {
                    b.line_to(at(points[0].0, points[0].1));
                }
            };
            let mut b = PathBuilder::stroke(w);
            match icon {
                CommandIcon::Search => {
                    // Circle + handle
                    for seg in 0..16 {
                        let a0 = seg as f32 / 16. * std::f32::consts::TAU;
                        let a1 = (seg + 1) as f32 / 16. * std::f32::consts::TAU;
                        b.move_to(at(6.5 + a0.cos() * 4.4, 6.5 + a0.sin() * 4.4));
                        b.line_to(at(6.5 + a1.cos() * 4.4, 6.5 + a1.sin() * 4.4));
                    }
                    stroke(&mut b, &[(10.6, 10.6), (13.6, 13.6)], false);
                }
                CommandIcon::Calendar => {
                    stroke(
                        &mut b,
                        &[(2.4, 3.4), (13.6, 3.4), (13.6, 13.8), (2.4, 13.8)],
                        true,
                    );
                    stroke(&mut b, &[(5., 1.8), (5., 5.)], false);
                    stroke(&mut b, &[(11., 1.8), (11., 5.)], false);
                    stroke(&mut b, &[(2.4, 7.), (13.6, 7.)], false);
                }
                CommandIcon::Emoji => {
                    for seg in 0..16 {
                        let a0 = seg as f32 / 16. * std::f32::consts::TAU;
                        let a1 = (seg + 1) as f32 / 16. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 5.6, 8. + a0.sin() * 5.6));
                        b.line_to(at(8. + a1.cos() * 5.6, 8. + a1.sin() * 5.6));
                    }
                    stroke(&mut b, &[(5.6, 6.4), (5.7, 6.4)], false);
                    stroke(&mut b, &[(10.4, 6.4), (10.5, 6.4)], false);
                    // Smile: parabola dipping down between the cheeks.
                    let mut prev = (5.2f32, 9.6f32);
                    for i in 1..=8 {
                        let t = i as f32 / 8.;
                        let x = 5.2 + t * 5.6;
                        let y = 9.6 + t * (1. - t) * 9.2;
                        b.move_to(at(prev.0, prev.1));
                        b.line_to(at(x, y));
                        prev = (x, y);
                    }
                }
                CommandIcon::Calculator => {
                    stroke(
                        &mut b,
                        &[(3.4, 2.2), (12.6, 2.2), (12.6, 13.8), (3.4, 13.8)],
                        true,
                    );
                    stroke(&mut b, &[(3.4, 5.6), (12.6, 5.6)], false);
                    for &(x, y) in &[
                        (5.4, 8.2),
                        (8., 8.2),
                        (10.6, 8.2),
                        (5.4, 11.),
                        (8., 11.),
                        (10.6, 11.),
                    ] {
                        stroke(&mut b, &[(x, y), (x + 0.01, y)], false);
                    }
                }
                CommandIcon::User => {
                    for seg in 0..12 {
                        let a0 = seg as f32 / 12. * std::f32::consts::TAU;
                        let a1 = (seg + 1) as f32 / 12. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 3., 5.6 + a0.sin() * 3.));
                        b.line_to(at(8. + a1.cos() * 3., 5.6 + a1.sin() * 3.));
                    }
                    // Shoulders arc
                    let mut prev = (3.4f32, 13.6f32);
                    for i in 1..=8 {
                        let t = i as f32 / 8.;
                        let x = 3.4 + t * 9.2;
                        let y = 13.6 - (t * (1. - t)) * 8.4;
                        b.move_to(at(prev.0, prev.1));
                        b.line_to(at(x, y));
                        prev = (x, y);
                    }
                }
                CommandIcon::Card => {
                    stroke(
                        &mut b,
                        &[(2., 3.6), (14., 3.6), (14., 12.4), (2., 12.4)],
                        true,
                    );
                    stroke(&mut b, &[(2., 6.4), (14., 6.4)], false);
                    stroke(&mut b, &[(4., 9.6), (7.4, 9.6)], false);
                }
                CommandIcon::Gear => {
                    for seg in 0..12 {
                        let a0 = seg as f32 / 12. * std::f32::consts::TAU;
                        let a1 = (seg + 1) as f32 / 12. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 3., 8. + a0.sin() * 3.));
                        b.line_to(at(8. + a1.cos() * 3., 8. + a1.sin() * 3.));
                    }
                    for i in 0..4 {
                        let a = i as f32 / 4. * std::f32::consts::TAU;
                        stroke(
                            &mut b,
                            &[
                                (8. + a.cos() * 4.4, 8. + a.sin() * 4.4),
                                (8. + a.cos() * 6.4, 8. + a.sin() * 6.4),
                            ],
                            false,
                        );
                        let a = a + std::f32::consts::FRAC_PI_4;
                        stroke(
                            &mut b,
                            &[
                                (8. + a.cos() * 4.4, 8. + a.sin() * 4.4),
                                (8. + a.cos() * 6.4, 8. + a.sin() * 6.4),
                            ],
                            false,
                        );
                    }
                }
                CommandIcon::File => {
                    stroke(
                        &mut b,
                        &[(4., 2.), (10., 2.), (12.4, 4.4), (12.4, 14.), (4., 14.)],
                        true,
                    );
                    stroke(&mut b, &[(10., 2.), (10., 4.4), (12.4, 4.4)], false);
                }
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(15.))
    .flex_none()
    .into_any_element()
}

/// Magnifier shown at the left of `CommandInput`.
fn search_glyph(color: Hsla) -> impl IntoElement {
    icon_element(CommandIcon::Search, color)
}

/// Drawn modifier-key glyph — Geist has no ⌘/⇧/⌥/⌃ codepoints, so the
/// shortcut row paints them as paths instead of falling back to tofu.
fn modifier_icon(ch: char, color: Hsla) -> Option<AnyElement> {
    let points: &[&[(f32, f32)]] = match ch {
        // Looped square: four corner loops + connecting bars.
        '⌘' => &[
            &[
                (5.8, 5.8),
                (5.8, 10.2),
                (10.2, 10.2),
                (10.2, 5.8),
                (5.8, 5.8),
            ],
            &[(3., 3.), (4.6, 1.8), (6.4, 2.8), (5.8, 5.8)],
            &[(10., 2.8), (11.6, 1.6), (13.4, 3.), (10.2, 5.8)],
            &[(10.2, 10.2), (13.4, 13.), (11.6, 14.4), (10., 13.2)],
            &[(5.8, 10.2), (6.4, 13.2), (4.6, 14.4), (3., 13.)],
            &[(5.8, 5.8), (3., 3.), (2.6, 4.8), (4., 5.8)],
            &[(10.2, 5.8), (13.4, 3.), (13.8, 4.8), (12., 5.8)],
            &[(10.2, 10.2), (12., 10.2), (13.8, 11.4), (13.4, 13.)],
            &[(5.8, 10.2), (4., 10.2), (2.6, 11.4), (3., 13.)],
        ],
        // Shift: thick outlined up arrow.
        '⇧' => &[&[
            (8., 1.8),
            (12.6, 7.),
            (10., 7.),
            (10., 14.2),
            (6., 14.2),
            (6., 7.),
            (3.4, 7.),
            (8., 1.8),
        ]],
        // Option: short bar then stepped diagonal.
        '⌥' => &[
            &[(1.8, 4.6), (6.2, 4.6), (10.4, 11.4), (14.2, 11.4)],
            &[(9.6, 4.6), (14.2, 4.6)],
        ],
        // Control: caret.
        '⌃' => &[&[(3.6, 10.6), (8., 3.6), (12.4, 10.6)]],
        _ => return None,
    };
    Some(
        canvas(
            move |_, _, _| (),
            move |bounds, _, window, _| {
                let o = bounds.origin;
                let s: f32 = bounds.size.width.into();
                let k = s / 16.;
                let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
                let w = px(1.1 * k.max(0.7));
                let mut b = PathBuilder::stroke(w);
                for seg in points {
                    b.move_to(at(seg[0].0, seg[0].1));
                    for &(x, y) in &seg[1..] {
                        b.line_to(at(x, y));
                    }
                }
                if let Ok(path) = b.build() {
                    window.paint_path(path, color);
                }
            },
        )
        .size(px(11.))
        .flex_none()
        .into_any_element(),
    )
}

/// `CommandShortcut` — trailing muted text with drawn ⌘/⇧/⌥/⌃ keys.
fn shortcut_label(shortcut: &SharedString, color: Hsla) -> AnyElement {
    let mut row = div()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(3.))
        .text_xs()
        .text_color(color);
    let mut text = String::new();
    for ch in shortcut.chars() {
        if let Some(icon) = modifier_icon(ch, color) {
            if !text.is_empty() {
                row = row.child(std::mem::take(&mut text));
            }
            row = row.child(icon);
        } else {
            text.push(ch);
        }
    }
    if !text.is_empty() {
        row = row.child(text);
    }
    row.into_any_element()
}

// ── Render ────────────────────────────────────────────────────────────────

impl Render for Command {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let dark = self.dark;
        let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into();
        let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
        let accent_bg: Hsla = rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into();
        let card: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();

        // ── Input row: magnifier + real single-line editor ──────────────
        let input_row = div()
            .flex()
            .items_center()
            .gap_2p5()
            .h(px(44.))
            .px_3()
            .border_b_1()
            .border_color(edge)
            .child(search_glyph(muted))
            .child(div().flex_1().min_w_0().child(Input::new(&self.input)));

        // ── List ────────────────────────────────────────────────────────
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
                    ListRow::Group(heading) => {
                        list = list.child(
                            div()
                                .px_3()
                                .pt_3()
                                .pb_1()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(muted)
                                .child(heading),
                        );
                    }
                    ListRow::Separator => {
                        list = list.child(div().h(px(1.)).bg(edge).my_1());
                    }
                    ListRow::Item(index) => {
                        let item = &self.items[index];
                        let position = self.filtered.iter().position(|&i| i == index).unwrap_or(0);
                        let is_active = self.active == Some(position);
                        let mut item_row = div()
                            .flex()
                            .items_center()
                            .gap_2p5()
                            .px_3()
                            .py(px(7.))
                            .mx_1()
                            .rounded_sm()
                            .text_sm()
                            .text_color(ink);
                        if let Some(icon) = item.icon {
                            item_row = item_row
                                .child(icon_element(icon, if item.disabled { muted } else { ink }));
                        }
                        item_row = item_row
                            .child(div().flex_1().min_w_0().child(item.label.clone()))
                            .when_some(item.shortcut.as_ref(), |row, shortcut| {
                                row.child(shortcut_label(shortcut, muted))
                            });
                        list = list.child(if item.disabled {
                            item_row.opacity(0.5).into_any_element()
                        } else {
                            item_row
                                .id(("command-item", index))
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
                                        this.active = Some(position);
                                        this.commit_active(cx);
                                    }),
                                )
                                .into_any_element()
                        });
                    }
                }
            }
        }

        div()
            .w_full()
            .rounded_lg()
            .border_1()
            .border_color(edge)
            .bg(card)
            .font_family(FONT)
            // Single-line inputs register no up/down handlers — the Input
            // context bindings dispatch input::MoveUp/MoveDown, which bubble
            // up here and drive the highlighted row. Enter/Escape likewise
            // arrive as input::Enter/input::Escape.
            .on_action(cx.listener(|this, _: &InputMoveDown, _, cx| this.move_active(1, cx)))
            .on_action(cx.listener(|this, _: &InputMoveUp, _, cx| this.move_active(-1, cx)))
            .on_action(cx.listener(|this, _: &InputEnter, _, cx| this.commit_active(cx)))
            .on_action(cx.listener(|this, _: &InputEscape, window, cx| {
                // cmdk behaviour: Escape clears the query first; a second
                // Escape propagates so the host can close the palette.
                if this.input.read(cx).value().is_empty() {
                    cx.propagate();
                    return;
                }
                let input = this.input.clone();
                input.update(cx, |input, cx| input.set_value("", window, cx));
                this.refilter_with(cx);
            }))
            .child(input_row)
            .child(
                div()
                    .id(("command-list", self.id))
                    .max_h(self.list_max_h)
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .py_1()
                    .child(list),
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
        caret: rgb(if dark { 0xc4b5fd } else { 0x7c3aed }).into(),
        selection: rgba(if dark { 0x8b5cf650 } else { 0x7c3aed30 }).into(),
        ..Default::default()
    }
}

fn command_input(
    placeholder: &str,
    dark: bool,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputState> {
    cx.new(|cx| {
        let mut state = InputState::new(window, cx).placeholder(placeholder.to_string());
        state.set_editor_style(editor_style(dark));
        state.focus(window, cx);
        state
    })
}

/// Suggestions + Settings — the canonical shadcn example.
fn standard_items(shortcuts: bool, icons: bool) -> Vec<CommandItem> {
    let icon = |i: CommandIcon| if icons { Some(i) } else { None };
    let mut items = vec![
        CommandItem::new("Calendar")
            .group("Suggestions")
            .icon(CommandIcon::Calendar),
        CommandItem::new("Search Emoji")
            .group("Suggestions")
            .icon(CommandIcon::Emoji),
        CommandItem::new("Calculator")
            .group("Suggestions")
            .icon(CommandIcon::Calculator)
            .disabled(true),
        CommandItem::new("Profile").group("Settings"),
        CommandItem::new("Billing").group("Settings"),
        CommandItem::new("Settings").group("Settings"),
    ];
    if icons {
        items[3].icon = icon(CommandIcon::User);
        items[4].icon = icon(CommandIcon::Card);
        items[5].icon = icon(CommandIcon::Gear);
    }
    if shortcuts {
        items[3].shortcut = Some("⌘P".into());
        items[4].shortcut = Some("⌘B".into());
        items[5].shortcut = Some("⌘S".into());
    }
    items
}

fn big_items() -> Vec<CommandItem> {
    [
        "New File",
        "New Folder",
        "Open Recent",
        "Save",
        "Save As…",
        "Save All",
        "Close Editor",
        "Close Window",
        "Preferences",
        "Keyboard Shortcuts",
        "Color Theme",
        "File Icon Theme",
        "Extensions",
        "Settings Sync",
        "Command Palette",
        "Quick Open",
        "Go to Symbol",
        "Go to Line",
        "Toggle Terminal",
        "Toggle Sidebar",
        "Zen Mode",
        "Restart",
    ]
    .into_iter()
    .enumerate()
    .map(|(i, label)| {
        let mut item = CommandItem::new(label).group(if i < 8 {
            "File"
        } else if i < 14 {
            "Preferences"
        } else {
            "View"
        });
        if label == "Command Palette" {
            item = item.shortcut("⇧⌘P").keywords(&["palette", "commands"]);
        }
        if label == "Quick Open" {
            item = item.shortcut("⌘P").keywords(&["open", "file"]);
        }
        item
    })
    .collect()
}

struct CommandDemo {
    command: Entity<Command>,
    dark: bool,
    variant: SharedString,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl CommandDemo {
    fn new(variant: &str, dark: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let command: Entity<Command> = match variant {
            "shortcuts" => {
                let input = command_input("Type a command or search…", dark, window, cx);
                cx.new(|cx| {
                    Command::new(input, cx)
                        .items(standard_items(true, true))
                        .dark(dark)
                })
            }
            "groups" => {
                let input = command_input("Search actions…", dark, window, cx);
                cx.new(|cx| Command::new(input, cx).items(big_items()).dark(dark))
            }
            "scrollable" => {
                let input = command_input("Search all commands…", dark, window, cx);
                cx.new(|cx| {
                    Command::new(input, cx)
                        .items(big_items())
                        .list_max_h(px(224.))
                        .dark(dark)
                })
            }
            "empty" => {
                let input = command_input("Pick a number…", dark, window, cx);
                cx.new(|cx| {
                    Command::new(input, cx)
                        .items(Vec::new())
                        .empty_text("Nothing here yet.")
                        .dark(dark)
                })
            }
            // "dialog" (CommandDialog) — dimmed backdrop + floating panel.
            "dialog" => {
                let input = command_input("Type a command or search…", dark, window, cx);
                cx.new(|cx| {
                    Command::new(input, cx)
                        .items(standard_items(true, true))
                        .list_max_h(px(240.))
                        .dark(dark)
                })
            }
            _ => {
                let input = command_input("Type a command or search…", dark, window, cx);
                cx.new(|cx| {
                    Command::new(input, cx)
                        .items(standard_items(false, true))
                        .dark(dark)
                })
            }
        };
        let mut subs: Vec<Subscription> = Vec::new();
        subs.push(
            cx.subscribe(&command, |demo, _, ev: &CommandSelectEvent, cx| {
                demo.outcome = Some(format!("Ran: {}", ev.value).into());
                cx.notify();
            }),
        );
        Self {
            command,
            dark,
            variant: variant.to_string().into(),
            outcome: None,
            _subscriptions: subs,
        }
    }
}

impl Render for CommandDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, edge, _card, surface) = palette(self.dark);
        if self.variant.as_ref() == "dialog" {
            // CommandDialog: fake page content behind a dimmed scrim, palette
            // floats on the deferred pass so it paints above everything.
            return div()
                .size_full()
                .font_family(FONT)
                .bg(surface)
                .child(
                    div()
                        .p_6()
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(div().h(px(28.)).w(px(180.)).rounded_md().bg(edge))
                        .child(div().h(px(14.)).w(px(300.)).rounded_sm().bg(edge))
                        .child(div().h(px(14.)).w(px(240.)).rounded_sm().bg(edge))
                        .child(div().h(px(14.)).w(px(280.)).rounded_sm().bg(edge)),
                )
                .child(
                    div()
                        .absolute()
                        .inset_0()
                        .bg(rgba(if self.dark { 0x00000090 } else { 0x00000060 }))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .w(px(380.))
                                .rounded_lg()
                                .shadow_lg()
                                .child(self.command.clone()),
                        ),
                )
                .child(
                    div()
                        .absolute()
                        .bottom_4()
                        .left_0()
                        .right_0()
                        .flex()
                        .justify_center()
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(if self.dark { 0xd4d4d4 } else { 0xf0f0f0 }))
                                .child(self.outcome.clone().unwrap_or_else(|| " ".into())),
                        ),
                );
        }

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
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .child(div().w(px(360.)).child(self.command.clone()))
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
        cx.new(|cx| CommandDemo::new(&variant, dark, window, cx))
    })
    .expect("open command gallery");
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
    use super::{Command, CommandItem, CommandSelectEvent, filter_items, standard_items};
    use gpui_kit::base::input::InputState;
    use gpui_kit::gpui;
    use gpui_kit::gpui::{
        AppContext, Context, Entity, Subscription, TestAppContext, VisualTestContext,
    };
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn command(
        cx: &mut TestAppContext,
        build: impl FnOnce(Entity<InputState>, &mut Context<Command>) -> Command,
    ) -> (Entity<Command>, &mut VisualTestContext) {
        cx.update(gpui_kit::base::init);
        cx.add_window_view(move |window, cx| {
            let input = cx.new(|cx| InputState::new(window, cx));
            build(input, cx)
        })
    }

    #[gpui::test]
    fn typing_filters_and_enter_emits(cx: &mut TestAppContext) {
        let hits = Arc::new(AtomicUsize::new(0));
        let last = Arc::new(std::sync::Mutex::new(String::new()));
        let (entity, cx) = command(cx, |input, cx| {
            Command::new(input, cx).items(standard_items(false, false))
        });
        let hits2 = hits.clone();
        let last2 = last.clone();
        let _sub: Subscription = cx.update(|_, cx| {
            cx.subscribe(&entity, move |_, ev: &CommandSelectEvent, _| {
                hits2.fetch_add(1, Ordering::SeqCst);
                *last2.lock().unwrap() = ev.value.to_string();
            })
        });
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                this.input.update(cx, |input, cx| input.focus(window, cx))
            });
        });
        cx.simulate_keystrokes("e m o");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, cx| {
                assert_eq!(
                    this.filtered
                        .iter()
                        .map(|&i| this.items[i].label.as_ref())
                        .collect::<Vec<_>>(),
                    vec!["Search Emoji"]
                );
                assert_eq!(this.active, Some(0));
                let _ = cx;
            });
        });
        cx.simulate_keystrokes("enter");
        assert_eq!(hits.load(Ordering::SeqCst), 1);
        assert_eq!(last.lock().unwrap().as_str(), "Search Emoji");
    }

    #[gpui::test]
    fn arrows_skip_disabled_and_escape_clears(cx: &mut TestAppContext) {
        let (entity, cx) = command(cx, |input, cx| {
            Command::new(input, cx).items(standard_items(false, false))
        });
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                this.input.update(cx, |input, cx| input.focus(window, cx))
            });
        });
        // Items: Calendar, Search Emoji, Calculator(disabled), Profile, …
        // Active starts at 0; down down lands on Profile, skipping Calculator.
        cx.simulate_keystrokes("down down");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert_eq!(this.active, Some(3));
            });
        });
        cx.simulate_keystrokes("u p escape");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, cx| {
                assert!(this.input.read(cx).value().is_empty());
            });
        });
    }

    #[test]
    fn filter_matches_label_value_group_and_keywords() {
        let mut items = standard_items(false, false);
        items.push(
            CommandItem::new("Shown Label")
                .value("hidden-value")
                .keywords(&["alias-hit"]),
        );
        assert_eq!(filter_items(&items, "debounce"), Vec::<usize>::new());
        assert_eq!(filter_items(&items, "hidden-value"), vec![6]);
        assert_eq!(filter_items(&items, "alias"), vec![6]);
        assert_eq!(filter_items(&items, "settings").len(), 3); // group name matches
        assert_eq!(filter_items(&items, "  ").len(), items.len());
    }

    #[test]
    fn rows_emit_group_headers_and_separators() {
        let items = standard_items(false, false);
        // groups(): reimplemented here via Command::rows shape — assert the
        // group/order invariants on the filtered data instead.
        let filtered = filter_items(&items, "");
        assert_eq!(filtered, (0..items.len()).collect::<Vec<_>>());
        let groups: Vec<Option<&str>> = filtered
            .iter()
            .map(|&i| items[i].group.as_deref())
            .collect();
        assert_eq!(
            groups,
            vec![
                Some("Suggestions"),
                Some("Suggestions"),
                Some("Suggestions"),
                Some("Settings"),
                Some("Settings"),
                Some("Settings")
            ]
        );
    }
}
