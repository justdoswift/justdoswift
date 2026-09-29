//! shadcn/ui-style data table for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/aria/data-table (the
//! TanStack Table guide — sorting, filtering, pagination, row selection,
//! column visibility and row actions in one table).
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use kit::base::input::{Input, InputEditorStyle, InputEvent, InputState};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

const FONT: &str = "Geist";
const ROW_H: Pixels = px(40.);
const HEADER_H: Pixels = px(40.);
const TOOLBAR_H: Pixels = px(44.);
const FOOTER_H: Pixels = px(44.);
const MENU_W: Pixels = px(156.);
const COLS_MENU_W: Pixels = px(140.);

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// Emitted when a row-action menu item is chosen.
#[derive(Clone, Debug, PartialEq)]
pub struct DataTableActionEvent {
    /// The action item's label.
    pub action: SharedString,
    /// The row's `id`.
    pub row: SharedString,
}

/// One column definition (`ColumnDef` analogue).
#[derive(Clone, Debug)]
pub struct DataColumn {
    /// Identity; also used by the visibility menu.
    pub id: SharedString,
    pub header: SharedString,
    /// Clicking the header cycles asc → desc → none.
    pub sortable: bool,
    /// Right-aligned and sorted numerically (digits + decimal parsed out
    /// of the cell text, so "$316.00" sorts as 316).
    pub numeric: bool,
    /// Fixed width; unset columns share the remaining space.
    pub width: Option<Pixels>,
}

