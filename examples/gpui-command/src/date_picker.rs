//! shadcn/ui-style date picker for GPUI — `PopoverTrigger → Button +
//! Popover → Calendar`. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/date-picker
use crate::calendar::{Calendar, CalendarMode, CalendarSelectEvent, CalendarSelection, Date};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use kit::base::input::{Input, InputEditorStyle, InputEvent, InputState};
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

const FONT: &str = "Geist";
const TRIGGER_H: Pixels = px(32.);
static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// What the user picked — mirrors `CalendarSelection` with the time field.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DatePick {
    Single(Date),
    Range { start: Date, end: Option<Date> },
}

/// Emitted whenever the selection changes (calendar click, typed date,
/// natural-language parse or time edit).
#[derive(Clone, Debug, PartialEq)]
pub struct DatePickerEvent {
    pub selection: Option<DatePick>,
    pub time: Option<(u32, u32)>,
}

/// `DatePicker` — trigger button (or typed input) + deferred popover
/// carrying a `Calendar` entity.
pub struct DatePicker {
    calendar: Option<Entity<Calendar>>,
    mode: CalendarMode,
    date: Option<Date>,
    range_start: Option<Date>,
    range_end: Option<Date>,
    /// Typed-date field (`Input` variant) — parses "YYYY-MM-DD" / "M/D/YYYY".
    date_input: Option<Entity<InputState>>,
    /// Natural-language field (`natural` variant) — parses on Enter.
    natural_input: Option<Entity<InputState>>,
    natural_note: Option<SharedString>,
    /// Time field inside the popover (`time` variant) — parses "H:MM".
    time_input: Option<Entity<InputState>>,
    time: Option<(u32, u32)>,
    /// Field label rendered above the trigger ("Date of birth").
    /// Date text waiting to be written into `date_input` — subscribe
    /// callbacks have no `Window`, so the sync happens in `render`.
    pending_input_text: Option<String>,
    label: Option<SharedString>,
    placeholder: SharedString,
    open: bool,
    disabled: bool,
    dark: bool,
    root_bounds: Rc<RefCell<Bounds<Pixels>>>,
    trigger_bounds: Rc<RefCell<Bounds<Pixels>>>,
    id: usize,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<DatePickerEvent> for DatePicker {}

impl DatePicker {
    /// `calendar` is caller-created (mode, months, dropdown caption etc).
    /// Pass `None` for an input-only picker (`natural` variant).
    pub fn new(
        calendar: Option<Entity<Calendar>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            calendar: None,
            mode: CalendarMode::Single,
            date: None,
            range_start: None,
            range_end: None,
            date_input: None,
            natural_input: None,
            natural_note: None,
            time_input: None,
            time: None,
            pending_input_text: None,
            label: None,
            placeholder: "Pick a date".into(),
            open: false,
            disabled: false,
            dark: false,
            root_bounds: Rc::new(RefCell::new(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(0.), px(0.)),
            })),
            trigger_bounds: Rc::new(RefCell::new(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(0.), px(0.)),
            })),
            id: NEXT_KEY.fetch_add(1, Ordering::Relaxed),
            _subscriptions: Vec::new(),
        };
        if let Some(calendar) = calendar {
            this.mode = calendar.read(cx).mode_of();
            let sub = cx.subscribe(
                &calendar,
                |this: &mut Self, _, ev: &CalendarSelectEvent, cx| {
                    this.on_calendar_pick(ev.selection, cx);
                },
            );
            this._subscriptions.push(sub);
            this.calendar = Some(calendar);
        }
        let _ = window;
        this
    }

    /// Show a "Label" caption above the trigger.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn placeholder(mut self, text: impl Into<SharedString>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Typed-date input in place of the text label — parses on every
    /// change and on Enter.
    pub fn date_input(mut self, input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let sub = cx.subscribe(
            &input,
            |this: &mut Self, _, event: &InputEvent, cx| match event {
                InputEvent::Change | InputEvent::PressEnter { .. } => {
                    let input = this.date_input.clone().unwrap();
                    let text = input.read(cx).value().to_string();
                    if let Some(d) = parse_date(&text) {
                        this.date = Some(d);
                        if let Some(cal) = &this.calendar {
                            cal.update(cx, |c, cx| c.set_selected(d, cx));
                        }
                        this.emit_current(cx);
                    }
                    cx.notify();
                }
                _ => {}
            },
        );
        self._subscriptions.push(sub);
        self.date_input = Some(input);
        self
    }

    /// Natural-language input — Enter parses "tomorrow", "in 3 days",
    /// "next Friday", ISO/US dates, and echoes a sentence.
    pub fn natural_input(mut self, input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let sub = cx.subscribe(
            &input,
            |this: &mut Self, _, event: &InputEvent, cx| match event {
                InputEvent::Change | InputEvent::PressEnter { .. } => {
                    let input = this.natural_input.clone().unwrap();
                    let text = input.read(cx).value().to_string();
                    if let Some(d) = parse_natural(&text) {
                        this.date = Some(d);
                        this.natural_note =
                            Some(format!("Your post will be published on {}.", fmt_long(d)).into());
                        this.emit_current(cx);
                    } else if !text.trim().is_empty() {
                        this.natural_note = Some("Could not parse that date.".into());
                    } else {
                        this.natural_note = None;
                    }
                    cx.notify();
                }
                _ => {}
            },
        );
        self._subscriptions.push(sub);
        self.natural_input = Some(input);
        self
    }

    /// "Time" field rendered under the calendar in the popover.
    pub fn time_input(mut self, input: Entity<InputState>, cx: &mut Context<Self>) -> Self {
        let sub = cx.subscribe(&input, |this: &mut Self, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                let input = this.time_input.clone().unwrap();
                let text = input.read(cx).value().to_string();
                this.time = parse_time(&text);
                this.emit_current(cx);
            }
        });
        self._subscriptions.push(sub);
        self.time_input = Some(input);
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

    /// Controlled open/close.
    pub fn set_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.open = open;
        cx.notify();
    }

    /// Current single selection (single mode).
    pub fn date(&self) -> Option<Date> {
        self.date
    }

    /// Current range endpoints (range mode).
    pub fn range(&self) -> Option<(Date, Option<Date>)> {
        self.range_start.map(|s| (s, self.range_end))
    }

    fn on_calendar_pick(&mut self, sel: CalendarSelection, cx: &mut Context<Self>) {
        match sel {
            CalendarSelection::Single(d) => {
                self.date = Some(d);
                if self.date_input.is_some() {
                    self.pending_input_text = Some(fmt_iso(d));
                }
                self.emit_current(cx);
                // Single pick closes the popover like Radix.
                self.open = false;
            }
            CalendarSelection::Range { start, end } => {
                self.range_start = Some(start);
                self.range_end = end;
                self.emit_current(cx);
                if end.is_some() {
                    self.open = false;
                }
            }
        }
        cx.notify();
    }

    fn emit_current(&self, cx: &mut Context<Self>) {
        let selection = match self.mode {
            CalendarMode::Single => self.date.map(DatePick::Single),
            CalendarMode::Range => self.range_start.map(|start| DatePick::Range {
                start,
                end: self.range_end,
            }),
        };
        cx.emit(DatePickerEvent {
            selection,
            time: self.time,
        });
    }

    /// Trigger label — placeholder when nothing is picked.
    fn trigger_label(&self) -> (SharedString, bool) {
        let (text, empty) = match self.mode {
            CalendarMode::Single => match self.date {
                Some(d) => (fmt_long(d), false),
                None => (self.placeholder.to_string(), true),
            },
            CalendarMode::Range => match (self.range_start, self.range_end) {
                (Some(s), Some(e)) => (format!("{} – {}", fmt_long(s), fmt_long(e)), false),
                (Some(s), None) => (format!("{} –", fmt_long(s)), false),
                _ => (self.placeholder.to_string(), true),
            },
        };
        (text.into(), empty)
    }
}

