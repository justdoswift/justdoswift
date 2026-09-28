//! A desktop-first command palette. The native and WASM hosts share this module.
pub mod accordion;
pub mod action_swap;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod attachment;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod bubble;
pub mod button;
pub mod button_group;
pub mod button_metallic;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod chart;

pub mod motion_button;
pub mod motion_tabs;

use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{self as kit, *};
use kit::base::input::{Input, InputBase, InputEditorStyle, InputEvent, InputState};

#[derive(Clone, Debug)]
pub struct Command {
    pub id: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub category: &'static str,
    pub keywords: &'static str,
    pub glyph: &'static str,
}

/// Emitted once when a command is activated. The host owns the actual side effect.
#[derive(Clone, Debug)]
pub struct CommandExecuted(pub Command);

pub struct CommandPalette {
    commands: Vec<Command>,
    matches: Vec<usize>,
    selected: usize,
    input: Entity<InputState>,
    focus: FocusHandle,
    scroll: ScrollHandle,
    open: bool,
    last_command: Option<Command>,
    _subscription: Subscription,
}

impl EventEmitter<CommandExecuted> for CommandPalette {}

fn matching(commands: &[Command], query: &str) -> Vec<usize> {
    let query = query.to_lowercase();
    let words: Vec<_> = query.split_whitespace().collect();
    commands
        .iter()
        .enumerate()
        .filter_map(|(i, command)| {
            let text = format!(
                "{} {} {} {}",
                command.label, command.description, command.category, command.keywords
            )
            .to_lowercase();
            words.iter().all(|word| text.contains(word)).then_some(i)
        })
        .collect()
}

impl CommandPalette {
    pub fn new(commands: Vec<Command>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            let mut state = InputState::new(window, cx).placeholder("Search commands…");
            state.set_editor_style(InputEditorStyle {
                foreground: rgb(0xededef).into(),
                muted_foreground: rgb(0x85858e).into(),
                caret: rgb(0xc4b5fd).into(),
                selection: rgba(0x8b5cf650).into(),
                ..Default::default()
            });
            state.focus(window, cx);
            state
        });
        let subscription = cx.subscribe_in(
            &input,
            window,
            |this: &mut Self, _, event, window, cx| match event {
                InputEvent::Change => {
                    this.matches = matching(&this.commands, &this.input.read(cx).value());
                    this.selected = 0;
                    this.scroll.set_offset(point(px(0.), px(0.)));
                    cx.notify();
                }
                InputEvent::PressEnter { .. } => this.execute(window, cx),
                _ => {}
            },
        );
        Self {
            matches: (0..commands.len()).collect(),
            commands,
            selected: 0,
            input,
            focus: cx.focus_handle(),
            scroll: ScrollHandle::new(),
            open: true,
            last_command: None,
            _subscription: subscription,
        }
    }

    pub fn show(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open = true;
        self.selected = 0;
        self.input.update(cx, |input, cx| {
            input.set_value("", window, cx);
            input.focus(window, cx);
        });
        self.matches = (0..self.commands.len()).collect();
        cx.notify();
    }

    fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open = false;
        self.focus.focus(window, cx);
        cx.notify();
    }

    fn execute(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(&index) = self.matches.get(self.selected) {
            let command = self.commands[index].clone();
            self.last_command = Some(command.clone());
            self.close(window, cx);
            cx.emit(CommandExecuted(command));
        }
    }

    fn key_down(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let key = event.keystroke.key.as_str();
        let modifiers = event.keystroke.modifiers;
        if key == "k" && (modifiers.platform || modifiers.control) {
            if self.open {
                self.close(window, cx);
            } else {
                self.show(window, cx);
            }
            cx.stop_propagation();
        } else if self.open && matches!(key, "up" | "down" | "escape") {
            if key == "escape" {
                self.close(window, cx);
            } else if !self.matches.is_empty() {
                let len = self.matches.len();
                self.selected = if key == "down" {
                    (self.selected + 1) % len
                } else {
                    (self.selected + len - 1) % len
                };
                self.scroll.scroll_to_item(self.selected);
                cx.notify();
            }
            cx.stop_propagation();
        }
    }
}