impl DataColumn {
    pub fn new(id: impl Into<SharedString>, header: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            sortable: false,
            numeric: false,
            width: None,
        }
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }

    pub fn width(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

/// One data row — `cells` is parallel to the column list.
#[derive(Clone, Debug)]
pub struct DataRow {
    pub id: SharedString,
    pub cells: Vec<SharedString>,
}

impl DataRow {
    pub fn new(id: impl Into<SharedString>, cells: Vec<SharedString>) -> Self {
        Self {
            id: id.into(),
            cells,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SortDir {
    Asc,
    Desc,
}

/// `DataTable` — one entity owns columns, rows, sort/filter state,
/// selection, pagination and the floating menus.
pub struct DataTable {
    columns: Vec<DataColumn>,
    rows: Vec<DataRow>,
    sort: Option<(usize, SortDir)>,
    /// Filter box (`DataTableToolbar`); `None` hides the toolbar input.
    filter: Option<Entity<InputState>>,
    /// Column index the filter applies to; `None` matches every cell.
    filter_col: Option<usize>,
    /// Selection column on the left (row checkboxes + footer count).
    selection: bool,
    /// Selected row indices (into `rows`, stable across sort/filter).
    selected: HashSet<usize>,
    /// Row-action menu items; empty hides the ⋯ column.
    actions: Vec<SharedString>,
    /// Row index with an open action menu.
    menu_row: Option<usize>,
    /// "Columns" dropdown open.
    cols_menu: bool,
    /// Show the "Columns" visibility button in the toolbar.
    show_cols_button: bool,
    /// Per-column visibility (toggled from the Columns menu).
    col_visible: Vec<bool>,
    page: usize,
    page_size: usize,
    /// Show footer with count + Previous/Next.
    pagination: bool,
    dark: bool,
    scroll: ScrollHandle,
    /// Root bounds captured each prepaint → local coords for popups.
    root_bounds: Rc<RefCell<Bounds<Pixels>>>,
    id: usize,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<DataTableActionEvent> for DataTable {}

impl DataTable {
    pub fn new(columns: Vec<DataColumn>, rows: Vec<DataRow>, _cx: &mut Context<Self>) -> Self {
        let visible = columns.iter().map(|_| true).collect();
        Self {
            columns,
            rows,
            sort: None,
            filter: None,
            filter_col: None,
            selection: false,
            selected: HashSet::new(),
            actions: Vec::new(),
            menu_row: None,
            cols_menu: false,
            show_cols_button: false,
            col_visible: visible,
            page: 0,
            page_size: 10,
            pagination: false,
            dark: false,
            scroll: ScrollHandle::new(),
            root_bounds: Rc::new(RefCell::new(Bounds {
                origin: point(px(0.), px(0.)),
                size: size(px(0.), px(0.)),
            })),
            id: NEXT_KEY.fetch_add(1, Ordering::Relaxed),
            _subscriptions: Vec::new(),
        }
    }

    /// Wire a caller-created `InputState` as the toolbar filter.
    /// `filter_col = Some(i)` restricts matching to column `i`.
    pub fn filter_input(
        mut self,
        input: Entity<InputState>,
        filter_col: Option<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        let sub = cx.subscribe(&input, |this: &mut Self, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.page = 0;
                cx.notify();
            }
        });
        self._subscriptions.push(sub);
        self.filter = Some(input);
        self.filter_col = filter_col;
        self
    }

    /// Show the selection column + "N of M selected" footer.
    pub fn selection(mut self, selection: bool) -> Self {
        self.selection = selection;
        self.pagination = true;
        self
    }

    /// Row-action menu items (e.g. ["Copy ID","View details"]) — enables
    /// the ⋯ column.
    pub fn actions(mut self, actions: &[&str]) -> Self {
        self.actions = actions.iter().map(|a| (*a).into()).collect();
        self
    }

    /// Pagination with `page_size` rows per page.
    pub fn paginate(mut self, page_size: usize) -> Self {
        self.pagination = true;
        self.page_size = page_size.max(1);
        self
    }

    /// Show the "Columns" visibility dropdown in the toolbar.
    pub fn column_visibility(mut self, enabled: bool) -> Self {
        self.show_cols_button = enabled;
        self
    }

    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// Selected row ids (stable across sort/filter/pagination).
    pub fn selected_ids(&self) -> Vec<SharedString> {
        let mut ids: Vec<_> = self
            .selected
            .iter()
            .filter_map(|&i| self.rows.get(i).map(|r| r.id.clone()))
            .collect();
        ids.sort();
        ids
    }

    /// Filter → sort → paginate pipeline. First two stages return indices
    /// into `rows`; pagination slices them.
    fn filtered_indices(&self, cx: &mut Context<Self>) -> Vec<usize> {
        let query = self
            .filter
            .as_ref()
            .map(|input| input.read(cx).value().to_string().to_lowercase())
            .unwrap_or_default()
            .trim()
            .to_string();
        let mut indices: Vec<usize> = self
            .rows
            .iter()
            .enumerate()
            .filter_map(|(i, row)| {
                if query.is_empty() {
                    return Some(i);
                }
                let hit = match self.filter_col {
                    Some(col) => row
                        .cells
                        .get(col)
                        .is_some_and(|c| c.to_lowercase().contains(&query)),
                    None => row.cells.iter().any(|c| c.to_lowercase().contains(&query)),
                };
                hit.then_some(i)
            })
            .collect();
        if let Some((col, dir)) = self.sort {
            let numeric = self.columns.get(col).is_some_and(|c| c.numeric);
            indices.sort_by(|&a, &b| {
                let ca = self.rows[a].cells.get(col).map(AsRef::as_ref).unwrap_or("");
                let cb = self.rows[b].cells.get(col).map(AsRef::as_ref).unwrap_or("");
                let ord = if numeric {
                    cell_num(ca)
                        .partial_cmp(&cell_num(cb))
                        .unwrap_or(std::cmp::Ordering::Equal)
                } else {
                    ca.to_lowercase().cmp(&cb.to_lowercase())
                };
                match dir {
                    SortDir::Asc => ord,
                    SortDir::Desc => ord.reverse(),
                }
            });
        }
        indices
    }

    fn page_of<'a>(&self, indices: &'a [usize]) -> &'a [usize] {
        if !self.pagination {
            return indices;
        }
        let start = self.page * self.page_size;
        indices
            .get(start..(start + self.page_size).min(indices.len()))
            .unwrap_or(&[])
    }

    fn page_count(&self, filtered: usize) -> usize {
        (filtered.max(1) + self.page_size - 1) / self.page_size
    }

    fn toggle_sort(&mut self, col: usize, cx: &mut Context<Self>) {
        self.sort = match self.sort {
            Some((c, SortDir::Asc)) if c == col => Some((col, SortDir::Desc)),
            Some((c, SortDir::Desc)) if c == col => None,
            _ => Some((col, SortDir::Asc)),
        };
        self.page = 0;
        cx.notify();
    }

    /// Header checkbox state over the filtered set: 0 → Unchecked,
    /// all → Checked, partial → Indeterminate.
    fn header_state(&self, filtered: &[usize]) -> MarkState {
        let sel = filtered
            .iter()
            .filter(|i| self.selected.contains(*i))
            .count();
        if sel == 0 {
            MarkState::Unchecked
        } else if sel == filtered.len() {
            MarkState::Checked
        } else {
            MarkState::Indeterminate
        }
    }

    fn toggle_header(&mut self, filtered: &[usize], cx: &mut Context<Self>) {
        match self.header_state(filtered) {
            MarkState::Checked => {
                for &i in filtered {
                    self.selected.remove(&i);
                }
            }
            _ => {
                for &i in filtered {
                    self.selected.insert(i);
                }
            }
        }
        cx.notify();
    }
}

/// Numeric sort key — strips everything except digits, `.` and `-`.
fn cell_num(text: &str) -> f64 {
    text.chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect::<String>()
        .parse()
        .unwrap_or(0.)
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum MarkState {
    Unchecked,
    Checked,
    Indeterminate,
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

fn sort_glyph(dir: Option<SortDir>, color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 12.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.3 * k.max(0.7)));
            match dir {
                Some(SortDir::Asc) => {
                    b.move_to(at(6., 9.5));
                    b.line_to(at(6., 2.5));
                    b.move_to(at(3., 5.5));
                    b.line_to(at(6., 2.5));
                    b.line_to(at(9., 5.5));
                }
                Some(SortDir::Desc) => {
                    b.move_to(at(6., 2.5));
                    b.line_to(at(6., 9.5));
                    b.move_to(at(3., 6.5));
                    b.line_to(at(6., 9.5));
                    b.line_to(at(9., 6.5));
                }
                None => {
                    b.move_to(at(6., 2.));
                    b.line_to(at(6., 5.5));
                    b.move_to(at(3.5, 2.4));
                    b.line_to(at(6., 2.));
                    b.line_to(at(8.5, 2.4));
                    b.move_to(at(6., 10.));
                    b.line_to(at(6., 6.5));
                    b.move_to(at(3.5, 9.6));
                    b.line_to(at(6., 10.));
                    b.line_to(at(8.5, 9.6));
                }
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(12.))
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
    .size(px(12.))
    .flex_none()
    .into_any_element()
}

/// ⋯ ellipsis for the row-action trigger — filled dots so they stay
/// legible at small sizes.
fn dots_glyph(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let c = bounds.center();
            let s: f32 = bounds.size.width.into();
            let mut b = PathBuilder::stroke(px(2.6));
            for dx in [-s * 0.3, 0., s * 0.3] {
                let p = c + point(px(dx), px(0.));
                b.move_to(p + point(px(-0.7), px(0.)));
                b.line_to(p + point(px(0.7), px(0.)));
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(16.))
    .flex_none()
    .into_any_element()
}

fn mark_glyph(state: MarkState, color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.6 * k.max(0.7)));
            match state {
                MarkState::Checked => {
                    b.move_to(at(3.4, 8.4));
                    b.line_to(at(6.8, 11.8));
                    b.line_to(at(12.8, 4.6));
                }
                MarkState::Indeterminate => {
                    b.move_to(at(4., 8.));
                    b.line_to(at(12., 8.));
                }
                MarkState::Unchecked => return,
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

fn search_glyph(color: Hsla) -> AnyElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 16.;
            let at = |x: f32, y: f32| o + point(px(x * k), px(y * k));
            let mut b = PathBuilder::stroke(px(1.2 * k.max(0.7)));
            for i in 0..12 {
                let a0 = i as f32 / 12. * std::f32::consts::TAU;
                let a1 = (i + 1) as f32 / 12. * std::f32::consts::TAU;
                b.move_to(at(6.5 + a0.cos() * 4.2, 6.5 + a0.sin() * 4.2));
                b.line_to(at(6.5 + a1.cos() * 4.2, 6.5 + a1.sin() * 4.2));
            }
            b.move_to(at(10.5, 10.5));
            b.line_to(at(13.6, 13.6));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
    .size(px(14.))
    .flex_none()
    .into_any_element()
}

