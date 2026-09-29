//! Single-window gallery that mounts every component demo for native smoke tests.
//! Run with `--gallery`; scroll the list, interact with each demo in place.
use gpui_kit::{self as kit, *};
use std::borrow::Cow;

const FONT: &str = "Geist";

struct Section {
    name: &'static str,
    hint: &'static str,
    height: f32,
    view: AnyView,
}

type DemoBuilder = fn(&str, bool, &mut Window, &mut App) -> AnyView;

const DEMOS: &[(&str, &str, f32, DemoBuilder, &str)] = &[
    (
        "Accordion",
        "collapsible sections",
        400.,
        crate::accordion::demo_view,
        "default",
    ),
    (
        "Alert",
        "callout variants",
        190.,
        crate::alert::demo_view,
        "default",
    ),
    (
        "Alert Dialog",
        "modal confirmation",
        430.,
        crate::alert_dialog::demo_view,
        "default",
    ),
    (
        "Aspect Ratio",
        "media ratio box",
        340.,
        crate::aspect_ratio::demo_view,
        "default",
    ),
    (
        "Attachment",
        "file chip list",
        400.,
        crate::attachment::demo_view,
        "default",
    ),
    (
        "Avatar",
        "image · fallback · group",
        220.,
        crate::avatar::demo_view,
        "basic",
    ),
    (
        "Badge",
        "status labels",
        200.,
        crate::badge::demo_view,
        "default",
    ),
    (
        "Breadcrumb",
        "path navigation",
        170.,
        crate::breadcrumb::demo_view,
        "default",
    ),
    (
        "Bubble",
        "chat message",
        320.,
        crate::bubble::demo_view,
        "default",
    ),
    (
        "Button",
        "variants showcase",
        320.,
        crate::button::demo_view,
        "variants",
    ),
    (
        "Button Group",
        "segmented actions",
        230.,
        crate::button_group::demo_view,
        "basic",
    ),
    (
        "Calendar",
        "month grid",
        430.,
        crate::calendar::demo_view,
        "basic",
    ),
    (
        "Card",
        "content container",
        400.,
        crate::card::demo_view,
        "basic",
    ),
    (
        "Carousel",
        "slide deck",
        380.,
        crate::carousel::demo_view,
        "basic",
    ),
    ("Chart", "bar chart", 440., crate::chart::demo_view, "bar"),
    (
        "Checkbox",
        "checked states",
        260.,
        crate::checkbox::demo_view,
        "basic",
    ),
    (
        "Collapsible",
        "show / hide",
        340.,
        crate::collapsible::demo_view,
        "basic",
    ),
    (
        "Combobox",
        "filterable select",
        470.,
        crate::combobox::demo_view,
        "basic",
    ),
    (
        "Command",
        "cmdk palette",
        500.,
        crate::command::demo_view,
        "basic",
    ),
    (
        "Context Menu",
        "right-click menu",
        360.,
        crate::context_menu::demo_view,
        "basic",
    ),
    (
        "Data Table",
        "sortable grid",
        540.,
        crate::data_table::demo_view,
        "full",
    ),
    (
        "Date Picker",
        "popover calendar",
        560.,
        crate::date_picker::demo_view,
        "basic",
    ),
];

pub struct Gallery {
    sections: Vec<Section>,
    scroll: ScrollHandle,
    initial_offset: f32,
    offset_locked: bool,
}

impl Render for Gallery {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        // Re-assert the launch offset once real bounds exist — the first paint
        // runs with zero scroll bounds, which can leave the view mispositioned.
        if !self.offset_locked && self.scroll.bounds().size.height > px(1.) {
            self.scroll
                .set_offset(point(px(0.), px(self.initial_offset)));
            self.offset_locked = true;
        }
        let ink = rgb(0x171717);
        let muted = rgb(0x737373);
        let edge = rgb(0xe4e4e4);
        div()
            .size_full()
            .flex()
            .flex_col()
            .font_family(FONT)
            .bg(rgb(0xf4f4f5))
            .text_color(ink)
            .child(
                div()
                    .flex_none()
                    .flex()
                    .items_baseline()
                    .gap_3()
                    .px_6()
                    .py_4()
                    .border_b_1()
                    .border_color(edge)
                    .bg(rgb(0xffffff))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("GPUI Gallery"),
                    )
                    .child(div().text_xs().text_color(muted).child(format!(
                        "{} components — scroll, click, right-click",
                        self.sections.len()
                    ))),
            )
            .child(
                div()
                    .id("gallery-scroll")
                    .flex_1()
                    .min_h_0()
                    .overflow_y_scroll()
                    .track_scroll(&self.scroll)
                    .child(div().flex().flex_col().gap_6().p_6().children(
                        self.sections.iter().enumerate().map(|(i, s)| {
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .flex()
                                        .items_baseline()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::MEDIUM)
                                                .child(format!("{:02} {}", i + 1, s.name)),
                                        )
                                        .child(div().text_xs().text_color(muted).child(s.hint)),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .h(px(s.height))
                                        .overflow_hidden()
                                        .rounded_lg()
                                        .border_1()
                                        .border_color(edge)
                                        .child(s.view.clone()),
                                )
                        }),
                    )),
            )
    }
}

pub fn run_native() {
    // `--gallery [scroll_px]` — optional initial scroll offset for batched
    // screenshot passes over the long section list.
    let offset: f32 = std::env::args()
        .skip_while(|arg| arg != "--gallery")
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.);
    kit::application().run(move |cx| {
        kit::init(cx);
        cx.text_system()
            .add_fonts(vec![
                Cow::Borrowed(include_bytes!("../assets/Geist-Medium.ttf")),
                Cow::Borrowed(include_bytes!("../assets/Geist-Regular.ttf")),
            ])
            .expect("load Geist fonts");
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::centered(size(px(960.), px(900.)), cx)),
            ..Default::default()
        };
        cx.open_window(options, |window, cx| {
            let sections = DEMOS
                .iter()
                .map(|(name, hint, height, build, variant)| Section {
                    name,
                    hint,
                    height: *height,
                    view: build(variant, false, window, cx),
                })
                .collect();
            let scroll = ScrollHandle::new();
            scroll.set_offset(point(px(0.), px(offset)));
            cx.new(|_| Gallery {
                sections,
                scroll,
                initial_offset: offset,
                offset_locked: false,
            })
        })
        .expect("open gallery window");
        cx.on_window_closed(|cx, _| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
