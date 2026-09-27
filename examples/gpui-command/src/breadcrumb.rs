//! shadcn/ui-style breadcrumb for GPUI. Desktop and WASM share this module.
//! Reference: https://ui.shadcn.com/docs/components/base/breadcrumb
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

/// Emitted when a breadcrumb link (or the ellipsis) is pressed.
#[derive(Clone, Debug, PartialEq)]
pub struct BreadcrumbNavigateEvent {
    pub label: SharedString,
}

/// One entry in a [`Breadcrumb`].
#[derive(Clone, Debug, PartialEq)]
pub enum CrumbItem {
    /// Clickable ancestor link.
    Link(SharedString),
    /// The current page — rendered as text, not a link.
    Page(SharedString),
    /// Collapsed "…" marker for elided middle items; pressable.
    Ellipsis,
}

impl CrumbItem {
    pub fn link(label: impl Into<SharedString>) -> Self {
        Self::Link(label.into())
    }
    pub fn page(label: impl Into<SharedString>) -> Self {
        Self::Page(label.into())
    }
}

/// Glyph drawn between crumbs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrumbSeparator {
    /// Chevron-right (mirrored in RTL).
    Chevron,
    Slash,
    Dot,
}

/// A breadcrumb trail entity. Links and the ellipsis are focusable and
/// emit [`BreadcrumbNavigateEvent`]; the trailing [`CrumbItem::Page`] is
/// rendered as plain text.
pub struct Breadcrumb {
    items: Vec<CrumbItem>,
    separator: CrumbSeparator,
    rtl: bool,
    dark: bool,
}
impl EventEmitter<BreadcrumbNavigateEvent> for Breadcrumb {}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            items: vec![
                CrumbItem::link("Home"),
                CrumbItem::link("Components"),
                CrumbItem::page("Breadcrumb"),
            ],
            separator: CrumbSeparator::Chevron,
            rtl: false,
            dark: false,
        }
    }
    pub fn items(mut self, items: impl Into<Vec<CrumbItem>>) -> Self {
        self.items = items.into();
        self
    }
    pub fn separator(mut self, separator: CrumbSeparator) -> Self {
        self.separator = separator;
        self
    }
    /// Mirror the trail for right-to-left locales: item order reverses
    /// and chevrons point left.
    pub fn rtl(mut self, rtl: bool) -> Self {
        self.rtl = rtl;
        self
    }
    pub fn dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }
}

