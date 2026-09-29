//! shadcn/ui-style calendar for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/calendar
use crate::button::{Button, ButtonIcon, ButtonSize, ButtonVariant, IconPos};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, base::Button as BaseButton, *};
use std::borrow::Cow;

const FONT: &str = "Geist";
const CELL: f32 = 32.;

/// Proleptic Gregorian date (no external date crate needed).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
    /// Days since 1970-01-01 (Howard Hinnant's civil algorithm).
    fn ordinal(self) -> i64 {
        let y = if self.month <= 2 {
            self.year - 1
        } else {
            self.year
        } as i64;
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = (self.month as i64 + 9) % 12;
        let doy = (153 * mp + 2) / 5 + self.day as i64 - 1;
        era * 146097 + yoe * 365 + yoe / 4 - yoe / 100 + doy - 719468
    }
    fn from_ordinal(z: i64) -> Self {
        let z = z + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        Self::new((if month <= 2 { y + 1 } else { y }) as i32, month, day)
    }
    pub fn add_days(self, days: i64) -> Self {
        Self::from_ordinal(self.ordinal() + days)
    }
    /// 0 = Sunday.
    pub(crate) fn weekday(self) -> u32 {
        ((self.ordinal() + 4).rem_euclid(7)) as u32
    }
    fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
            2 => 28,
            _ => 30,
        }
    }
    pub(crate) fn month_name(month: u32) -> &'static str {
        [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ][(month - 1) as usize]
    }
    /// Local today. `SystemTime` panics on wasm32, so the browser path goes
    /// through `js_sys::Date` instead.
    pub fn today() -> Self {
        #[cfg(target_family = "wasm")]
        {
            let d = js_sys::Date::new_0();
            Self::new(d.get_full_year() as i32, d.get_month() + 1, d.get_date())
        }
        #[cfg(not(target_family = "wasm"))]
        {
            let days = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
                / 86400;
            Self::from_ordinal(days)
        }
    }
}

/// What a click selected, emitted as `CalendarSelectEvent`.
#[derive(Clone, Copy, Debug)]
pub enum CalendarSelection {
    Single(Date),
    Range { start: Date, end: Option<Date> },
}

#[derive(Clone, Copy, Debug)]
pub struct CalendarSelectEvent {
    pub selection: CalendarSelection,
}
impl gpui_kit::EventEmitter<CalendarSelectEvent> for Calendar {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CalendarMode {
    Single,
    Range,
}

/// Which caption dropdown is open (shadcn's `captionLayout="dropdown"`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum DropKind {
    Month,
    Year,
}

/// A month-grid calendar entity. Single mode picks one date; Range mode picks
/// start then end (a third click restarts). `months(2)` renders two months
/// side by side like shadcn's RangeCalendar.
pub struct Calendar {
    visible: (i32, u32),
    mode: CalendarMode,
    months: usize,
    selected: Option<Date>,
    range_start: Option<Date>,
    range_end: Option<Date>,
    disabled: Vec<Date>,
    dark: bool,
    dropdown: bool,
    open: Option<DropKind>,
}