// ── Parsing & formatting ──────────────────────────────────────────────────

pub(crate) fn fmt_long(d: Date) -> String {
    format!("{} {}, {}", Date::month_name(d.month), d.day, d.year)
}

fn fmt_iso(d: Date) -> String {
    format!("{:04}-{:02}-{:02}", d.year, d.month, d.day)
}

/// `YYYY-MM-DD` or `M/D/YYYY` (also `M-D-YYYY`, `YYYY/M/D`).
pub(crate) fn parse_date(text: &str) -> Option<Date> {
    let t = text.trim();
    for sep in ['-', '/'] {
        let parts: Vec<&str> = t.split(sep).collect();
        if parts.len() == 3 {
            let a: i64 = parts[0].trim().parse().ok()?;
            let b: i64 = parts[1].trim().parse().ok()?;
            let c: i64 = parts[2].trim().parse().ok()?;
            let (y, m, d) = if parts[0].len() == 4 {
                (a, b, c)
            } else {
                (c, a, b)
            };
            if (1..=12).contains(&m) && (1..=31).contains(&d) && (1..=9999).contains(&y) {
                return Some(Date::new(y as i32, m as u32, d as u32));
            }
        }
    }
    None
}

/// "9:00", "21:15", "9 am", "9:30 pm".
fn parse_time(text: &str) -> Option<(u32, u32)> {
    let t = text.trim().to_lowercase();
    let pm = t.contains("pm");
    let am = t.contains("am");
    let digits: String = t
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ':')
        .collect();
    let mut parts = digits.split(':');
    let h: u32 = parts.next()?.parse().ok()?;
    let m: u32 = parts.next().unwrap_or("0").parse().unwrap_or(0);
    let h = if pm && h < 12 {
        h + 12
    } else if am && h == 12 {
        0
    } else {
        h
    };
    (h <= 23 && m <= 59).then_some((h, m))
}

