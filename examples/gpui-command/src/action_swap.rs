//! A small, native copy button with animated in-place feedback.
//! Both the desktop host and the WASM showcase render this exact component.
use gpui_kit::{self as kit, *};
use kit::base::Button;
use std::time::Duration;

// Shared native SVG masks let GPUI's resvg renderer apply a Gaussian blur to
// both the text outlines and icon. No DOM/CSS animation is involved.
const COPY: &str = include_str!("../assets/swap-copy.svg");
const CHECK: &str = include_str!("../assets/swap-copied.svg");
const RETRY: &str = include_str!("../assets/swap-retry.svg");

// CSS ease-out: cubic-bezier(0, 0, 0.58, 1), matching the reference keyframes.
fn ease_out(time: f32) -> f32 {
    if time <= 0. {
        return 0.;
    }
    if time >= 1. {
        return 1.;
    }
    let (mut low, mut high) = (0., 1.);
    for _ in 0..16 {
        let t = (low + high) * 0.5;
        let x = 3. * (1. - t) * t * t * 0.58 + t * t * t;
        if x < time {
            low = t;
        } else {
            high = t;
        }
    }
    let t = (low + high) * 0.5;
    3. * (1. - t) * t * t + t * t * t
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Feedback {
    Idle,
    Copied,
    Failed,
}
impl Feedback {
    fn label(self) -> &'static str {
        match self {
            Self::Idle => "Copy code",
            Self::Copied => "Copied",
            Self::Failed => "Try again",
        }
    }
    fn artwork(self) -> &'static str {
        match self {
            Self::Idle => COPY,
            Self::Copied => CHECK,
            Self::Failed => RETRY,
        }
    }
}

/// Construct inside `cx.new`; pass the exact text to copy.
/// Owns feedback, keyboard activation, duplicate-click protection and timer cleanup.
pub struct ActionSwapButton {
    text: SharedString,
    state: Feedback,
    generation: usize,
    pending: bool,
    task: Option<Task<()>>,
}

impl ActionSwapButton {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            state: Feedback::Idle,
            generation: 0,
            pending: false,
            task: None,
        }
    }

    fn transition(&mut self, state: Feedback, cx: &mut Context<Self>) {
        self.state = state;
        self.generation = self.generation.wrapping_add(1);
        cx.notify();
    }

    fn finish(&mut self, success: bool, cx: &mut Context<Self>) {
        self.pending = false;
        self.transition(
            if success {
                Feedback::Copied
            } else {
                Feedback::Failed
            },
            cx,
        );
        self.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(1600))
                .await;
            let _ = this.update(cx, |this, cx| this.transition(Feedback::Idle, cx));
        }));
    }

    fn copy(&mut self, cx: &mut Context<Self>) {
        // Repeated clicks cannot schedule overlapping feedback/reset sequences.
        if self.pending || self.state == Feedback::Copied {
            return;
        }
        self.task = None;
        self.pending = true;
        #[cfg(not(target_family = "wasm"))]
        {
            cx.write_to_clipboard(ClipboardItem::new_string(self.text.to_string()));
            self.finish(true, cx);
        }
        #[cfg(target_family = "wasm")]
        {
            // Request clipboard access during the user gesture. Only show Copied
            // once the browser's async write succeeds; denial remains retryable.
            let Some(window) = web_sys::window() else {
                self.finish(false, cx);
                return;
            };
            let promise = window.navigator().clipboard().write_text(&self.text);
            self.task = Some(cx.spawn(async move |this, cx| {
                let success = wasm_bindgen_futures::JsFuture::from(promise).await.is_ok();
                let _ = this.update(cx, |this, cx| this.finish(success, cx));
            }));
        }
        cx.notify();
    }

    fn content(state: Feedback, progress: f32) -> Div {
        // Finite blur levels keep GPUI's SVG cache bounded across repeated clicks.
        // 1/32 px steps are smaller than one device pixel at common display scales.
        let blur = ((4. * (1. - progress)) * 32.).round() / 32.;
        let artwork = state.artwork().replace("__BLUR__", &format!("{blur:.5}"));
        let direction = if state == Feedback::Idle { 1. } else { -1. };
        div()
            .relative()
            .w(px(112.))
            .h(px(42.))
            .opacity(progress)
            .child(
                svg()
                    .data(artwork.as_bytes())
                    .w(px(112.))
                    .h(px(42.))
                    // Match CSS translateY at paint time. Layout offsets are
                    // rounded to device pixels, stepping this short movement.
                    .with_transformation(Transformation::translate(point(
                        px(0.),
                        px(direction * 4. * (1. - progress)),
                    )))
                    .text_color(rgb(0xffffff)),
            )
    }
}