fn chevron(color: Hsla, mirrored: bool) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 12.;
            let pxk = |v: f32| px(v * k);
            let at = |x: f32, y: f32| o + point(pxk(x), pxk(y));
            let mut b = PathBuilder::stroke(px(1.5 * k.max(0.8)));
            if mirrored {
                b.move_to(at(7.5, 2.5));
                b.line_to(at(4., 6.));
                b.line_to(at(7.5, 9.5));
            } else {
                b.move_to(at(4.5, 2.5));
                b.line_to(at(8., 6.));
                b.line_to(at(4.5, 9.5));
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

fn slash(color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let o = bounds.origin;
            let s: f32 = bounds.size.width.into();
            let k = s / 12.;
            let pxk = |v: f32| px(v * k);
            let at = |x: f32, y: f32| o + point(pxk(x), pxk(y));
            let mut b = PathBuilder::stroke(px(1.3 * k.max(0.8)));
            b.move_to(at(8., 2.));
            b.line_to(at(4., 10.));
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

fn dot(color: Hsla) -> impl IntoElement {
    canvas(
        move |_, _, _| (),
        move |bounds, _, window, _| {
            let s: f32 = bounds.size.width.into();
            let r = px(s * 0.14);
            let c = bounds.origin + point(px(s * 0.5), px(s * 0.5));
            let mut b = PathBuilder::fill();
            b.move_to(c + point(r, px(0.)));
            for i in 1..=12 {
                let rad = i as f32 / 12. * std::f32::consts::TAU;
                b.line_to(c + point(r * rad.cos(), r * rad.sin()));
            }
            if let Ok(path) = b.build() {
                window.paint_path(path, color);
            }
        },
    )
}

impl Render for Breadcrumb {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ink = rgb(if self.dark { 0xededed } else { 0x171717 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
        let edge = rgb(if self.dark { 0x9a9a9a } else { 0xa3a3a3 });

        let mut items: Vec<CrumbItem> = self.items.clone();
        if self.rtl {
            items.reverse();
        }
        let entity = cx.entity().downgrade();
        let mut list = div().flex().items_center().gap_1p5();

        for (i, item) in items.into_iter().enumerate() {
            if i > 0 {
                let sep = div().flex_none().size(px(12.)).child(match self.separator {
                    CrumbSeparator::Chevron => chevron(edge.into(), self.rtl).into_any_element(),
                    CrumbSeparator::Slash => slash(edge.into()).into_any_element(),
                    CrumbSeparator::Dot => dot(edge.into()).into_any_element(),
                });
                list = list.child(sep);
            }

            let (label, is_page) = match &item {
                CrumbItem::Link(label) => (label.clone(), false),
                CrumbItem::Page(label) => (label.clone(), true),
                CrumbItem::Ellipsis => (SharedString::from("More"), false),
            };

            if is_page {
                list = list.child(
                    div()
                        .flex_none()
                        .font_family(FONT)
                        .text_sm()
                        .font_weight(FontWeight::NORMAL)
                        .text_color(ink)
                        .child(label),
                );
                continue;
            }

            let is_ellipsis = item == CrumbItem::Ellipsis;
            let display = if is_ellipsis {
                SharedString::from("…")
            } else {
                label.clone()
            };
            let key = format!("crumb-{}-{}", i, label);
            let focus = window
                .use_keyed_state(key.clone(), cx, |_, cx| cx.focus_handle())
                .read(cx)
                .clone();
            let entity = entity.clone();
            let emit_label = if is_ellipsis {
                SharedString::from("More items")
            } else {
                label.clone()
            };

            let mut crumb = div()
                .id(ElementId::Name(key.into()))
                .track_focus(&focus.tab_index(0))
                .flex_none()
                .flex()
                .items_center()
                .justify_center()
                .rounded(px(4.))
                .font_family(FONT)
                .text_sm()
                .cursor_pointer()
                .focus_visible(|style| style.border_1().border_color(edge))
                .on_click(move |_, _, cx| {
                    if let Some(breadcrumb) = entity.upgrade() {
                        let label = emit_label.clone();
                        let _ = breadcrumb.update(cx, |_, cx| {
                            cx.emit(BreadcrumbNavigateEvent { label });
                            cx.notify();
                        });
                    }
                });

            crumb = if is_ellipsis {
                crumb
                    .size(px(16.))
                    .text_color(ink)
                    .pb(px(6.))
                    .child(display)
            } else {
                crumb
                    .px_1()
                    .text_color(muted)
                    .hover(|style| style.text_color(ink))
                    .child(display)
            };
            list = list.child(crumb);
        }
        list
    }
}

#[derive(Clone, Copy)]
enum DemoKind {
    Basic,
    Separator,
    Collapsed,
    Link,
    Rtl,
}

pub struct BreadcrumbDemo {
    breadcrumb: Entity<Breadcrumb>,
    dark: bool,
    outcome: Option<SharedString>,
    _subscriptions: Vec<Subscription>,
}

impl BreadcrumbDemo {
    fn new(variant: &str, dark: bool, cx: &mut Context<Self>) -> Self {
        let kind = match variant {
            "separator" => DemoKind::Separator,
            "collapsed" => DemoKind::Collapsed,
            "link" => DemoKind::Link,
            "rtl" => DemoKind::Rtl,
            _ => DemoKind::Basic,
        };
        let d = |b: Breadcrumb| b.dark(dark);
        let breadcrumb = match kind {
            DemoKind::Basic => d(Breadcrumb::new()),
            DemoKind::Separator => d(Breadcrumb::new().separator(CrumbSeparator::Dot)),
            DemoKind::Collapsed => d(Breadcrumb::new().items(vec![
                CrumbItem::link("Home"),
                CrumbItem::Ellipsis,
                CrumbItem::link("Components"),
                CrumbItem::page("Breadcrumb"),
            ])),
            DemoKind::Link => d(Breadcrumb::new()),
            DemoKind::Rtl => d(Breadcrumb::new().rtl(true)),
        };

        let breadcrumb = cx.new(|_| breadcrumb);
        let subscriptions = vec![cx.subscribe(
            &breadcrumb,
            |demo, _, event: &BreadcrumbNavigateEvent, cx| {
                demo.outcome = Some(format!("Navigate to {}", event.label).into());
                cx.notify();
            },
        )];
        Self {
            breadcrumb,
            dark,
            outcome: None,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for BreadcrumbDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let surface = rgb(if self.dark { 0x202020 } else { 0xf8f8f8 });
        let muted = rgb(if self.dark { 0xa3a3a3 } else { 0x737373 });
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
                    .child(self.breadcrumb.clone())
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
        cx.new(|cx| BreadcrumbDemo::new(&variant, dark, cx))
    })
    .expect("open breadcrumb gallery");
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
    use super::{Breadcrumb, CrumbItem, CrumbSeparator, SharedString};

    #[test]
    fn builders_store_content() {
        let breadcrumb = Breadcrumb::new()
            .items(vec![
                CrumbItem::link("Home"),
                CrumbItem::Ellipsis,
                CrumbItem::page("Breadcrumb"),
            ])
            .separator(CrumbSeparator::Dot)
            .rtl(true)
            .dark(true);
        assert_eq!(breadcrumb.items.len(), 3);
        assert_eq!(breadcrumb.items[1], CrumbItem::Ellipsis);
        assert_eq!(breadcrumb.separator, CrumbSeparator::Dot);
        assert!(breadcrumb.rtl && breadcrumb.dark);
    }

    #[test]
    fn item_builders() {
        assert_eq!(
            CrumbItem::link("Home"),
            CrumbItem::Link(SharedString::from("Home"))
        );
        assert_eq!(
            CrumbItem::page("Doc"),
            CrumbItem::Page(SharedString::from("Doc"))
        );
    }
}