// ── Render ────────────────────────────────────────────────────────────────

struct TableColors {
    ink: Hsla,
    muted: Hsla,
    edge: Hsla,
    card: Hsla,
    accent: Hsla,
    head: Hsla,
}

fn table_colors(dark: bool) -> TableColors {
    TableColors {
        ink: rgb(if dark { 0xededed } else { 0x171717 }).into(),
        muted: rgb(if dark { 0xa3a3a3 } else { 0x737373 }).into(),
        edge: rgb(if dark { 0x3b3b3b } else { 0xe4e4e4 }).into(),
        card: rgb(if dark { 0x232323 } else { 0xffffff }).into(),
        accent: rgb(if dark { 0x2c2c2c } else { 0xf0f0f0 }).into(),
        head: rgb(if dark { 0x272727 } else { 0xfafafa }).into(),
    }
}

fn checkbox_cell(state: MarkState, colors: &TableColors) -> AnyElement {
    let marked = state != MarkState::Unchecked;
    let mut box_el = div()
        .flex_none()
        .size(px(15.))
        .rounded(px(4.))
        .border_1()
        .border_color(if marked { colors.ink } else { colors.muted });
    if marked {
        box_el = box_el
            .bg(colors.ink)
            .border_color(colors.ink)
            .child(mark_glyph(state, colors.card));
    }
    box_el.into_any_element()
}