impl Render for ActionSwapButton {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let animated = self.generation > 0 && !cx.reduce_motion();
        let state = self.state;
        let incoming = if animated {
            div()
                .with_animation(
                    ("swap-in", self.generation),
                    Animation::new(Duration::from_millis(280)).with_easing(ease_out),
                    move |el, progress| el.child(Self::content(state, progress)),
                )
                .into_any_element()
        } else {
            Self::content(state, 1.).into_any_element()
        };
        Button::new("action-swap")
            .accessibility_label(if self.pending {
                "Copying code"
            } else {
                self.state.label()
            })
            .w(px(130.))
            .h(px(42.))
            .rounded_full()
            .border_1()
            .border_color(rgb(0x232323))
            .bg(rgb(0x171717))
            .text_color(rgb(0xffffff))
            .font_family("IBM Plex Sans")
            .flex()
            .items_center()
            .justify_center()
            .cursor_pointer()
            .shadow(vec![
                BoxShadow::new(px(0.), px(3.), rgba(0x00000012).into()).blur_radius(px(5.)),
            ])
            .hover(|style| style.bg(rgb(0x262626)))
            .active(|style| style.bg(rgb(0x363636)))
            .focus_visible(|style| style.border_color(rgb(0x839fd1)))
            .on_click(cx.listener(|this, _, _, cx| this.copy(cx)))
            .child(div().w(px(112.)).h(px(42.)).child(incoming))
    }
}

pub struct ActionSwapDemo {
    button: Entity<ActionSwapButton>,
    dark: bool,
}
impl ActionSwapDemo {
    pub fn new(dark: bool, cx: &mut Context<Self>) -> Self {
        Self {
            button: cx.new(|_| ActionSwapButton::new(include_str!("action_swap.rs"))),
            dark,
        }
    }
}
impl Render for ActionSwapDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(if self.dark {
                rgb(0x202020)
            } else {
                rgb(0xf8f8f8)
            })
            .child(self.button.clone())
    }
}

pub fn setup(dark: bool, cx: &mut App) {
    kit::init(cx);
    cx.text_system()
        .add_fonts(vec![std::borrow::Cow::Borrowed(include_bytes!(
            "../assets/IBMPlexSans-Regular.ttf"
        ))])
        .expect("load bundled font");
    let options = WindowOptions {
        #[cfg(not(target_family = "wasm"))]
        window_bounds: Some(WindowBounds::centered(size(px(700.), px(420.)), cx)),
        ..Default::default()
    };
    cx.open_window(options, |_, cx| cx.new(|cx| ActionSwapDemo::new(dark, cx)))
        .expect("open action swap");
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
    kit::application().run(|cx| setup(false, cx));
}

#[cfg(test)]
mod motion_tests {
    use super::ease_out;

    #[test]
    fn easing_matches_css_ease_out_samples() {
        for (time, expected) in [
            (0., 0.),
            (0.25, 0.378138),
            (0.5, 0.684643),
            (0.75, 0.906535),
            (1., 1.),
        ] {
            assert!((ease_out(time) - expected).abs() < 0.0001);
        }
    }
}