impl Calendar {
    pub fn new() -> Self {
        let t = Date::today();
        Self {
            visible: (t.year, t.month),
            mode: CalendarMode::Single,
            months: 1,
            selected: None,
            range_start: None,
            range_end: None,
            disabled: Vec::new(),
            dark: false,
            dropdown: false,
            open: None,
        }
    }
    pub fn mode(mut self, mode: CalendarMode) -> Self {
        self.mode = mode;
        self
    }
    /// Number of month columns (1 or 2).
    pub fn months(mut self, months: usize) -> Self {
        self.months = months.clamp(1, 2);
        self
    }
    /// First visible month; defaults to the month containing today.
    pub fn visible_month(mut self, year: i32, month: u32) -> Self {
        self.visible = (year, month);
        self
    }
    pub fn selected(mut self, date: Date) -> Self {
        self.selected = Some(date);
        self
    }
    pub fn range(mut self, start: Date, end: Date) -> Self {
        self.range_start = Some(start);
        self.range_end = Some(end);
        self
    }
    /// Dates that cannot be picked (shadcn's "booked dates").
    pub fn disabled_dates(mut self, dates: Vec<Date>) -> Self {
        self.disabled = dates;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
    /// Show "September ⌄ / 2026 ⌄" caption selects instead of a text caption.
    pub fn dropdown(mut self, dropdown: bool) -> Self {
        self.dropdown = dropdown;
        self
    }
    /// Current mode (single or range) — used by the date picker wrapper.
    pub(crate) fn mode_of(&self) -> CalendarMode {
        self.mode
    }
    /// Programmatically set the single-mode selection.
    pub fn set_selected(&mut self, date: Date, cx: &mut Context<Self>) {
        self.selected = Some(date);
        self.visible = (date.year, date.month);
        cx.notify();
    }

    fn shift_month(&mut self, delta: i64) {
        let total = self.visible.0 as i64 * 12 + (self.visible.1 as i64 - 1) + delta;
        self.visible = (
            total.div_euclid(12) as i32,
            (total.rem_euclid(12) + 1) as u32,
        );
    }

    fn pick(&mut self, date: Date, cx: &mut Context<Self>) {
        let selection = match self.mode {
            CalendarMode::Single => {
                self.selected = Some(date);
                CalendarSelection::Single(date)
            }
            CalendarMode::Range => {
                if self.range_start.is_none() || self.range_end.is_some() {
                    self.range_start = Some(date);
                    self.range_end = None;
                } else {
                    let start = self.range_start.unwrap();
                    if date < start {
                        self.range_start = Some(date);
                    } else {
                        self.range_end = Some(date);
                    }
                }
                CalendarSelection::Range {
                    start: self.range_start.unwrap(),
                    end: self.range_end,
                }
            }
        };
        // Clicking a day outside the visible month(s) moves the view to it.
        let ym = self.visible.0 * 12 + self.visible.1 as i32;
        let dym = date.year * 12 + date.month as i32;
        if dym < ym || dym > ym + self.months as i32 - 1 {
            self.visible = (date.year, date.month);
        }
        cx.emit(CalendarSelectEvent { selection });
        cx.notify();
    }

    fn day_cell(
        &self,
        date: Date,
        outside: bool,
        today: Date,
        dark: bool,
        weak: &WeakEntity<Self>,
        _cx: &App,
    ) -> AnyElement {
        let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0x737373 } else { 0xa3a3a3 }).into();
        let accent: Hsla = rgb(if dark { 0x262626 } else { 0xf0f0f0 }).into();
        let primary: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let primary_ink: Hsla = rgb(if dark { 0x171717 } else { 0xffffff }).into();
        let disabled = self.disabled.contains(&date);
        let is_today = date == today;
        let is_selected = self.selected == Some(date);
        let is_start = self.range_start == Some(date);
        let is_end = self.range_end == Some(date);
        let in_middle = match (self.range_start, self.range_end) {
            (Some(s), Some(e)) => date > s && date < e,
            _ => false,
        };

        let mut cell = BaseButton::new(SharedString::from(format!(
            "day-{}-{:02}-{:02}",
            date.year, date.month, date.day
        )))
        .disabled(disabled)
        .flex_none()
        .size(px(CELL))
        .flex()
        .items_center()
        .justify_center()
        .font_family(FONT)
        .text_sm()
        .rounded(px(6.))
        .cursor_pointer();