impl Render for DataTable {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = table_colors(self.dark);
        let filtered = self.filtered_indices(cx);
        let page_rows: Vec<usize> = self.page_of(&filtered).to_vec();
        let pages = self.page_count(filtered.len());
        let show_toolbar = self.filter.is_some() || self.show_cols_button;
        let has_actions = !self.actions.is_empty();
        let visible: Vec<usize> = self
            .columns
            .iter()
            .enumerate()
            .filter_map(|(i, _)| self.col_visible[i].then_some(i))
            .collect();

        // ── Toolbar ────────────────────────────────────────────────────
        let mut toolbar = div().h(TOOLBAR_H).flex().items_center().gap_2();
        if let Some(input) = &self.filter {
            toolbar = toolbar.child(
                div()
                    .w(px(240.))
                    .h(px(32.))
                    .px_2()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.edge)
                    .bg(colors.card)
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(search_glyph(colors.muted))
                    .child(div().flex_1().min_w_0().child(Input::new(input))),
            );
        } else {
            toolbar = toolbar.child(div().flex_1());
        }
        toolbar = toolbar.child(div().flex_1());
        if self.show_cols_button {
            toolbar = toolbar.child(
                div()
                    .id(("dt-cols", self.id))
                    .h(px(32.))
                    .px_3()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.edge)
                    .bg(colors.card)
                    .flex()
                    .items_center()
                    .gap_2()
                    .cursor_pointer()
                    .text_sm()
                    .text_color(colors.ink)
                    .child("Columns")
                    .child(chevron_down(colors.muted))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, window, cx| {
                            window.prevent_default();
                            cx.stop_propagation();
                            this.cols_menu = !this.cols_menu;
                            this.menu_row = None;
                            cx.notify();
                        }),
                    ),
            );
        }

        // ── Header row ──────────────────────────────────────────────────
        let mut header = div()
            .h(HEADER_H)
            .flex()
            .items_center()
            .border_b_1()
            .border_color(colors.edge)
            .bg(colors.head);
        if self.selection {
            let filtered_for_header = filtered.clone();
            header = header.child(
                div()
                    .w(px(44.))
                    .flex()
                    .justify_center()
                    .id(("dt-head-sel", self.id))
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            window.prevent_default();
                            this.toggle_header(&filtered_for_header, cx);
                        }),
                    )
                    .child(checkbox_cell(self.header_state(&filtered), &colors)),
            );
        }
        for &ci in &visible {
            let col = &self.columns[ci];
            let mut cell = div()
                .h_full()
                .px_3()
                .flex()
                .items_center()
                .gap_1p5()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(colors.muted);
            cell = match col.width {
                Some(w) => cell.w(w).flex_none(),
                None => cell.flex_1().min_w_0(),
            };
            let dir = self.sort.and_then(|(c, d)| (c == ci).then_some(d));
            let mut label = div()
                .id(("dt-sort", ci))
                .flex()
                .items_center()
                .gap_1p5()
                .child(col.header.clone())
                .when(col.sortable, |l| {
                    l.child(sort_glyph(
                        dir,
                        if dir.is_some() {
                            colors.ink
                        } else {
                            colors.muted
                        },
                    ))
                });
            if col.sortable {
                label = label.cursor_pointer().on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |this, _, window, cx| {
                        window.prevent_default();
                        this.toggle_sort(ci, cx);
                    }),
                );
            }
            cell = cell.child(label);
            header = header.child(cell);
        }
        if has_actions {
            header = header.child(div().w(px(40.)).flex_none());
        }

        // ── Body ────────────────────────────────────────────────────────
        let mut body = div().flex().flex_col();
        if page_rows.is_empty() {
            body = body.child(
                div()
                    .h(px(96.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .text_color(colors.muted)
                    .child("No results."),
            );
        } else {
            for &row_i in &page_rows {
                let row = &self.rows[row_i];
                let is_sel = self.selected.contains(&row_i);
                let mut tr = div()
                    .id(("dt-row", row_i))
                    .h(ROW_H)
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(colors.edge)
                    .text_sm()
                    .text_color(colors.ink)
                    .hover(|s| s.bg(colors.accent));
                tr = tr.when(is_sel, |t| t.bg(colors.accent));
                if self.selection {
                    tr = tr.child(
                        div()
                            .w(px(44.))
                            .flex()
                            .justify_center()
                            .id(("dt-sel", row_i))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    window.prevent_default();
                                    if this.selected.contains(&row_i) {
                                        this.selected.remove(&row_i);
                                    } else {
                                        this.selected.insert(row_i);
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(checkbox_cell(
                                if is_sel {
                                    MarkState::Checked
                                } else {
                                    MarkState::Unchecked
                                },
                                &colors,
                            )),
                    );
                }
                for &ci in &visible {
                    let col = &self.columns[ci];
                    let mut cell = div()
                        .h_full()
                        .px_3()
                        .flex()
                        .items_center()
                        .min_w_0()
                        .overflow_hidden();
                    cell = match col.width {
                        Some(w) => cell.w(w).flex_none(),
                        None => cell.flex_1(),
                    };
                    cell = cell.when(col.numeric, |c| c.justify_end()).child(
                        div()
                            .overflow_hidden()
                            .child(row.cells.get(ci).cloned().unwrap_or_else(|| "".into())),
                    );
                    tr = tr.child(cell);
                }
                if has_actions {
                    tr = tr.child(
                        div()
                            .w(px(40.))
                            .flex_none()
                            .flex()
                            .justify_center()
                            .id(("dt-act", row_i))
                            .cursor_pointer()
                            .rounded_sm()
                            .hover(|s| s.bg(colors.accent))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                    this.menu_row = if this.menu_row == Some(row_i) {
                                        None
                                    } else {
                                        Some(row_i)
                                    };
                                    this.cols_menu = false;
                                    cx.notify();
                                }),
                            )
                            .child(dots_glyph(colors.muted)),
                    );
                }
                body = body.child(tr);
            }
        }

        let table = div()
            .w_full()
            .rounded_lg()
            .border_1()
            .border_color(colors.edge)
            .bg(colors.card)
            .overflow_hidden()
            .child(header)
            .child(
                div()
                    .id(("dt-body", 0usize))
                    .max_h(px(400.))
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .child(body),
            );

        // ── Footer ──────────────────────────────────────────────────────
        let mut footer = div().h(FOOTER_H).flex().items_center();
        if self.selection {
            footer = footer.child(div().text_xs().text_color(colors.muted).child(format!(
                "{} of {} row(s) selected.",
                self.selected.len(),
                self.rows.len()
            )));
        } else {
            footer = footer.child(div().text_xs().text_color(colors.muted).child(format!(
                "Page {} of {}",
                self.page + 1,
                pages
            )));
        }
        footer = footer.child(div().flex_1());
        if self.pagination {
            let btn = |label: &'static str, key: usize, enabled: bool, delta: isize| {
                let mut b = div()
                    .id(("dt-page", key))
                    .h(px(30.))
                    .px_3()
                    .rounded_md()
                    .border_1()
                    .border_color(colors.edge)
                    .bg(colors.card)
                    .flex()
                    .items_center()
                    .text_sm()
                    .text_color(if enabled { colors.ink } else { colors.muted });
                if enabled {
                    b = b.cursor_pointer().on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            window.prevent_default();
                            let next = this.page as isize + delta;
                            this.page = next.clamp(0, pages as isize - 1) as usize;
                            cx.notify();
                        }),
                    );
                } else {
                    b = b.opacity(0.5);
                }
                b.child(label)
            };
            footer = footer
                .child(btn("Previous", 0, self.page > 0, -1))
                .child(div().w(px(8.)))
                .child(btn("Next", 1, self.page + 1 < pages, 1));
        }

        // ── Root + popups ───────────────────────────────────────────────
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
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .when(show_toolbar, |r| r.child(toolbar))
                    .child(table)
                    .when(self.pagination, |r| r.child(footer)),
            );

        // Outside-click catcher + anchored popups.
        if self.menu_row.is_some() || self.cols_menu {
            let bounds = *self.root_bounds.borrow();
            let mut overlay = div().absolute().inset_0().on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, _, cx| {
                    this.menu_row = None;
                    this.cols_menu = false;
                    cx.notify();
                }),
            );
            if self.cols_menu {
                let mut menu = div()
                    .w(COLS_MENU_W)
                    .rounded_md()
                    .border_1()
                    .border_color(colors.edge)
                    .bg(colors.card)
                    .shadow_md()
                    .py_1()
                    .flex()
                    .flex_col();
                for (ci, col) in self.columns.iter().enumerate() {
                    let on = self.col_visible[ci];
                    menu = menu.child(
                        div()
                            .id(("dt-col", ci))
                            .h(px(30.))
                            .px_2()
                            .mx_1()
                            .rounded_sm()
                            .flex()
                            .items_center()
                            .gap_2()
                            .cursor_pointer()
                            .text_sm()
                            .text_color(colors.ink)
                            .hover(move |s| s.bg(colors.accent))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _, window, cx| {
                                    window.prevent_default();
                                    cx.stop_propagation();
                                    this.col_visible[ci] = !this.col_visible[ci];
                                    cx.notify();
                                }),
                            )
                            .child(checkbox_cell(
                                if on {
                                    MarkState::Checked
                                } else {
                                    MarkState::Unchecked
                                },
                                &colors,
                            ))
                            .child(col.header.clone()),
                    );
                }
                let menu_x = clamp_p(
                    bounds.size.width - COLS_MENU_W - px(4.),
                    px(4.),
                    bounds.size.width,
                );
                let menu_y = if show_toolbar { TOOLBAR_H } else { px(4.) } + px(4.);
                overlay = overlay.child(
                    div()
                        .absolute()
                        .left(menu_x)
                        .top(menu_y)
                        .child(deferred(menu)),
                );
            }
            if let Some(row_i) = self.menu_row {
                if let Some(page_pos) = page_rows.iter().position(|&i| i == row_i) {
                    let scroll_y = self.scroll.offset().y;
                    let anchor_y = (if show_toolbar { TOOLBAR_H } else { px(0.) })
                        + HEADER_H
                        + px(1.)
                        + page_pos as f32 * ROW_H
                        - scroll_y
                        + ROW_H;
                    let mut menu = div()
                        .w(MENU_W)
                        .rounded_md()
                        .border_1()
                        .border_color(colors.edge)
                        .bg(colors.card)
                        .shadow_md()
                        .py_1()
                        .flex()
                        .flex_col();
                    for (ai, action) in self.actions.iter().enumerate() {
                        let action = action.clone();
                        let label_text = action.clone();
                        let row_id = self.rows[row_i].id.clone();
                        menu = menu.child(
                            div()
                                .id(("dt-act-item", ai))
                                .h(px(30.))
                                .px_2()
                                .mx_1()
                                .rounded_sm()
                                .flex()
                                .items_center()
                                .cursor_pointer()
                                .text_sm()
                                .text_color(colors.ink)
                                .hover(move |s| s.bg(colors.accent))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _, window, cx| {
                                        window.prevent_default();
                                        cx.stop_propagation();
                                        this.menu_row = None;
                                        cx.emit(DataTableActionEvent {
                                            action: action.clone(),
                                            row: row_id.clone(),
                                        });
                                        cx.notify();
                                    }),
                                )
                                .child(label_text),
                        );
                    }
                    let menu_h = px(30.) * self.actions.len() as f32 + px(10.);
                    let menu_x = clamp_p(
                        bounds.size.width - MENU_W - px(10.),
                        px(4.),
                        bounds.size.width,
                    );
                    let menu_y = clamp_p(anchor_y, px(4.), bounds.size.height - menu_h - px(4.));
                    overlay = overlay.child(
                        div()
                            .absolute()
                            .left(menu_x)
                            .top(menu_y)
                            .child(deferred(menu)),
                    );
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