impl Render for CommandPalette {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let input = self.input.clone();
        let rows = self
            .matches
            .iter()
            .enumerate()
            .map(|(position, &index)| {
                let command = &self.commands[index];
                let selected = position == self.selected;
                div()
                    .id(("command", index))
                    .flex()
                    .items_center()
                    .gap_3()
                    .px_3()
                    .py_3()
                    .rounded_lg()
                    .cursor_pointer()
                    .bg(if selected {
                        rgb(0x303039)
                    } else {
                        rgb(0x202024)
                    })
                    .hover(|s| s.bg(rgb(0x2a2a31)))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.selected = position;
                        this.execute(window, cx);
                    }))
                    .child(
                        div()
                            .w_8()
                            .h_8()
                            .rounded_md()
                            .bg(rgb(0x37343f))
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(rgb(0xc4b5fd))
                            .child(command.glyph),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xf1f0f5))
                                    .child(command.label),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x9998a3))
                                    .child(command.description),
                            ),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x9998a3))
                            .child(if selected { "Enter" } else { command.category }),
                    )
            })
            .collect::<Vec<_>>();

        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .p_5()
            .font_family(".SystemUIFont")
            .bg(rgb(0x141417))
            .text_color(rgb(0xededef))
            .track_focus(&self.focus)
            .capture_key_down(cx.listener(Self::key_down))
            .child(
                div()
                    .w_full()
                    .max_w(px(540.))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .text_xs()
                            .text_color(rgb(0x85858e))
                            .child("WORKSPACE / COMMANDS")
                            .child("01"),
                    )
                    .when(self.open, |root| {
                        root.child(
                            div()
                                .w_full()
                                .rounded_xl()
                                .overflow_hidden()
                                .bg(rgb(0x202024))
                                .border_1()
                                .border_color(rgb(0x39383f))
                                .shadow_lg()
                                .child(
                                    InputBase::new("command-search")
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .p_4()
                                        .border_b_1()
                                        .border_color(rgb(0x34333b))
                                        .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                            input.update(cx, |input, cx| input.focus(window, cx));
                                        })
                                        .child(div().text_color(rgb(0xa99fc8)).child("/"))
                                        .child(
                                            div().flex_1().min_w_0().child(Input::new(&self.input)),
                                        )
                                        .child(
                                            div()
                                                .id("close-palette")
                                                .cursor_pointer()
                                                .text_xs()
                                                .text_color(rgb(0x9998a3))
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.close(window, cx)
                                                }))
                                                .child("esc"),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_4()
                                        .pt_3()
                                        .pb_2()
                                        .text_xs()
                                        .text_color(rgb(0x85858e))
                                        .child(format!(
                                            "SUGGESTED · {} COMMANDS",
                                            self.matches.len()
                                        )),
                                )
                                .child(
                                    div()
                                        .id("command-results")
                                        .max_h(px(300.))
                                        .overflow_y_scroll()
                                        .track_scroll(&self.scroll)
                                        .px_2()
                                        .pb_2()
                                        .children(rows)
                                        .when(self.matches.is_empty(), |list| {
                                            list.child(
                                                div()
                                                    .p_8()
                                                    .text_sm()
                                                    .text_color(rgb(0x9998a3))
                                                    .child(
                                                        "No commands found. Try “theme” or “file”.",
                                                    ),
                                            )
                                        }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .px_4()
                                        .py_3()
                                        .border_t_1()
                                        .border_color(rgb(0x34333b))
                                        .text_xs()
                                        .text_color(rgb(0x9998a3))
                                        .child("↑ ↓ Navigate     Enter to run")
                                        .child("GPUI / Rust"),
                                ),
                        )
                    })
                    .when(!self.open, |root| {
                        root.child(
                            div()
                                .p_8()
                                .rounded_xl()
                                .bg(rgb(0x202024))
                                .border_1()
                                .border_color(rgb(0x39383f))
                                .flex()
                                .flex_col()
                                .gap_4()
                                .child(
                                    div().text_lg().child(
                                        self.last_command
                                            .as_ref()
                                            .map(|c| format!("Executed: {}", c.label))
                                            .unwrap_or_else(|| "Ready when you are.".into()),
                                    ),
                                )
                                .child(div().text_sm().text_color(rgb(0x9998a3)).child(
                                    "Demo received the command. Your app supplies the action.",
                                ))
                                .child(
                                    div()
                                        .id("open-palette")
                                        .cursor_pointer()
                                        .p_3()
                                        .rounded_lg()
                                        .bg(rgb(0x393044))
                                        .text_sm()
                                        .on_click(
                                            cx.listener(|this, _, window, cx| {
                                                this.show(window, cx)
                                            }),
                                        )
                                        .child("Open command menu     Cmd / Ctrl K"),
                                ),
                        )
                    }),
            )
    }
}