/// Small chrono-lite parser: today/tomorrow/yesterday, "in N days|weeks|
/// months", "N days later", "next week", "next <weekday>", bare weekday
/// names, plus ISO/US dates.
pub(crate) fn parse_natural(text: &str) -> Option<Date> {
    let t = text.trim().to_lowercase();
    if t.is_empty() {
        return None;
    }
    let today = Date::today();
    match t.as_str() {
        "today" | "now" => return Some(today),
        "tomorrow" => return Some(today.add_days(1)),
        "yesterday" => return Some(today.add_days(-1)),
        "next week" => return Some(today.add_days(7)),
        "next month" => return Some(today.add_days(30)),
        _ => {}
    }
    // "in 3 days" / "in 2 weeks" / "3 days later" / "next friday".
    let tokens: Vec<&str> = t.split_whitespace().collect();
    if let Some(n) = tokens.iter().find_map(|tok| tok.parse::<i64>().ok()) {
        let unit = tokens.iter().find(|tok| {
            matches!(
                **tok,
                "day" | "days" | "week" | "weeks" | "month" | "months"
            )
        });
        let days = match unit.copied() {
            Some("day" | "days") => n,
            Some("week" | "weeks") => n * 7,
            Some("month" | "months") => n * 30,
            _ => return None,
        };
        return Some(today.add_days(days));
    }
    let weekday = tokens.iter().find_map(|tok| {
        [
            "sunday",
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
        ]
        .iter()
        .position(|d| *tok == *d)
    });
    if let Some(target) = weekday {
        let cur = today.weekday() as usize;
        let delta = if target > cur {
            target - cur
        } else {
            7 - (cur - target)
        };
        return Some(today.add_days(delta as i64));
    }
    parse_date(&t)
}

// ── Glyphs ────────────────────────────────────────────────────────────────