        if in_middle {
            cell = cell
                .rounded(px(0.))
                .bg(accent)
                .text_color(if outside { muted } else { ink });
        } else if is_selected || is_start || is_end {
            cell = cell.bg(primary).text_color(primary_ink);
        } else {
            cell = cell
                .text_color(if outside || disabled { muted } else { ink })
                .when(is_today, |style| style.bg(accent))
                .hover(|style| style.bg(accent))
                .focus_visible(|style| style.border_1().border_color(ink));
        }
        if disabled {
            cell = cell.opacity(0.45);
        }
        if !disabled {
            let weak = weak.clone();
            cell = cell.on_click(move |_, _, cx| {
                if let Some(cal) = weak.upgrade() {
                    let _ = cal.update(cx, |this, cx| this.pick(date, cx));
                }
            });
        }
        cell.child(date.day.to_string()).into_any_element()
    }

    fn month_grid(
        &self,
        ym: (i32, u32),
        show_prev: bool,
        show_next: bool,
        dark: bool,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
        let muted: Hsla = rgb(if dark { 0x737373 } else { 0xa3a3a3 }).into();
        let today = Date::today();
        let weak = cx.entity().downgrade();

        let nav_btn = |id: &str, a11y: &str, icon: ButtonIcon, delta: i64| {
            let weak = weak.clone();
            Button::new(SharedString::from(id.to_string()))
                .a11y_label(a11y)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconSm)
                .icon(icon, IconPos::Start)
                .dark(dark)
                .on_click(move |_, _, cx| {
                    if let Some(cal) = weak.upgrade() {
                        let _ = cal.update(cx, |this, cx| {
                            this.shift_month(delta);
                            cx.notify();
                        });
                    }
                })
        };
        let spacer = || div().flex_none().size(px(CELL));

        // Header: ‹ · caption · ›, caption centered between fixed slots.
        // With `dropdown` the caption becomes "September ⌄ · 2026 ⌄" selects.
        let caption = format!("{} {}", Date::month_name(ym.1), ym.0);
        let caption_el: AnyElement = if self.dropdown {
            let mk = |id: &str, label: String, kind: DropKind| {
                let weak = weak.clone();
                Button::new(SharedString::from(id.to_string()))
                    .label(label)
                    .icon(ButtonIcon::ChevronDown, IconPos::End)
                    .variant(ButtonVariant::Ghost)
                    .size(ButtonSize::Xs)
                    .dark(dark)
                    .on_click(move |_, _, cx| {
                        if let Some(cal) = weak.upgrade() {
                            let _ = cal.update(cx, |this, cx| {
                                this.open = if this.open == Some(kind) {
                                    None
                                } else {
                                    Some(kind)
                                };
                                cx.notify();
                            });
                        }
                    })
                    .into_any_element()
            };
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .gap(px(4.))
                .child(mk(
                    "cal-month",
                    Date::month_name(ym.1).to_string(),
                    DropKind::Month,
                ))
                .child(mk("cal-year", format!("{}", ym.0), DropKind::Year))
                .into_any_element()
        } else {
            div()
                .flex_1()
                .flex()
                .items_center()
                .justify_center()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .text_color(ink)
                .whitespace_nowrap()
                .child(caption)
                .into_any_element()
        };
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .h(px(CELL))
            .child(if show_prev {
                nav_btn("cal-prev", "Previous month", ButtonIcon::ChevronLeft, -1)
                    .into_any_element()
            } else {
                spacer().into_any_element()
            })
            .child(caption_el)
            .child(if show_next {
                nav_btn("cal-next", "Next month", ButtonIcon::ChevronRight, 1).into_any_element()
            } else {
                spacer().into_any_element()
            });

        // Weekday row.
        let weekdays = div().flex().flex_row().w_full().children(
            ["S", "M", "T", "W", "T", "F", "S"].into_iter().map(|d| {
                div()
                    .flex_none()
                    .size(px(CELL))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(muted)
                    .child(d)
            }),
        );

        // Day rows: outside days from the adjacent months are shown dimmed.
        let first = Date::new(ym.0, ym.1, 1);
        let lead = first.weekday() as i64;
        let days = Date::days_in_month(ym.0, ym.1) as i64;
        let cells = lead + days;
        let total = (cells + 6) / 7 * 7;
        let mut rows = Vec::new();
        for row in 0..(total / 7) {
            let mut week = Vec::new();
            for col in 0..7 {
                let idx = (row * 7 + col) as i64;
                let date = first.add_days(idx - lead);
                let outside = idx < lead || idx >= lead + days;
                week.push(self.day_cell(date, outside, today, dark, &weak, cx));
            }
            rows.push(div().flex().flex_row().w_full().mt(px(8.)).children(week));
        }

        // Popup for the caption dropdowns, anchored under the header. Rendered
        // only on the first month grid (the one owning the visible month).
        let popup: Option<AnyElement> = if self.dropdown && ym == self.visible {
            self.open.map(|kind| {
                let surface: Hsla = rgb(if dark { 0x232323 } else { 0xffffff }).into();
                let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
                let items: Vec<(String, i32, u32)> = match kind {
                    DropKind::Month => (1..=12u32)
                        .map(|m| (Date::month_name(m)[..3].to_string(), ym.0, m))
                        .collect(),
                    DropKind::Year => ((ym.0 - 5)..=(ym.0 + 6))
                        .map(|y| (format!("{}", y), y, ym.1))
                        .collect(),
                };
                let mut grid = div().flex().flex_row().flex_wrap().w(px(CELL * 7.));
                for (label, y, m) in items {
                    let is_current = y == ym.0 && m == ym.1;
                    let weak = weak.clone();
                    let b = Button::new(SharedString::from(format!("drop-{}-{}-{}", y, m, label)))
                        .label(label)
                        .variant(if is_current {
                            ButtonVariant::Secondary
                        } else {
                            ButtonVariant::Ghost
                        })
                        .size(ButtonSize::Xs)
                        .dark(dark)
                        .on_click(move |_, _, cx| {
                            if let Some(cal) = weak.upgrade() {
                                let _ = cal.update(cx, |this, cx| {
                                    this.visible = (y, m);
                                    this.open = None;
                                    cx.notify();
                                });
                            }
                        });
                    grid = grid.child(div().w(px(CELL * 7. / 4.)).flex().justify_center().child(b));
                }
                div()
                    .absolute()
                    .top(px(CELL + 4.))
                    .left_0()
                    .right_0()
                    .flex()
                    .justify_center()
                    .block_mouse_except_scroll()
                    .child(
                        div()
                            .bg(surface)
                            .border_1()
                            .border_color(edge)
                            .rounded_md()
                            .shadow_md()
                            .p_1()
                            .child(grid),
                    )
                    .into_any_element()
            })
        } else {
            None
        };

        div()
            .flex_none()
            .flex()
            .flex_col()
            .relative()
            .gap(px(16.))
            .w(px(CELL * 7.))
            .child(header)
            .child(div().flex().flex_col().child(weekdays).children(rows))
            .children(popup)
            .into_any_element()
    }
}