fn payments() -> Vec<DataRow> {
    [
        ("728ed52f", "success", "ken99@example.com", "$316.00"),
        ("489e1d42", "success", "Abe45@example.com", "$242.00"),
        (
            "aa1f2b3c",
            "processing",
            "Monserrat44@example.com",
            "$837.00",
        ),
        ("c3d4e5f6", "success", "Silas22@example.com", "$874.00"),
        ("f6e5d4c3", "failed", "carmella@example.com", "$721.00"),
        ("1a2b3c4d", "pending", "jason78@example.com", "$150.00"),
        ("5e6f7a8b", "success", "Mae23@example.com", "$420.00"),
        ("9c8b7a6e", "processing", "Reed11@example.com", "$92.50"),
        ("d4e5f6a7", "failed", "emma34@example.com", "$510.25"),
        ("b2c3d4e5", "success", "luca56@example.com", "$1,240.00"),
        ("7f8e9d0c", "pending", "nina12@example.com", "$64.00"),
        ("3c4b5a69", "processing", "otto90@example.com", "$233.75"),
    ]
    .into_iter()
    .map(|(id, status, email, amount)| {
        DataRow::new(id, vec![status.into(), email.into(), amount.into()])
    })
    .collect()
}

fn payment_columns() -> Vec<DataColumn> {
    vec![
        DataColumn::new("status", "Status").width(px(110.)),
        DataColumn::new("email", "Email"),
        DataColumn::new("amount", "Amount")
            .numeric(true)
            .sortable(true)
            .width(px(110.)),
    ]
}

