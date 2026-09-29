//! shadcn/ui-style context menu for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/context-menu
//! (React Aria). Right-clicking the trigger opens a floating menu at the
//! pointer; items support icons, shortcuts, checkboxes, radio groups,
//! submenus and a destructive variant.
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

const FONT: &str = "Geist";
const MENU_W: Pixels = px(208.);
const ITEM_H: Pixels = px(30.);
const LABEL_H: Pixels = px(24.);
const SEP_H: Pixels = px(9.);
const PAD_Y: Pixels = px(5.);

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Emitted when an item activates. For checkbox items `checked` carries the
/// new state; for radio items it is `Some(true)` when the row becomes chosen.
#[derive(Clone, Debug, PartialEq)]
pub struct ContextMenuSelectEvent {
    pub value: SharedString,
    /// `Some(checked)` for checkbox rows, `None` for plain actions.
    pub checked: Option<bool>,
    /// Radio group name when the row is a radio item.
    pub group: Option<SharedString>,
}

/// Leading glyphs for menu rows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuIcon {
    ArrowLeft,
    ArrowRight,
    Rotate,
    Save,
    Pencil,
    Share,
    Copy,
    Trash,
    Gear,
    Eye,
    User,
    Window,
}

/// Checkbox / radio / plain behaviour of a `ContextMenuItem`.
#[derive(Clone, Debug, PartialEq)]
enum ItemKind {
    Action,
    Checkbox { checked: bool },
    Radio { group: SharedString },
}

/// One row inside a `ContextMenu` (`ContextMenuItem`).
#[derive(Clone, Debug)]
pub struct ContextMenuItem {
    pub label: SharedString,
    /// Value emitted in `ContextMenuSelectEvent`; defaults to `label`.
    pub value: SharedString,
    /// `ContextMenuShortcut` trailing text (⌘/⇧/⌥/⌃ are drawn glyphs).
    pub shortcut: Option<SharedString>,
    pub icon: Option<MenuIcon>,
    pub disabled: bool,
    /// `variant="destructive"`.
    pub destructive: bool,
    kind: ItemKind,
    /// `ContextMenuSub` children (`ContextMenuSubContent`).
    children: Vec<ContextMenuEntry>,
}

impl ContextMenuItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label,
            shortcut: None,
            icon: None,
            disabled: false,
            destructive: false,
            kind: ItemKind::Action,
            children: Vec::new(),
        }
    }

    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn icon(mut self, icon: MenuIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    /// `selectionMode="multiple"` row.
    pub fn checkbox(mut self, checked: bool) -> Self {
        self.kind = ItemKind::Checkbox { checked };
        self
    }

    /// `selectionMode="single"` row inside a named radio group.
    pub fn radio(mut self, group: impl Into<SharedString>) -> Self {
        self.kind = ItemKind::Radio {
            group: group.into(),
        };
        self
    }

    /// `ContextMenuSub` — this row becomes a `ContextMenuSubTrigger`.
    pub fn submenu(mut self, children: Vec<ContextMenuEntry>) -> Self {
        self.children = children;
        self
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }
}

/// `ContextMenuContent` entries — items, separators and group labels.
#[derive(Clone, Debug)]
pub enum ContextMenuEntry {
    Item(ContextMenuItem),
    /// `ContextMenuSeparator`.
    Separator,
    /// `ContextMenuLabel` — small muted heading.
    Label(SharedString),
}

/// `ContextMenu` — the entity owns trigger, popup state, keyboard focus and
/// the active row. Wrap it around trigger content via `.trigger(..)`.
pub struct ContextMenu {
    trigger: Rc<dyn Fn() -> AnyElement>,
    items: Vec<ContextMenuEntry>,
    /// Pointer position (root-local) where the menu is anchored.
    open_at: Option<Point<Pixels>>,
    /// Active row index in the top-level menu (into `nav`).
    active: Option<usize>,
    /// Top-level item index whose submenu is open.
    open_sub: Option<usize>,
    /// Active row inside the open submenu.
    sub_active: Option<usize>,
    /// Radio group → current value.
    radio: HashMap<String, SharedString>,
    /// Scratch flag set by `commit_at` — checkbox/radio commits keep the
    /// menu open; plain actions dismiss it.
    kept_open: bool,
    /// Root bounds captured each prepaint so window-space pointer
    /// coordinates can be converted to menu-local coordinates.
    root_bounds: Rc<RefCell<Bounds<Pixels>>>,
    focus: FocusHandle,
    dark: bool,
    id: usize,
}

impl EventEmitter<ContextMenuSelectEvent> for ContextMenu {}