pub fn demo_commands() -> Vec<Command> {
    vec![
        Command {
            id: "new-file",
            label: "Create a new file",
            description: "Start something worth building",
            category: "Workspace",
            keywords: "document create",
            glyph: "+",
        },
        Command {
            id: "open-project",
            label: "Open project",
            description: "Pick up where you left off",
            category: "Workspace",
            keywords: "folder directory",
            glyph: "↗",
        },
        Command {
            id: "search",
            label: "Search everywhere",
            description: "Find files, symbols and ideas",
            category: "Navigation",
            keywords: "find query",
            glyph: "/",
        },
        Command {
            id: "theme",
            label: "Change appearance",
            description: "Make this workspace feel like you",
            category: "Settings",
            keywords: "theme dark light color",
            glyph: "*",
        },
        Command {
            id: "shortcuts",
            label: "Keyboard shortcuts",
            description: "Keep your hands on the keyboard",
            category: "Settings",
            keywords: "keys bindings",
            glyph: "K",
        },
        Command {
            id: "terminal",
            label: "New terminal",
            description: "A fresh shell for your next idea",
            category: "Workspace",
            keywords: "console command shell",
            glyph: ">_",
        },
    ]
}

fn setup(cx: &mut App) {
    kit::init(cx);
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/IBMPlexSans-Regular.ttf"
        ))])
        .expect("load bundled font");
    let options = WindowOptions {
        #[cfg(not(target_family = "wasm"))]
        window_bounds: Some(WindowBounds::centered(size(px(760.), px(600.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, |window, cx| {
        cx.new(|cx| CommandPalette::new(demo_commands(), window, cx))
    })
    .expect("open command palette");
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
pub fn run_native() {
    kit::application().run(setup);
}

#[cfg(target_family = "wasm")]
thread_local! { static APPLICATION: std::cell::RefCell<Option<ApplicationHandle>> = const { std::cell::RefCell::new(None) }; }

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(setup);
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(test)]
mod tests {
    use super::{demo_commands, matching};
    #[test]
    fn search_supports_keywords_case_and_multiple_terms() {
        let commands = demo_commands();
        assert_eq!(matching(&commands, " THEME "), vec![3]);
        assert_eq!(matching(&commands, "file create"), vec![0]);
        assert!(matching(&commands, "no-such-command").is_empty());
        assert_eq!(matching(&commands, "  ").len(), commands.len());
    }
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_action_swap(dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        action_swap::setup(dark, cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
fn announce_preview_after_first_frame(cx: &mut App) {
    // The web platform initializes graphics asynchronously before calling the
    // application setup. Its canvas exists earlier, so DOM insertion and the
    // exported run_* function returning are not usable readiness signals.
    if let Some(handle) = cx.windows().first().copied() {
        let _ = handle.update(cx, |_, window, _| schedule_preview_ready(window));
    }
}

#[cfg(target_family = "wasm")]
fn schedule_preview_ready(window: &mut Window) {
    window.on_next_frame(|window, _| {
        let size = window.viewport_size();
        if size.width <= px(0.) || size.height <= px(0.) {
            // ResizeObserver has not established the real viewport yet. The
            // initial GPU surface is only 1 × 1 and is not the preview frame.
            schedule_preview_ready(window);
            return;
        }

        // GPUI runs frame callbacks before draw/present. Force a real frame,
        // then announce readiness on the following frame, after submission.
        window.refresh();
        window.on_next_frame(|window, _| {
            let size = window.viewport_size();
            if size.width <= px(0.) || size.height <= px(0.) {
                schedule_preview_ready(window);
                return;
            }
            if let Some(browser) = web_sys::window()
                && let Ok(event) = web_sys::Event::new("gpui-preview-ready")
            {
                let _ = browser.dispatch_event(&event);
            }
        });
    });
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_metallic(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web()
        .with_assets(motion_button::ButtonAssets)
        .run_embedded(move |cx| {
            cx.set_reduce_motion(reduced_motion);
            motion_button::setup(&variant, dark, cx);
            announce_preview_after_first_frame(cx);
        });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_button(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        button::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_button_group(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        button_group::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_calendar(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        calendar::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_card(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        card::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_carousel(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        carousel::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_chart(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        chart::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_accordion(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        accordion::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_alert(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        alert::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_alert_dialog(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        alert_dialog::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_aspect_ratio(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        aspect_ratio::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_tabs(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web()
        .with_assets(motion_button::ButtonAssets)
        .run_embedded(move |cx| {
            cx.set_reduce_motion(reduced_motion);
            motion_tabs::setup(&variant, dark, cx);
            announce_preview_after_first_frame(cx);
        });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_attachment(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        attachment::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_avatar(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        avatar::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_badge(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        badge::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_breadcrumb(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        breadcrumb::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn run_bubble(variant: String, dark: bool, reduced_motion: bool) {
    console_error_panic_hook::set_once();
    kit::platform::web_init();
    let handle = kit::platform::single_threaded_web().run_embedded(move |cx| {
        cx.set_reduce_motion(reduced_motion);
        bubble::setup(&variant, dark, cx);
        announce_preview_after_first_frame(cx);
    });
    APPLICATION.with(|application| *application.borrow_mut() = Some(handle));
}