impl Render for Calendar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let surface: Hsla = rgb(if self.dark { 0x232323 } else { 0xffffff }).into();
        let edge: Hsla = rgb(if self.dark { 0x3b3b3b } else { 0xe4e4e4 }).into();
        let (y, m) = self.visible;
        let mut month2 = (y, m + 1);
        if month2.1 > 12 {
            month2 = (y + 1, 1);
        }
        div()
            .flex_none()
            .flex()
            .flex_row()
            .gap(px(16.))
            .bg(surface)
            .border_1()
            .border_color(edge)
            .rounded_lg()
            .p_3()
            .font_family(FONT)
            .child(self.month_grid((y, m), true, false, self.dark, cx))
            .when(self.months == 2, |style| {
                style.child(self.month_grid(month2, false, true, self.dark, cx))
            })
    }
}

/// Ghost preset buttons column for the Presets demo.
fn preset_column(dark: bool, weak: WeakEntity<Calendar>) -> impl IntoElement {
    let presets = [
        ("Today", 0i64),
        ("Tomorrow", 1),
        ("In 3 days", 3),
        ("In a week", 7),
        ("In 2 weeks", 14),
    ];
    div()
        .flex_none()
        .flex()
        .flex_col()
        .gap_1()
        .children(presets.into_iter().map(|(label, days)| {
            let weak = weak.clone();
            Button::new(SharedString::from(format!("preset-{days}")))
                .label(label)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Sm)
                .dark(dark)
                .on_click(move |_, _, cx| {
                    if let Some(cal) = weak.upgrade() {
                        let _ = cal.update(cx, |this, cx| {
                            let d = Date::today().add_days(days);
                            this.selected = Some(d);
                            this.visible = (d.year, d.month);
                            cx.emit(CalendarSelectEvent {
                                selection: CalendarSelection::Single(d),
                            });
                            cx.notify();
                        });
                    }
                })
                .into_any_element()
        }))
}