impl ContextMenu {
    /// `trigger` is a factory (GPUI elements are render-once).
    pub fn new(
        trigger: impl Fn() -> AnyElement + 'static,
        items: Vec<ContextMenuEntry>,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self {
            trigger: Rc::new(trigger),
            items,
            open_at: None,
            active: None,
            open_sub: None,
            sub_active: None,
            radio: HashMap::new(),
            kept_open: false,
            root_bounds: Rc::new(RefCell::new(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(0.), px(0.)),
            })),
            focus: cx.focus_handle(),
            dark: false,
            id: NEXT_KEY.fetch_add(1, Ordering::Relaxed),
        };
        // Seed radio groups from initial `.radio(..)` markers.
        this.collect_radio_defaults();
        this
    }

    fn collect_radio_defaults(&mut self) {
        for entry in &self.items {
            if let ContextMenuEntry::Item(item) = entry {
                if let ItemKind::Radio { group } = &item.kind {
                    self.radio
                        .entry(group.to_string())
                        .or_insert_with(|| item.value.clone());
                }
            }
        }
    }

    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// Controlled open/close — mirrors `open` + `onOpenChange`.
    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if open {
            let bounds = *self.root_bounds.borrow();
            self.open_at = Some(bounds.center());
            self.active = self
                .items
                .iter()
                .position(|e| matches!(e, ContextMenuEntry::Item(i) if !i.disabled));
            self.focus.focus(window, cx);
        } else {
            self.open_at = None;
            self.open_sub = None;
            self.sub_active = None;
        }
        cx.notify();
    }

    pub fn is_open(&self) -> bool {
        self.open_at.is_some()
    }

    /// Current checked state of a checkbox row (by value).
    pub fn checked(&self, value: &str) -> Option<bool> {
        self.items.iter().find_map(|e| match e {
            ContextMenuEntry::Item(i) if i.value.as_ref() == value => {
                if let ItemKind::Checkbox { checked } = i.kind {
                    Some(checked)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Current value of a radio group.
    pub fn radio_value(&self, group: &str) -> Option<SharedString> {
        self.radio.get(group).cloned()
    }

    fn close(&mut self, cx: &mut Context<Self>) {
        if self.open_at.is_some() {
            self.open_at = None;
            self.open_sub = None;
            self.sub_active = None;
            cx.notify();
        }
    }

    /// Navigable top-level rows: indices into `items` that are items.
    fn nav(&self) -> Vec<usize> {
        self.items
            .iter()
            .enumerate()
            .filter_map(|(i, e)| match e {
                ContextMenuEntry::Item(_) => Some(i),
                _ => None,
            })
            .collect()
    }

    fn sub_nav(&self) -> Vec<usize> {
        self.open_sub
            .and_then(|i| match self.items.get(i) {
                Some(ContextMenuEntry::Item(item)) => Some(item.children.clone()),
                _ => None,
            })
            .map(|children| {
                children
                    .iter()
                    .enumerate()
                    .filter_map(|(i, e)| match e {
                        ContextMenuEntry::Item(_) => Some(i),
                        _ => None,
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    fn activate(&mut self, top_index: usize, sub_index: Option<usize>, cx: &mut Context<Self>) {
        if let Some(sub_i) = sub_index {
            let child_is_item = matches!(
                self.items.get(top_index),
                Some(ContextMenuEntry::Item(item))
                    if matches!(item.children.get(sub_i), Some(ContextMenuEntry::Item(i)) if !i.disabled)
            );
            if !child_is_item {
                return;
            }
            self.commit_at(top_index, Some(sub_i), cx);
            // Checkbox/radio rows keep the menu open; plain actions close.
            if !self.kept_open {
                self.close(cx);
            }
            return;
        }
        let Some(ContextMenuEntry::Item(item)) = self.items.get(top_index) else {
            return;
        };
        if item.disabled {
            return;
        }
        if item.has_children() {
            self.open_sub = Some(top_index);
            self.sub_active = self.sub_nav().first().copied();
            cx.notify();
            return;
        }
        self.commit_at(top_index, None, cx);
        if !self.kept_open {
            self.close(cx);
        }
    }

    /// Toggle/emit for the item at `items[top]` or `items[top].children[sub]`.
    /// Sets `kept_open` so the caller knows whether to dismiss.
    fn commit_at(&mut self, top: usize, sub: Option<usize>, cx: &mut Context<Self>) {
        self.kept_open = false;
        let entry = match sub {
            Some(s) => match self.items.get_mut(top) {
                Some(ContextMenuEntry::Item(item)) => match item.children.get_mut(s) {
                    Some(e) => e,
                    None => return,
                },
                _ => return,
            },
            None => match self.items.get_mut(top) {
                Some(e) => e,
                None => return,
            },
        };
        let ContextMenuEntry::Item(item) = entry else {
            return;
        };
        match item.kind.clone() {
            ItemKind::Checkbox { checked } => {
                let next = !checked;
                item.kind = ItemKind::Checkbox { checked: next };
                cx.emit(ContextMenuSelectEvent {
                    value: item.value.clone(),
                    checked: Some(next),
                    group: None,
                });
                self.kept_open = true;
            }
            ItemKind::Radio { group } => {
                self.radio.insert(group.to_string(), item.value.clone());
                cx.emit(ContextMenuSelectEvent {
                    value: item.value.clone(),
                    checked: Some(true),
                    group: Some(group),
                });
                self.kept_open = true;
            }
            ItemKind::Action => {
                cx.emit(ContextMenuSelectEvent {
                    value: item.value.clone(),
                    checked: None,
                    group: None,
                });
            }
        }
        cx.notify();
    }

    fn move_active(&mut self, delta: isize, cx: &mut Context<Self>) {
        if self.open_sub.is_some() {
            let nav = self.sub_nav();
            if nav.is_empty() {
                return;
            }
            let len = nav.len() as isize;
            let cur = self
                .sub_active
                .and_then(|v| nav.iter().position(|&i| i == v))
                .map(|p| p as isize)
                .unwrap_or(0);
            let mut next = cur + delta;
            for _ in 0..nav.len() {
                let pos = next.rem_euclid(len) as usize;
                let idx = nav[pos];
                if let Some(ContextMenuEntry::Item(i)) = self
                    .open_sub
                    .and_then(|t| self.items.get(t))
                    .and_then(|e| match e {
                        ContextMenuEntry::Item(item) => item.children.get(idx),
                        _ => None,
                    })
                {
                    if !i.disabled {
                        self.sub_active = Some(idx);
                        cx.notify();
                        return;
                    }
                }
                next += delta;
            }
            return;
        }
        let nav = self.nav();
        if nav.is_empty() {
            return;
        }
        let len = nav.len() as isize;
        let cur = self
            .active
            .and_then(|v| nav.iter().position(|&i| i == v))
            .map(|p| p as isize)
            .unwrap_or(0);
        let mut next = cur + delta;
        for _ in 0..nav.len() {
            let pos = next.rem_euclid(len) as usize;
            let idx = nav[pos];
            if let ContextMenuEntry::Item(i) = &self.items[idx] {
                if !i.disabled {
                    self.active = Some(idx);
                    if i.has_children() && delta > 0 {
                        // Keep submenu state untouched during vertical nav.
                    }
                    cx.notify();
                    return;
                }
            }
            next += delta;
        }
    }
}

/// Pixel height of one rendered entry (used for clamping + submenu offset).
fn entry_height(e: &ContextMenuEntry) -> Pixels {
    match e {
        ContextMenuEntry::Item(_) => ITEM_H,
        ContextMenuEntry::Separator => SEP_H,
        ContextMenuEntry::Label(_) => LABEL_H,
    }
}

fn menu_height(items: &[ContextMenuEntry]) -> Pixels {
    items
        .iter()
        .fold(PAD_Y * 2., |acc, e| acc + entry_height(e))
}

/// Y offset (inside the panel) of entry `index`.
fn entry_offset(items: &[ContextMenuEntry], index: usize) -> Pixels {
    items[..index]
        .iter()
        .fold(PAD_Y, |acc, e| acc + entry_height(e))
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

// ── Glyphs ────────────────────────────────────────────────────────────────

fn icon_element(icon: MenuIcon, color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let w = px(1.2 * k.max(0.7));
            let seg = |b: &mut PathBuilder, points: &[(f32, f32)], close: bool| {
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
                MenuIcon::ArrowLeft => {
                    seg(&mut b, &[(11., 8.), (5., 8.)], false);
                    seg(&mut b, &[(8., 4.5), (4.5, 8.), (8., 11.5)], false);
                }
                MenuIcon::ArrowRight => {
                    seg(&mut b, &[(5., 8.), (11., 8.)], false);
                    seg(&mut b, &[(8., 4.5), (11.5, 8.), (8., 11.5)], false);
                }
                MenuIcon::Rotate => {
                    for i in 0..14 {
                        let a0 = 0.6 + i as f32 / 14. * std::f32::consts::TAU * 0.86;
                        let a1 = 0.6 + (i + 1) as f32 / 14. * std::f32::consts::TAU * 0.86;
                        b.move_to(at(8. + a0.cos() * 5.4, 8. + a0.sin() * 5.4));
                        b.line_to(at(8. + a1.cos() * 5.4, 8. + a1.sin() * 5.4));
                    }
                    seg(&mut b, &[(11.4, 2.), (12.6, 4.6), (10.1, 5.4)], false);
                }
                MenuIcon::Save => {
                    seg(
                        &mut b,
                        &[(3., 2.4), (11., 2.4), (13.4, 4.8), (13.4, 13.6), (3., 13.6)],
                        true,
                    );
                    seg(
                        &mut b,
                        &[(5.4, 2.4), (5.4, 6.6), (10.6, 6.6), (10.6, 2.4)],
                        false,
                    );
                    seg(
                        &mut b,
                        &[(5.4, 13.6), (5.4, 9.8), (10.6, 9.8), (10.6, 13.6)],
                        false,
                    );
                }
                MenuIcon::Pencil => {
                    seg(
                        &mut b,
                        &[
                            (10.8, 2.6),
                            (13.4, 5.2),
                            (5.6, 13.),
                            (2.6, 13.4),
                            (3., 10.4),
                        ],
                        true,
                    );
                    seg(&mut b, &[(9.4, 4.), (12., 6.6)], false);
                }
                MenuIcon::Share => {
                    seg(&mut b, &[(5.6, 9.6), (10.4, 5.2)], false);
                    seg(&mut b, &[(5.6, 9.6), (10.4, 13.8)], false);
                    for &(cx_, cy_) in &[(3.6, 9.6), (12.4, 4.6), (12.4, 14.)] {
                        for i in 0..8 {
                            let a0 = i as f32 / 8. * std::f32::consts::TAU;
                            let a1 = (i + 1) as f32 / 8. * std::f32::consts::TAU;
                            b.move_to(at(cx_ + a0.cos() * 1.9, cy_ + a0.sin() * 1.9));
                            b.line_to(at(cx_ + a1.cos() * 1.9, cy_ + a1.sin() * 1.9));
                        }
                    }
                }
                MenuIcon::Copy => {
                    seg(
                        &mut b,
                        &[(5.6, 5.6), (13.4, 5.6), (13.4, 13.4), (5.6, 13.4)],
                        true,
                    );
                    seg(
                        &mut b,
                        &[
                            (10.6, 5.6),
                            (10.6, 2.6),
                            (2.6, 2.6),
                            (2.6, 10.6),
                            (5.6, 10.6),
                        ],
                        false,
                    );
                }
                MenuIcon::Trash => {
                    seg(&mut b, &[(3.4, 4.4), (12.6, 4.4)], false);
                    seg(
                        &mut b,
                        &[(5.2, 4.4), (5.2, 2.8), (10.8, 2.8), (10.8, 4.4)],
                        false,
                    );
                    seg(
                        &mut b,
                        &[(4.4, 4.4), (5.4, 13.4), (10.6, 13.4), (11.6, 4.4)],
                        false,
                    );
                    seg(&mut b, &[(6.8, 7.), (7.2, 11.)], false);
                    seg(&mut b, &[(9.2, 7.), (8.8, 11.)], false);
                }
                MenuIcon::Gear => {
                    for i in 0..12 {
                        let a0 = i as f32 / 12. * std::f32::consts::TAU;
                        let a1 = (i + 1) as f32 / 12. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 3., 8. + a0.sin() * 3.));
                        b.line_to(at(8. + a1.cos() * 3., 8. + a1.sin() * 3.));
                    }
                    for i in 0..8 {
                        let a = i as f32 / 8. * std::f32::consts::TAU;
                        seg(
                            &mut b,
                            &[
                                (8. + a.cos() * 4.4, 8. + a.sin() * 4.4),
                                (8. + a.cos() * 6.2, 8. + a.sin() * 6.2),
                            ],
                            false,
                        );
                    }
                }
                MenuIcon::Eye => {
                    let mut prev = (2., 8.);
                    for i in 1..=12 {
                        let t = i as f32 / 12.;
                        let x = 2. + t * 12.;
                        let y = 8. - (t * (1. - t)) * 10.4;
                        b.move_to(at(prev.0, prev.1));
                        b.line_to(at(x, y));
                        prev = (x, y);
                    }
                    prev = (2., 8.);
                    for i in 1..=12 {
                        let t = i as f32 / 12.;
                        let x = 2. + t * 12.;
                        let y = 8. + (t * (1. - t)) * 10.4;
                        b.move_to(at(prev.0, prev.1));
                        b.line_to(at(x, y));
                        prev = (x, y);
                    }
                    for i in 0..8 {
                        let a0 = i as f32 / 8. * std::f32::consts::TAU;
                        let a1 = (i + 1) as f32 / 8. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 1.9, 8. + a0.sin() * 1.9));
                        b.line_to(at(8. + a1.cos() * 1.9, 8. + a1.sin() * 1.9));
                    }
                }
                MenuIcon::User => {
                    for i in 0..10 {
                        let a0 = i as f32 / 10. * std::f32::consts::TAU;
                        let a1 = (i + 1) as f32 / 10. * std::f32::consts::TAU;
                        b.move_to(at(8. + a0.cos() * 2.8, 5.4 + a0.sin() * 2.8));
                        b.line_to(at(8. + a1.cos() * 2.8, 5.4 + a1.sin() * 2.8));
                    }
                    let mut prev = (3.6f32, 13.4f32);
                    for i in 1..=8 {
                        let t = i as f32 / 8.;
                        let x = 3.6 + t * 8.8;
                        let y = 13.4 - t * (1. - t) * 8.2;
                        b.move_to(at(prev.0, prev.1));
                        b.line_to(at(x, y));
                        prev = (x, y);
                    }
                }
                MenuIcon::Window => {
                    seg(
                        &mut b,
                        &[(2.4, 3.2), (13.6, 3.2), (13.6, 12.8), (2.4, 12.8)],
                        true,
                    );
                    seg(&mut b, &[(2.4, 6.), (13.6, 6.)], false);
                    seg(&mut b, &[(4.2, 4.6), (4.3, 4.6)], false);
                    seg(&mut b, &[(6.2, 4.6), (6.3, 4.6)], false);
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

/// Check glyph for checkbox rows.
fn check_glyph(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let mut b = PathBuilder::stroke(px(1.4 * k));
            b.move_to(o + point(px(3.4 * k), px(8.4 * k)));
            b.line_to(o + point(px(6.8 * k), px(11.6 * k)));
            b.line_to(o + point(px(12.6 * k), px(4.6 * k)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(14.))
    .flex_none()
    .into_any_element()
}

/// Filled dot for radio rows.
fn dot_glyph(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let c = bounds.center();
            let r: f32 = bounds.size.width.into();
            let r = r * 0.22;
            let mut b = PathBuilder::fill();
            for i in 0..12 {
                let a = i as f32 / 12. * std::f32::consts::TAU;
                let p = c + point(px(a.cos() * r), px(a.sin() * r));
                if i == 0 {
                    b.move_to(p);
                } else {
                    b.line_to(p);
                }
            }
            b.line_to(c + point(px(r), px(0.)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(14.))
    .flex_none()
    .into_any_element()
}

/// Right-pointing chevron for `ContextMenuSubTrigger`.
fn sub_chevron(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let c = bounds.center();
            let s: f32 = bounds.size.width.into();
            let mut b = PathBuilder::stroke(px(1.2));
            b.move_to(c + point(px(-s * 0.12), px(-s * 0.3)));
            b.line_to(c + point(px(s * 0.18), px(0.)));
            b.line_to(c + point(px(-s * 0.12), px(s * 0.3)));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(10.))
    .flex_none()
    .into_any_element()
}

/// `⌘`/`⇧`/`⌥`/`⌃` drawn as paths — Geist lacks those codepoints.
fn modifier_icon(ch: char, color: Hsla) -> Option<AnyElement> {
    let points: &[&[(f32, f32)]] = match ch {
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
        '⌥' => &[
            &[(1.8, 4.6), (6.2, 4.6), (10.4, 11.4), (14.2, 11.4)],
            &[(9.6, 4.6), (14.2, 4.6)],
        ],
        '⌃' => &[&[(3.6, 10.6), (8., 3.6), (12.4, 10.6)]],
        // Backspace: bevelled box with an ✕.
        '⌫' => &[
            &[
                (1.8, 5.),
                (5.4, 5.),
                (14.2, 5.),
                (14.2, 11.),
                (5.4, 11.),
                (1.8, 5.),
            ],
            &[(1.8, 5.), (4.6, 8.), (1.8, 11.)],
            &[(6.6, 6.6), (11.4, 9.8)],
            &[(11.4, 6.6), (6.6, 9.8)],
        ],
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
        .size(px(10.))
        .flex_none()
        .into_any_element(),
    )
}

fn shortcut_label(shortcut: &SharedString, color: Hsla) -> AnyElement {
    let mut row = div()
        .flex_none()
        .flex()
        .items_center()
        .gap(px(2.))
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

struct MenuColors {
    ink: Hsla,
    muted: Hsla,
    edge: Hsla,
    accent: Hsla,
    card: Hsla,
    danger: Hsla,
}

fn menu_colors(dark: bool) -> MenuColors {
    MenuColors {
        ink: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        edge: rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(),
        accent: rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into(),
        card: rgb(if dark { 0x232323 } else { 0xffffff }).into(),
        danger: rgb(if dark { 0xf87171 } else { 0xdc2626 }).into(),
    }
}

impl ContextMenu {
    /// Render one entry row at `index` inside `items`. `active_idx` is the
    /// currently highlighted index; `is_sub` tags ids so the two levels never
    /// collide.
    fn render_entry(
        &self,
        index: usize,
        entry: &ContextMenuEntry,
        level: usize,
        active: Option<usize>,
        colors: &MenuColors,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match entry {
            ContextMenuEntry::Separator => div()
                .h(px(1.))
                .bg(colors.edge)
                .mx(px(-4.))
                .my_1()
                .into_any_element(),
            ContextMenuEntry::Label(text) => div()
                .h(LABEL_H)
                .px_2()
                .flex()
                .items_center()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(colors.muted)
                .child(text.clone())
                .into_any_element(),
            ContextMenuEntry::Item(item) => {
                let is_active = active == Some(index);
                let needs_gutter = matches!(
                    item.kind,
                    ItemKind::Checkbox { .. } | ItemKind::Radio { .. }
                );
                let text_color = if item.disabled {
                    colors.muted
                } else if item.destructive {
                    colors.danger
                } else {
                    colors.ink
                };
                let mut row = div()
                    .h(ITEM_H)
                    .px_2()
                    .mx_1()
                    .rounded_sm()
                    .flex()
                    .items_center()
                    .gap_2()
                    .text_sm()
                    .text_color(text_color);
                // Selection gutter: check / radio dot.
                if needs_gutter {
                    let mark = match &item.kind {
                        ItemKind::Checkbox { checked: true } => {
                            check_glyph(colors.ink).into_any_element()
                        }
                        ItemKind::Radio { group } => {
                            if self.radio.get(group.as_ref()).map(|v| v == &item.value)
                                == Some(true)
                            {
                                dot_glyph(colors.ink).into_any_element()
                            } else {
                                div().size(px(14.)).into_any_element()
                            }
                        }
                        _ => div().size(px(14.)).into_any_element(),
                    };
                    row = row.child(mark);
                }
                if let Some(icon) = item.icon {
                    row = row.child(icon_element(icon, text_color));
                }
                row = row.child(
                    div()
                        .flex_1()
                        .min_w_0()
                        .overflow_hidden()
                        .child(item.label.clone()),
                );
                if let Some(shortcut) = item.shortcut.as_ref() {
                    row = row.child(shortcut_label(shortcut, colors.muted));
                }
                let has_children = item.has_children();
                if has_children {
                    row = row.child(sub_chevron(if item.disabled {
                        colors.muted
                    } else {
                        colors.ink
                    }));
                }
                let is_sub_open = level == 0 && self.open_sub == Some(index);
                if item.disabled {
                    return row.opacity(0.55).into_any_element();
                }
                let row = row
                    .id(("ctx-item", self.id * 1_000 + level * 100 + index))
                    .cursor_pointer()
                    .when(is_active || is_sub_open, |r| {
                        let mut r = r.bg(colors.accent);
                        if item.destructive {
                            r = r.text_color(colors.danger);
                        }
                        r
                    })
                    .on_mouse_move(cx.listener(move |this, _, _, cx| {
                        if level == 0 {
                            let changed = this.active != Some(index)
                                || (has_children && this.open_sub != Some(index))
                                || (!has_children && this.open_sub.is_some());
                            this.active = Some(index);
                            if has_children {
                                if this.open_sub != Some(index) {
                                    this.open_sub = Some(index);
                                    this.sub_active = None;
                                }
                            } else if this.open_sub.is_some() {
                                this.open_sub = None;
                                this.sub_active = None;
                            }
                            if changed {
                                cx.notify();
                            }
                        } else if this.sub_active != Some(index) {
                            this.sub_active = Some(index);
                            cx.notify();
                        }
                    }))
                    .on_mouse_down(MouseButton::Left, {
                        cx.listener(move |this, _, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            if level == 0 {
                                this.activate(index, None, cx);
                            } else if let Some(top) = this.open_sub {
                                this.activate(top, Some(index), cx);
                            }
                        })
                    });
                row.into_any_element()
            }
        }
    }

    fn render_panel(
        &self,
        items: &[ContextMenuEntry],
        level: usize,
        active: Option<usize>,
        colors: &MenuColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut panel = div()
            .w(MENU_W)
            .rounded_md()
            .border_1()
            .border_color(colors.edge)
            .bg(colors.card)
            .shadow_md()
            .py(PAD_Y)
            .flex()
            .flex_col();
        for (index, entry) in items.iter().enumerate() {
            panel = panel.child(self.render_entry(index, entry, level, active, colors, cx));
        }
        panel.on_mouse_down(MouseButton::Left, |_, _, cx| {
            cx.stop_propagation();
        })
    }
}

impl Render for ContextMenu {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus = self.focus.clone();

        // Trigger surface.
        let trigger_content = (self.trigger)();
        let trigger = div()
            .id(("ctx-trigger", self.id))
            .child(trigger_content)
            .on_mouse_down(
                MouseButton::Right,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    window.prevent_default();
                    let bounds = *this.root_bounds.borrow();
                    this.open_at = Some(event.position - bounds.origin);
                    this.active = this
                        .items
                        .iter()
                        .position(|e| matches!(e, ContextMenuEntry::Item(i) if !i.disabled));
                    this.open_sub = None;
                    this.sub_active = None;
                    this.focus.focus(window, cx);
                    cx.notify();
                }),
            );

        let bounds_cell = self.root_bounds.clone();
        let mut root = div()
            .size_full()
            .font_family(FONT)
            // children[0] is a size_full wrapper — its bounds equal this
            // root's content box, which converts window-space pointer coords
            // into menu-local coords for the overlay.
            .on_children_prepainted(move |children_bounds, _, _| {
                if let Some(b) = children_bounds.first() {
                    *bounds_cell.borrow_mut() = *b;
                }
            })
            .child(
                div()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(trigger),
            );

        if let Some(anchor) = self.open_at {
            let items = self.items.clone();
            let menu_h = menu_height(&items);
            // Clamp inside the root bounds.
            let bounds = *self.root_bounds.borrow();
            let x = clamp_p(anchor.x, px(4.), bounds.size.width - MENU_W - px(4.));
            let y = clamp_p(anchor.y, px(4.), bounds.size.height - menu_h - px(4.));

            let mut overlay = div()
                .id(("ctx-overlay", self.id))
                .absolute()
                .inset_0()
                .track_focus(&focus)
                .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                    match event.keystroke.key.as_str() {
                        "down" => this.move_active(1, cx),
                        "up" => this.move_active(-1, cx),
                        "enter" => {
                            if let Some(top) = this.open_sub {
                                if let Some(sub) = this.sub_active {
                                    this.activate(top, Some(sub), cx);
                                    return;
                                }
                            }
                            if let Some(i) = this.active {
                                this.activate(i, None, cx);
                            }
                        }
                        "right" => {
                            if let Some(i) = this.active {
                                if let Some(ContextMenuEntry::Item(item)) = this.items.get(i) {
                                    if item.has_children() {
                                        this.activate(i, None, cx);
                                    }
                                }
                            }
                        }
                        "left" | "escape" => {
                            if this.open_sub.is_some() && event.keystroke.key.as_str() == "left" {
                                this.open_sub = None;
                                this.sub_active = None;
                                cx.notify();
                            } else {
                                this.close(cx);
                            }
                        }
                        _ => {}
                    }
                }))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _, window, cx| {
                        window.prevent_default();
                        this.close(cx);
                    }),
                )
                .on_mouse_down(
                    MouseButton::Right,
                    cx.listener(|this, event: &MouseDownEvent, window, cx| {
                        window.prevent_default();
                        // Right-click elsewhere re-anchors the menu.
                        let bounds = *this.root_bounds.borrow();
                        this.open_at = Some(event.position - bounds.origin);
                        this.active = None;
                        this.open_sub = None;
                        this.sub_active = None;
                        cx.notify();
                    }),
                );

            let colors2 = menu_colors(self.dark);
            let panel = self.render_panel(&items, 0, self.active, &colors2, cx);
            overlay = overlay.child(div().absolute().left(x).top(y).child(deferred(panel)));

            // Submenu panel at the subtrigger row.
            if let Some(sub_i) = self.open_sub {
                if let Some(ContextMenuEntry::Item(item)) = self.items.get(sub_i) {
                    if item.has_children() {
                        let sub_items = item.children.clone();
                        let sub_h = menu_height(&sub_items);
                        let sub_x = clamp_p(
                            x + MENU_W - px(4.),
                            px(4.),
                            bounds.size.width - MENU_W - px(4.),
                        );
                        let sub_y = clamp_p(
                            y + entry_offset(&items, sub_i),
                            px(4.),
                            bounds.size.height - sub_h - px(4.),
                        );
                        let colors3 = menu_colors(self.dark);
                        let sub_panel =
                            self.render_panel(&sub_items, 1, self.sub_active, &colors3, cx);
                        overlay = overlay.child(
                            div()
                                .absolute()
                                .left(sub_x)
                                .top(sub_y)
                                .child(deferred(sub_panel)),
                        );
                    }
                }
            }

            root = root.child(overlay);
        }

        root
    }
}

// ── Demo gallery ──────────────────────────────────────────────────────────

fn palette(dark: bool) -> (Hsla, Hsla, Hsla, Hsla) {
    (
        rgb(if dark { 0xededed } else { 0x171717 }).into(),
        rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(),
        rgb(if dark { 0x202020 } else { 0xf8f8f8 }).into(),
    )
}

fn basic_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Profile")),
        ContextMenuEntry::Item(ContextMenuItem::new("Billing")),
        ContextMenuEntry::Item(ContextMenuItem::new("Team")),
        ContextMenuEntry::Item(ContextMenuItem::new("Subscription")),
    ]
}