struct DataTableDemo {
    table: Entity<DataTable>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

/// The toolbar filter input — an `InputState` entity so callers keep a
/// handle if they need it.
fn new_filter(
    dark: bool,
    window: &mut Window,
    cx: &mut Context<DataTableDemo>,
) -> Entity<InputState> {
    cx.new(|cx| {
        let mut state = InputState::new(window, cx).placeholder("Filter emails…".to_string());
        state.set_editor_style(editor_style(dark));
        state
    })
}

impl DataTableDemo {
    fn new(variant: &str, dark: bool, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let table: Entity<DataTable> = match variant {
            "sorting" => cx.new(|cx| {
                let mut cols = payment_columns();
                cols[0] = cols[0].clone().sortable(true);
                cols[1] = cols[1].clone().sortable(true);
                DataTable::new(cols, payments(), cx).dark(dark)
            }),
            "filtering" => {
                let input = new_filter(dark, window, cx);
                cx.new(|cx| {
                    let mut cols = payment_columns();
                    cols[1] = cols[1].clone().sortable(true);
                    DataTable::new(cols, payments(), cx)
                        .filter_input(input, Some(1), cx)
                        .dark(dark)
                })
            }
            "pagination" => cx.new(|cx| {
                DataTable::new(payment_columns(), payments(), cx)
                    .paginate(5)
                    .dark(dark)
            }),
            "selection" => cx.new(|cx| {
                DataTable::new(payment_columns(), payments(), cx)
                    .selection(true)
                    .paginate(5)
                    .dark(dark)
            }),
            "actions" => cx.new(|cx| {
                DataTable::new(payment_columns(), payments(), cx)
                    .actions(&["Copy payment ID", "View customer", "View payment details"])
                    .paginate(6)
                    .dark(dark)
            }),
            // Kitchen sink — the full shadcn example.
            "full" => {
                let input = new_filter(dark, window, cx);
                cx.new(|cx| {
                    let mut cols = payment_columns();
                    cols[1] = cols[1].clone().sortable(true);
                    DataTable::new(cols, payments(), cx)
                        .filter_input(input, Some(1), cx)
                        .selection(true)
                        .actions(&["Copy payment ID", "View customer", "View payment details"])
                        .column_visibility(true)
                        .paginate(5)
                        .dark(dark)
                })
            }
            _ => cx.new(|cx| DataTable::new(payment_columns(), payments(), cx).dark(dark)),
        };
        let sub = cx.subscribe(&table, |demo, _, ev: &DataTableActionEvent, cx| {
            demo.outcome = Some(format!("{} → row {}", ev.action, ev.row).into());
            cx.notify();
        });
        Self {
            table,
            dark,
            outcome: None,
            _subscriptions: vec![sub],
        }
    }
}

impl Render for DataTableDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let (_ink, muted, _card, surface) = palette(self.dark);
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
            .child(div().w(px(620.)).child(self.table.clone()))
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
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(520.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, move |window, cx| {
        cx.new(|cx| DataTableDemo::new(&variant, dark, window, cx))
    })
    .expect("open data-table gallery");
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
    use super::{DataTable, DataTableActionEvent, MarkState, payment_columns, payments};
    use gpui_kit::base::input::InputState;
    use gpui_kit::gpui;
    use gpui_kit::gpui::{
        AppContext, Context, Entity, Subscription, TestAppContext, VisualTestContext, Window,
    };