/// Hour/minute stepper column for the Date & Time composition demo.
fn time_column(dark: bool, time: (u32, u32), weak: WeakEntity<CalendarDemo>) -> impl IntoElement {
    let ink: Hsla = rgb(if dark { 0xededed } else { 0x171717 }).into();
    let muted: Hsla = rgb(if dark { 0x737373 } else { 0xa3a3a3 }).into();
    let edge: Hsla = rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into();

    fn field(
        id: &str,
        label: &str,
        value: u32,
        max: u32,
        dark: bool,
        ink: Hsla,
        muted: Hsla,
        edge: Hsla,
        weak: WeakEntity<CalendarDemo>,
        pick: fn(&mut (u32, u32)) -> &mut u32,
    ) -> impl IntoElement {
        let step = move |delta: i64| {
            let weak = weak.clone();
            Button::new(SharedString::from(format!("{id}-{delta}")))
                .icon(
                    if delta > 0 {
                        ButtonIcon::ChevronUp
                    } else {
                        ButtonIcon::ChevronDown
                    },
                    IconPos::Start,
                )
                .a11y_label(format!("{label} {delta:+}"))
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconXs)
                .dark(dark)
                .on_click(move |_, _, cx| {
                    if let Some(demo) = weak.upgrade() {
                        let _ = demo.update(cx, |this, cx| {
                            let v = pick(&mut this.time);
                            *v = ((*v as i64 + delta).rem_euclid(max as i64)) as u32;
                            cx.notify();
                        });
                    }
                })
                .into_any_element()
        };
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(2.))
            .child(div().text_xs().text_color(muted).child(label.to_string()))
            .child(step(1))
            .child(
                div()
                    .w(px(56.))
                    .h(px(32.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_1()
                    .border_color(edge)
                    .rounded_md()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(ink)
                    .child(format!("{value:02}")),
            )
            .child(step(-1))
    }

    div()
        .flex_none()
        .flex()
        .flex_col()
        .gap_2()
        .pt_6()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(muted)
                .child("Time"),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_start()
                .gap_2()
                .child(field(
                    "hour",
                    "Hours",
                    time.0,
                    24,
                    dark,
                    ink,
                    muted,
                    edge,
                    weak.clone(),
                    |t| &mut t.0,
                ))
                .child(div().pt(px(56.)).text_sm().text_color(muted).child(":"))
                .child(field(
                    "minute",
                    "Minutes",
                    time.1,
                    60,
                    dark,
                    ink,
                    muted,
                    edge,
                    weak,
                    |t| &mut t.1,
                )),
        )
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Range,
    Presets,
    Booked,
    Dropdown,
    DateTime,
}

pub struct CalendarDemo {
    variant: String,
    dark: bool,
    calendar: Entity<Calendar>,
    outcome: Option<SharedString>,
    /// HH:MM for the DateTime composition demo.
    time: (u32, u32),
}