fn submenu_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Back").shortcut("⌘[").disabled(true)),
        ContextMenuEntry::Item(
            ContextMenuItem::new("Forward")
                .shortcut("⌘]")
                .disabled(true),
        ),
        ContextMenuEntry::Item(ContextMenuItem::new("Reload").shortcut("⌘R")),
        ContextMenuEntry::Item(ContextMenuItem::new("More Tools").submenu(vec![
            ContextMenuEntry::Item(ContextMenuItem::new("Save Page As…").shortcut("⌘S")),
            ContextMenuEntry::Item(ContextMenuItem::new("Create Shortcut…")),
            ContextMenuEntry::Item(ContextMenuItem::new("Name Window…")),
            ContextMenuEntry::Separator,
            ContextMenuEntry::Item(ContextMenuItem::new("Developer Tools")),
        ])),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Item(
            ContextMenuItem::new("Show Bookmarks")
                .shortcut("⌘B")
                .checkbox(true),
        ),
        ContextMenuEntry::Item(ContextMenuItem::new("Show Full URLs").checkbox(false)),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Item(ContextMenuItem::new("Inspect").shortcut("⌥⌘I")),
    ]
}

fn shortcuts_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Undo").shortcut("⌘Z")),
        ContextMenuEntry::Item(ContextMenuItem::new("Redo").shortcut("⇧⌘Z")),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Item(ContextMenuItem::new("Cut").shortcut("⌘X")),
        ContextMenuEntry::Item(ContextMenuItem::new("Copy").shortcut("⌘C")),
        ContextMenuEntry::Item(ContextMenuItem::new("Paste").shortcut("⌘V")),
    ]
}