    fn table(
        cx: &mut TestAppContext,
        build: impl FnOnce(&mut Window, &mut Context<DataTable>) -> DataTable,
    ) -> (Entity<DataTable>, &mut VisualTestContext) {
        cx.update(gpui_kit::base::init);
        cx.add_window_view(move |window, cx| build(window, cx))
    }

    #[gpui::test]
    fn sort_cycles_asc_desc_none(cx: &mut TestAppContext) {
        let (entity, cx) = table(cx, |_, cx| {
            let mut cols = payment_columns();
            cols[1] = cols[1].clone().sortable(true);
            DataTable::new(cols, payments(), cx)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.toggle_sort(1, cx);
                let order: Vec<_> = this
                    .filtered_indices(cx)
                    .iter()
                    .map(|&i| this.rows[i].cells[1].clone())
                    .collect();
                assert!(
                    order
                        .windows(2)
                        .all(|w| w[0].to_lowercase() <= w[1].to_lowercase())
                );
                this.toggle_sort(1, cx);
                let order: Vec<_> = this
                    .filtered_indices(cx)
                    .iter()
                    .map(|&i| this.rows[i].cells[1].clone())
                    .collect();
                assert!(
                    order
                        .windows(2)
                        .all(|w| w[0].to_lowercase() >= w[1].to_lowercase())
                );
                this.toggle_sort(1, cx);
                assert!(this.sort.is_none());
            });
        });
    }

    #[gpui::test]
    fn numeric_sort_orders_amounts(cx: &mut TestAppContext) {
        let (entity, cx) = table(cx, |_, cx| {
            DataTable::new(payment_columns(), payments(), cx)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.toggle_sort(2, cx);
                let amounts: Vec<f64> = this
                    .filtered_indices(cx)
                    .iter()
                    .map(|&i| super::cell_num(this.rows[i].cells[2].as_ref()))
                    .collect();
                assert!(amounts.windows(2).all(|w| w[0] <= w[1]));
                assert_eq!(amounts.first(), Some(&64.0));
            });
        });
    }

    #[gpui::test]
    fn pagination_slices_pages(cx: &mut TestAppContext) {
        let (entity, cx) = table(cx, |_, cx| {
            DataTable::new(payment_columns(), payments(), cx).paginate(5)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                let all = this.filtered_indices(cx);
                assert_eq!(all.len(), 12);
                assert_eq!(this.page_of(&all).len(), 5);
                assert_eq!(this.page_count(all.len()), 3);
                this.page = 2;
                assert_eq!(this.page_of(&all).len(), 2);
            });
        });
    }

    #[gpui::test]
    fn typing_in_filter_shrinks_rows(cx: &mut TestAppContext) {
        let (entity, cx) = table(cx, |window, cx| {
            let input = cx.new(|cx| InputState::new(window, cx));
            DataTable::new(payment_columns(), payments(), cx).filter_input(input, Some(1), cx)
        });
        cx.update(|window, cx| {
            entity.update(cx, |this, cx| {
                let input = this.filter.clone().unwrap();
                input.update(cx, |i, cx| i.focus(window, cx));
            });
        });
        cx.simulate_keystrokes("c a r");
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                let hits = this.filtered_indices(cx);
                // Only carmella@example.com contains "car" in column 1.
                assert_eq!(hits.len(), 1);
                assert_eq!(this.rows[hits[0]].cells[1].as_ref(), "carmella@example.com");
            });
        });
    }

    #[gpui::test]
    fn header_checkbox_selects_filtered_rows(cx: &mut TestAppContext) {
        let (entity, cx) = table(cx, |_, cx| {
            DataTable::new(payment_columns(), payments(), cx).selection(true)
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                let filtered = this.filtered_indices(cx);
                this.toggle_header(&filtered, cx);
                assert_eq!(this.selected.len(), 12);
                assert_eq!(this.header_state(&filtered), MarkState::Checked);
                this.toggle_header(&filtered, cx);
                assert_eq!(this.selected.len(), 0);
            });
        });
    }

    #[gpui::test]
    fn action_menu_emits_row_id(cx: &mut TestAppContext) {
        use std::sync::Arc;
        use std::sync::Mutex;
        let last = Arc::new(Mutex::new(String::new()));
        let (entity, cx) = table(cx, |_, cx| {
            DataTable::new(payment_columns(), payments(), cx).actions(&["Copy payment ID"])
        });
        let last2 = last.clone();
        let _sub: Subscription = cx.update(|_, cx| {
            cx.subscribe(&entity, move |_, ev: &DataTableActionEvent, _| {
                *last2.lock().unwrap() = format!("{}:{}", ev.action, ev.row);
            })
        });
        cx.update(|_, cx| {
            entity.update(cx, |this, cx| {
                this.menu_row = Some(3);
                // The same emit the menu row's click handler performs.
                cx.emit(DataTableActionEvent {
                    action: "Copy payment ID".into(),
                    row: this.rows[3].id.clone(),
                });
            });
        });
        assert_eq!(last.lock().unwrap().as_str(), "Copy payment ID:c3d4e5f6");
    }
}