impl CalendarDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let today = Date::today();
        let kind = match variant {
            "range" => DemoKind::Range,
            "presets" => DemoKind::Presets,
            "booked" => DemoKind::Booked,
            "dropdown" => DemoKind::Dropdown,
            "datetime" => DemoKind::DateTime,
            _ => DemoKind::Basic,
        };
        let calendar = cx.new(|_| {
            let mut cal = Calendar::new().dark(dark);
            match kind {
                DemoKind::Range => {
                    cal = cal
                        .mode(CalendarMode::Range)
                        .months(2)
                        .range(today.add_days(4), today.add_days(11));
                }
                DemoKind::Basic => cal = cal.selected(today),
                DemoKind::Presets => cal = cal.selected(today),
                DemoKind::Dropdown => {
                    cal = cal.selected(today).dropdown(true);
                }
                DemoKind::DateTime => cal = cal.selected(today),
                DemoKind::Booked => {
                    cal = cal.disabled_dates(vec![
                        today.add_days(3),
                        today.add_days(5),
                        today.add_days(6),
                        today.add_days(11),
                        today.add_days(12),
                    ]);
                }
            }
            cal
        });
        cx.subscribe(
            &calendar,
            |this: &mut Self, _cal, event: &CalendarSelectEvent, cx| {
                this.outcome = Some(
                    match event.selection {
                        CalendarSelection::Single(d) => {
                            format!("Selected {:04}-{:02}-{:02}", d.year, d.month, d.day)
                        }
                        CalendarSelection::Range { start, end } => match end {
                            Some(e) => format!(
                                "Range {:04}-{:02}-{:02} → {:04}-{:02}-{:02}",
                                start.year, start.month, start.day, e.year, e.month, e.day
                            ),
                            None => format!(
                                "Range start {:04}-{:02}-{:02}",
                                start.year, start.month, start.day
                            ),
                        },
                    }
                    .into(),
                );
                cx.notify();
            },
        )
        .detach();
        Self {
            variant: variant.to_string(),
            dark,
            calendar,
            outcome: None,
            time: (9, 30),
        }
    }
}

impl Render for CalendarDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let kind = match self.variant.as_str() {
            "range" => DemoKind::Range,
            "presets" => DemoKind::Presets,
            "booked" => DemoKind::Booked,
            "dropdown" => DemoKind::Dropdown,
            "datetime" => DemoKind::DateTime,
            _ => DemoKind::Basic,
        };
        let weak = self.calendar.downgrade();
        let content: AnyElement = match kind {
            DemoKind::Presets => div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(16.))
                .child(self.calendar.clone())
                .child(preset_column(self.dark, weak))
                .into_any_element(),
            DemoKind::DateTime => div()
                .flex()
                .flex_row()
                .items_start()
                .gap(px(16.))
                .child(self.calendar.clone())
                .child(time_column(self.dark, self.time, cx.entity().downgrade()))
                .into_any_element(),
            _ => self.calendar.clone().into_any_element(),
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
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(460.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |_, cx| {
        cx.new(|cx| CalendarDemo::new(&variant, dark, cx))
    })
    .expect("open calendar gallery");
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
    cx.new(|cx| CalendarDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::Date;

    #[test]
    fn civil_round_trip() {
        let d = Date::new(2026, 9, 25);
        assert_eq!(Date::from_ordinal(d.ordinal()), d);
        assert_eq!(Date::new(1970, 1, 1).ordinal(), 0);
        assert_eq!(Date::new(2026, 9, 25).weekday(), 5); // Friday
        assert_eq!(Date::days_in_month(2024, 2), 29);
        assert_eq!(Date::days_in_month(2025, 2), 28);
        assert_eq!(Date::new(2026, 1, 31).add_days(1), Date::new(2026, 2, 1));
        assert_eq!(Date::new(2026, 3, 1).add_days(-1), Date::new(2026, 2, 28));
    }

    #[test]
    fn month_shift() {
        let mut c = super::Calendar::new().visible_month(2026, 12);
        c.shift_month(1);
        assert_eq!(c.visible, (2027, 1));
        c.shift_month(-1);
        assert_eq!(c.visible, (2026, 12));
        c.shift_month(-1);
        assert_eq!(c.visible, (2026, 11));
    }
}