fn groups_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Label("File".into()),
        ContextMenuEntry::Item(ContextMenuItem::new("New File")),
        ContextMenuEntry::Item(ContextMenuItem::new("Duplicate")),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Label("Edit".into()),
        ContextMenuEntry::Item(ContextMenuItem::new("Rename")),
        ContextMenuEntry::Item(ContextMenuItem::new("Move to Trash")),
    ]
}

fn icons_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Back").icon(MenuIcon::ArrowLeft)),
        ContextMenuEntry::Item(ContextMenuItem::new("Forward").icon(MenuIcon::ArrowRight)),
        ContextMenuEntry::Item(ContextMenuItem::new("Reload").icon(MenuIcon::Rotate)),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Item(ContextMenuItem::new("Copy Link").icon(MenuIcon::Share)),
        ContextMenuEntry::Item(ContextMenuItem::new("Duplicate").icon(MenuIcon::Copy)),
        ContextMenuEntry::Item(ContextMenuItem::new("Settings").icon(MenuIcon::Gear)),
    ]
}

fn selection_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Label("Panels".into()),
        ContextMenuEntry::Item(ContextMenuItem::new("Show Sidebar").checkbox(true)),
        ContextMenuEntry::Item(ContextMenuItem::new("Show Status Bar").checkbox(false)),
        ContextMenuEntry::Item(ContextMenuItem::new("Show Minimap").checkbox(true)),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Label("Theme".into()),
        ContextMenuEntry::Item(ContextMenuItem::new("System").radio("theme")),
        ContextMenuEntry::Item(ContextMenuItem::new("Light").radio("theme")),
        ContextMenuEntry::Item(ContextMenuItem::new("Dark").radio("theme")),
    ]
}