/// Calendar icon — rounded rect, top bar, two tabs, two day dots.
fn calendar_glyph(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.3 * k.max(0.7)));
            // Frame (rounded look via clipped corners).
            b.move_to(at(3., 4.4));
            b.line_to(at(13., 4.4));
            b.move_to(at(2.6, 4.));
            b.line_to(at(2.6, 13.4));
            b.move_to(at(13.4, 4.));
            b.line_to(at(13.4, 13.4));
            b.move_to(at(3., 13.8));
            b.line_to(at(13., 13.8));
            // Hangers.
            b.move_to(at(5.4, 2.2));
            b.line_to(at(5.4, 5.2));
            b.move_to(at(10.6, 2.2));
            b.line_to(at(10.6, 5.2));
            // Date dots.
            b.move_to(at(5.4, 8.2));
            b.line_to(at(5.9, 8.2));
            b.move_to(at(8., 8.2));
            b.line_to(at(8.5, 8.2));
            b.move_to(at(10.6, 8.2));
            b.line_to(at(11.1, 8.2));
            b.move_to(at(5.4, 11.));
            b.line_to(at(5.9, 11.));
            b.move_to(at(8., 11.));
            b.line_to(at(8.5, 11.));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(14.))
    .flex_none()
    .into_any_element()
}