fn destructive_items() -> Vec<ContextMenuEntry> {
    vec![
        ContextMenuEntry::Item(ContextMenuItem::new("Rename").icon(MenuIcon::Pencil)),
        ContextMenuEntry::Item(ContextMenuItem::new("Share").icon(MenuIcon::Share)),
        ContextMenuEntry::Item(ContextMenuItem::new("Archive").icon(MenuIcon::Window)),
        ContextMenuEntry::Separator,
        ContextMenuEntry::Item(
            ContextMenuItem::new("Delete")
                .icon(MenuIcon::Trash)
                .shortcut("⌘⌫")
                .destructive(true),
        ),
    ]
}

struct ContextMenuDemo {
    menu: Entity<ContextMenu>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl ContextMenuDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let items = match variant {
            "submenu" => submenu_items(),
            "shortcuts" => shortcuts_items(),
            "groups" => groups_items(),
            "icons" => icons_items(),
            "selection" => selection_items(),
            "destructive" => destructive_items(),
            _ => basic_items(),
        };
        let surface_edge = palette(dark).2;
        let menu = cx.new(|cx| {
            ContextMenu::new(
                move || {
                    div()
                        .w(px(360.))
                        .h(px(160.))
                        .rounded_lg()
                        .border_1()
                        .border_dashed()
                        .border_color(surface_edge)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_sm()
                                .text_color(rgb(0x737373))
                                .child("Right click here"),
                        )
                        .into_any_element()
                },
                items,
                cx,
            )
            .dark(dark)
        });
        let sub = cx.subscribe(&menu, |demo, _, ev: &ContextMenuSelectEvent, cx| {
            demo.outcome = Some(match ev.checked {
                Some(c) => format!("{} → {}", ev.value, c).into(),
                None => format!("Chose: {}", ev.value).into(),
            });
            cx.notify();
        });
        Self {
            menu,
            dark,
            outcome: None,
            _subscriptions: vec![sub],
        }
    }
}