fn chevron_down(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let c = bounds.center();
            let s: f32 = bounds.size.width.into();
            let mut b = PathBuilder::stroke(px(1.2));
            b.move_to(c + point(px(-s * 0.3), px(-s * 0.14)));
            b.line_to(c + point(px(0.), px(s * 0.16)));
            b.line_to(c + point(px(s * 0.3), px(-s * 0.14)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(10.))
    .flex_none()
    .into_any_element()
}

// ── Render ────────────────────────────────────────────────────────────────

impl Render for DatePicker {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Apply a calendar pick to the typed-date field (subscribe callbacks
        // can't reach a `Window`, render can).
        if let (Some(input), Some(text)) = (self.date_input.clone(), self.pending_input_text.take())
        {
            input.update(cx, |input, cx| input.set_value(text, window, cx));
        }
        let dark = self.dark;
        let ink = rgb(if dark { 0xededed } else { 0x171717 });
        let muted = rgb(if dark { 0xa3a3a3 } else { 0x737373 });
        let edge = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 });
        let card = rgb(if dark { 0x232323 } else { 0xffffff });

        let (text, empty) = self.trigger_label();

        // ── Trigger ─────────────────────────────────────────────────────
        // For the typed-input variant only the icon button toggles — the
        // input itself types dates and must not bubble a toggle click.
        let mut trigger = div()
            .h(TRIGGER_H)
            .px_3()
            .rounded_md()
            .border_1()
            .border_color(edge)
            .bg(card)
            .flex()
            .items_center()
            .gap_2()
            .text_sm()
            .when(self.disabled, |t| t.opacity(0.5));
        if let Some(input) = &self.date_input {
            trigger = trigger
                .child(
                    div()
                        .id(("dp-icon", self.id))
                        .size(px(20.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| {
                                window.prevent_default();
                                cx.stop_propagation();
                                this.open = !this.open;
                                cx.notify();
                            }),
                        )
                        .child(calendar_glyph(if empty {
                            muted.into()
                        } else {
                            ink.into()
                        })),
                )
                .child(div().flex_1().min_w_0().child(Input::new(input)));
        } else {
            trigger = trigger
                .child(calendar_glyph(if empty {
                    muted.into()
                } else {
                    ink.into()
                }))
                .child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .text_color(if empty { muted } else { ink })
                        .child(text),
                )
                .child(chevron_down(muted.into()));
            if !self.disabled {
                trigger = trigger.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        window.prevent_default();
                        this.open = !this.open;
                        cx.notify();
                    }),
                );
            }
        }
        // Capture the trigger rect for popover anchoring — the handler sits
        // on a wrapper so first() reports the trigger's own bounds.
        let trigger_cell = self.trigger_bounds.clone();
        let trigger = div()
            .on_children_prepainted(move |children_bounds, _, _| {
                if let Some(b) = children_bounds.first() {
                    *trigger_cell.borrow_mut() = *b;
                }
            })
            .child(trigger);

        // ── Field label ─────────────────────────────────────────────────
        let mut field = div().flex().flex_col().gap_1p5();
        if let Some(label) = &self.label {
            field = field.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(ink)
                    .child(label.clone()),
            );
        }
        let mut field = field.child(
            div()
                .w(px(280.))
                .when(self.mode == CalendarMode::Range, |w| w.w(px(400.)))
                .child(trigger),
        );
        if self.natural_input.is_some() {
            // Natural variant renders its own input + note instead.
            field = div().flex().flex_col().gap_1p5();
            if let Some(label) = &self.label {
                field = field.child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(ink)
                        .child(label.clone()),
                );
            }
            let input = self.natural_input.clone().unwrap();
            field = field.child(
                div()
                    .w(px(280.))
                    .h(TRIGGER_H)
                    .px_3()
                    .rounded_md()
                    .border_1()
                    .border_color(edge)
                    .bg(card)
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(div().flex_1().min_w_0().child(Input::new(&input)))
                    .child(calendar_glyph(muted.into())),
            );
            if let Some(note) = &self.natural_note {
                field = field.child(div().text_xs().text_color(muted).child(note.clone()));
            }
        }

        // ── Popover ─────────────────────────────────────────────────────
        let bounds_cell = self.root_bounds.clone();
        let mut root = div()
            .size_full()
            .flex()
            .flex_col()
            .font_family(FONT)
            .on_children_prepainted(move |children_bounds, _, _| {
                if let Some(b) = children_bounds.first() {
                    *bounds_cell.borrow_mut() = *b;
                }
            })
            .child(div().flex().flex_col().w_full().items_center().child(field));

        if self.open {
            let _root = *self.root_bounds.borrow();
            let tb = *self.trigger_bounds.borrow();
            let mut overlay = div()
                .absolute()
                .inset_0()
                .id(("dp-overlay", self.id))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, _, cx| {
                        this.open = false;
                        cx.notify();
                    }),
                );
            let mut popover = div()
                .rounded_lg()
                .border_1()
                .border_color(edge)
                .bg(card)
                .shadow_lg()
                .p_3()
                .flex()
                .flex_col()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation());
            if let Some(cal) = &self.calendar {
                popover = popover.child(cal.clone());
            }
            if let Some(time_input) = &self.time_input {
                popover = popover.child(
                    div()
                        .mt_2()
                        .pt_2()
                        .border_t_1()
                        .border_color(edge)
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().w(px(44.)).text_sm().text_color(ink).child("Time"))
                        .child(
                            div()
                                .w(px(80.))
                                .h(px(28.))
                                .px_2()
                                .rounded_md()
                                .border_1()
                                .border_color(edge)
                                .bg(card)
                                .flex()
                                .items_center()
                                .child(Input::new(time_input)),
                        ),
                );
            }
            // Anchor below the trigger, clamped inside the component.
            let local_x = tb.origin.x - _root.origin.x;
            let local_y = tb.origin.y - _root.origin.y + tb.size.height + px(6.);
            let popover_w = if self.mode == CalendarMode::Range {
                px(560.)
            } else {
                px(296.)
            };
            let max_x = if _root.size.width > popover_w {
                _root.size.width - popover_w
            } else {
                px(0.)
            };
            let local_x = clamp_p(local_x, px(0.), max_x);
            overlay = overlay.child(
                div()
                    .absolute()
                    .left(local_x)
                    .top(local_y)
                    .child(deferred(popover)),
            );
            root = root.child(overlay);
        }

        root
    }
}

fn clamp_p(v: Pixels, lo: Pixels, hi: Pixels) -> Pixels {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

fn palette(dark: bool) -> (Hsla, Hsla, Hsla, Hsla) {
    (
        rgb(if dark { 0xededed } else { 0x171717 }).into(),
        rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        rgb(if dark { 0x232323 } else { 0xffffff }).into(),
        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into(),
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

struct DatePickerDemo {
    picker: Entity<DatePicker>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl DatePickerDemo {
    fn new(variant: &str, dark: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let picker: Entity<DatePicker> = match variant {
            "range" => cx.new(|cx| {
                let cal = cx.new(|_| {
                    Calendar::new()
                        .mode(CalendarMode::Range)
                        .months(2)
                        .dark(dark)
                });
                DatePicker::new(Some(cal), window, cx)
                    .placeholder("Pick a date range")
                    .dark(dark)
            }),
            "dob" => cx.new(|cx| {
                let cal = cx.new(|_| {
                    Calendar::new()
                        .dropdown(true)
                        .visible_month(1990, 1)
                        .dark(dark)
                });
                DatePicker::new(Some(cal), window, cx)
                    .label("Date of birth")
                    .placeholder("Select date")
                    .dark(dark)
            }),
            "input" => cx.new(|cx| {
                let cal = cx.new(|_| Calendar::new().dark(dark));
                let input = cx.new(|cx| {
                    let mut s = InputState::new(window, cx).placeholder("Select date".to_string());
                    s.set_editor_style(editor_style(dark));
                    s
                });
                DatePicker::new(Some(cal), window, cx)
                    .label("Subscription Date")
                    .date_input(input, cx)
                    .dark(dark)
            }),
            "time" => cx.new(|cx| {
                let cal = cx.new(|_| Calendar::new().dark(dark));
                let input = cx.new(|cx| {
                    let mut s = InputState::new(window, cx).placeholder("9:00".to_string());
                    s.set_editor_style(editor_style(dark));
                    s
                });
                DatePicker::new(Some(cal), window, cx)
                    .label("Date")
                    .placeholder("Select date")
                    .time_input(input, cx)
                    .dark(dark)
            }),
            "natural" => cx.new(|cx| {
                let input = cx.new(|cx| {
                    let mut s =
                        InputState::new(window, cx).placeholder("E.g. tomorrow".to_string());
                    s.set_editor_style(editor_style(dark));
                    s
                });
                DatePicker::new(None, window, cx)
                    .label("Schedule Date")
                    .natural_input(input, cx)
                    .dark(dark)
            }),
            _ => cx.new(|cx| {
                let cal = cx.new(|_| Calendar::new().dark(dark));
                DatePicker::new(Some(cal), window, cx).dark(dark)
            }),
        };
        let sub = cx.subscribe(&picker, |demo, _, ev: &DatePickerEvent, cx| {
            demo.outcome = Some(
                match ev.selection {
                    Some(DatePick::Single(d)) => format!("Picked {}", fmt_long(d)),
                    Some(DatePick::Range { start, end }) => match end {
                        Some(e) => format!("Range {} – {}", fmt_long(start), fmt_long(e)),
                        None => format!("From {} –", fmt_long(start)),
                    },
                    None => " ".to_string(),
                }
                .into(),
            );
            cx.notify();
        });
        Self {
            picker,
            dark,
            outcome: None,
            _subscriptions: vec![sub],
        }
    }
}

impl Render for DatePickerDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, _card, surface) = palette(self.dark);
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .p_6()
            .font_family(FONT)
            .bg(surface)
            .child(
                div()
                    .pt(px(60.))
                    .w_full()
                    .flex()
                    .justify_center()
                    .child(self.picker.clone()),
            )
            .child(
                div()
                    .mt_4()
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
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(560.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |window, cx| {
        cx.new(|cx| DatePickerDemo::new(&variant, dark, window, cx))
    })
    .expect("open date-picker gallery");
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
    use super::{
        Date, DatePick, DatePicker, DatePickerEvent, parse_date, parse_natural, parse_time,
    };
    use crate::calendar::{Calendar, CalendarMode};
    use gpui_kit::gpui;
    use gpui_kit::gpui::{
        AppContext, Context, Entity, Subscription, TestAppContext, VisualTestContext, Window,
    };

    fn picker(
        cx: &mut TestAppContext,
        build: impl FnOnce(&mut Window, &mut Context<DatePicker>) -> DatePicker,
    ) -> (Entity<DatePicker>, &mut VisualTestContext) {
        cx.update(gpui_kit::base::init);
        cx.add_window_view(move |window, cx| build(window, cx))
    }

    #[gpui::test]
    fn parses_dates_times_and_natural_language(_cx: &mut TestAppContext) {
        let today = Date::today();
        assert_eq!(parse_date("2026-09-30"), Some(Date::new(2026, 9, 30)));
        assert_eq!(parse_date("9/30/2026"), Some(Date::new(2026, 9, 30)));
        assert_eq!(parse_date("30/9/2026"), None); // day > 31 handled? no — 30/9 is M/D only
        assert_eq!(parse_date("not a date"), None);
        assert_eq!(parse_time("9:00"), Some((9, 0)));
        assert_eq!(parse_time("9:30 pm"), Some((21, 30)));
        assert_eq!(parse_time("12 am"), Some((0, 0)));
        assert_eq!(parse_time("25:99"), None);
        assert_eq!(parse_natural("tomorrow"), Some(today.add_days(1)));
        assert_eq!(parse_natural("in 3 days"), Some(today.add_days(3)));
        assert_eq!(parse_natural("in 2 weeks"), Some(today.add_days(14)));
        assert_eq!(parse_natural("next month"), Some(today.add_days(30)));
        assert!(parse_natural("garbage").is_none());
    }

    #[gpui::test]
    fn single_pick_updates_label_and_closes(cx: &mut TestAppContext) {
        let (entity, cx) = picker(cx, |window, cx| {
            let cal = cx.new(|_| Calendar::new());
            DatePicker::new(Some(cal), window, cx)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.set_open(true, cx);
                this.on_calendar_pick(
                    crate::calendar::CalendarSelection::Single(Date::new(2026, 9, 30)),
                    cx,
                );
                assert_eq!(this.date(), Some(Date::new(2026, 9, 30)));
                assert!(!this.open); // single pick closes
                let (text, empty) = this.trigger_label();
                assert_eq!(text.as_ref(), "September 30, 2026");
                assert!(!empty);
            });
        });
    }

    #[gpui::test]
    fn range_pick_stays_open_until_end(cx: &mut TestAppContext) {
        let (entity, cx) = picker(cx, |window, cx| {
            let cal = cx.new(|_| Calendar::new().mode(CalendarMode::Range));
            DatePicker::new(Some(cal), window, cx)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.set_open(true, cx);
                this.on_calendar_pick(
                    crate::calendar::CalendarSelection::Range {
                        start: Date::new(2026, 1, 20),
                        end: None,
                    },
                    cx,
                );
                assert!(this.open); // mid-range stays open
                let (text, _) = this.trigger_label();
                assert_eq!(text.as_ref(), "January 20, 2026 –");
                this.on_calendar_pick(
                    crate::calendar::CalendarSelection::Range {
                        start: Date::new(2026, 1, 20),
                        end: Some(Date::new(2026, 2, 9)),
                    },
                    cx,
                );
                assert!(!this.open);
                assert_eq!(
                    this.range(),
                    Some((Date::new(2026, 1, 20), Some(Date::new(2026, 2, 9))))
                );
            });
        });
    }

    #[gpui::test]
    fn calendar_pick_emits_event(cx: &mut TestAppContext) {
        use std::sync::Arc;
        use std::sync::Mutex;
        let last = Arc::new(Mutex::new(String::new()));
        let (entity, cx) = picker(cx, |window, cx| {
            let cal = cx.new(|_| Calendar::new());
            DatePicker::new(Some(cal), window, cx)
        });
        let last2 = last.clone();
        let _sub: Subscription = cx.update(|_, cx| {
            cx.subscribe(&entity, move |_, ev: &DatePickerEvent, _| {
                if let Some(DatePick::Single(d)) = ev.selection {
                    *last2.lock().unwrap() = format!("{}-{}-{}", d.year, d.month, d.day);
                }
            })
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.on_calendar_pick(
                    crate::calendar::CalendarSelection::Single(Date::new(2026, 12, 25)),
                    cx,
                );
            });
        });
        assert_eq!(last.lock().unwrap().as_str(), "2026-12-25");
    }
}