impl Render for ContextMenuDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, _edge, surface) = palette(self.dark);
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
            .child(self.menu.clone())
            .child(
                div()
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
        window_bounds: Some(WindowBounds::centered(size(px(720.), px(480.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |window, cx| {
        cx.new(|cx| {
            let _ = window;
            ContextMenuDemo::new(&variant, dark, cx)
        })
    })
    .expect("open context-menu gallery");
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
    cx.new(|cx| ContextMenuDemo::new(variant, dark, cx)).into()
}

pub fn run_native(variant: &str) {
    let variant = variant.to_string();
    kit::application().run(move |cx| setup(&variant, false, cx));
}

#[cfg(test)]
mod tests {
    use super::{ContextMenu, ContextMenuEntry, ContextMenuItem};
    use gpui_kit::gpui;
    use gpui_kit::gpui::{AppContext, Context, Entity, TestAppContext, VisualTestContext};
    use gpui_kit::prelude::FluentBuilder;
    use gpui_kit::{IntoElement, Styled};

    fn menu(
        cx: &mut TestAppContext,
        items: Vec<ContextMenuEntry>,
    ) -> (Entity<ContextMenu>, &mut VisualTestContext) {
        cx.update(gpui_kit::base::init);
        cx.add_window_view(move |_, cx| {
            ContextMenu::new(|| gpui::div().into_any_element(), items, cx)
        })
    }

    #[gpui::test]
    fn right_click_opens_and_enter_selects(cx: &mut TestAppContext) {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};
        let hits = Arc::new(AtomicUsize::new(0));
        let (entity, cx) = menu(
            cx,
            vec![
                ContextMenuEntry::Item(ContextMenuItem::new("Profile")),
                ContextMenuEntry::Item(ContextMenuItem::new("Billing")),
            ],
        );
        let hits2 = hits.clone();
        let _sub = cx.update(|_, cx| {
            cx.subscribe(&entity, move |_, ev: &super::ContextMenuSelectEvent, _| {
                assert_eq!(ev.value.as_ref(), "Profile");
                hits2.fetch_add(1, Ordering::SeqCst);
            })
        });
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| this.set_open(true, window, cx));
        });
        cx.simulate_keystrokes("enter");
        assert_eq!(hits.load(Ordering::SeqCst), 1);
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| assert!(!this.is_open()));
        });
    }

    #[gpui::test]
    fn arrows_skip_disabled_and_escape_closes(cx: &mut TestAppContext) {
        let (entity, cx) = menu(
            cx,
            vec![
                ContextMenuEntry::Item(ContextMenuItem::new("A").disabled(true)),
                ContextMenuEntry::Item(ContextMenuItem::new("B")),
                ContextMenuEntry::Item(ContextMenuItem::new("C")),
            ],
        );
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| this.set_open(true, window, cx));
        });
        // active lands on first enabled (index 1); down → 2; up → 1; the
        // disabled row at 0 is skipped both directions.
        cx.simulate_keystrokes("down up escape");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert!(!this.is_open());
            });
        });
    }

    #[gpui::test]
    fn checkbox_toggles_and_stays_open(cx: &mut TestAppContext) {
        let (entity, cx) = menu(
            cx,
            vec![ContextMenuEntry::Item(
                ContextMenuItem::new("Show Sidebar").checkbox(true),
            )],
        );
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| this.set_open(true, window, cx));
        });
        cx.simulate_keystrokes("enter");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert_eq!(this.checked("Show Sidebar"), Some(false));
                assert!(this.is_open());
            });
        });
    }

    #[gpui::test]
    fn right_arrow_opens_submenu(cx: &mut TestAppContext) {
        let (entity, cx) = menu(
            cx,
            vec![
                ContextMenuEntry::Item(ContextMenuItem::new("Plain")),
                ContextMenuEntry::Item(ContextMenuItem::new("More").submenu(vec![
                    ContextMenuEntry::Item(ContextMenuItem::new("Sub A")),
                    ContextMenuEntry::Item(ContextMenuItem::new("Sub B")),
                ])),
            ],
        );
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| this.set_open(true, window, cx));
        });
        cx.simulate_keystrokes("down right");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| {
                assert_eq!(this.open_sub, Some(1));
                assert_eq!(this.sub_active, Some(0));
            });
        });
        cx.simulate_keystrokes("left");
        cx.update(|_, cx| {
            entity.read_with(cx, |this, _| assert_eq!(this.open_sub, None));
        });
    }

    #[test]
    fn heights_are_consistent() {
        let items = basic_entries();
        // item(30) + separator(9) + item(30) + 2×pad(5) = 79
        assert_eq!(super::menu_height(&items), super::px(79.));
        assert_eq!(
            super::entry_offset(&items, 2) - super::entry_offset(&items, 1),
            super::SEP_H
        );
    }

    fn basic_entries() -> Vec<ContextMenuEntry> {
        vec![
            ContextMenuEntry::Item(ContextMenuItem::new("A")),
            ContextMenuEntry::Separator,
            ContextMenuEntry::Item(ContextMenuItem::new("B")),
        ]
    }
}
