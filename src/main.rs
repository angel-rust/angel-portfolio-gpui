use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Hsla,
    SharedString, Window, WindowBounds, WindowOptions,
};

/// Minimal Zed-like theme (dark graphite, thin borders, soft radii).
struct Theme {
    bg: Hsla,
    surface: Hsla,
    surface_alt: Hsla,
    border: Hsla,
    text: Hsla,
    muted: Hsla,
    accent: Hsla,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: rgb(0x0e0e10).into(),
            surface: rgb(0x15161a).into(),
            surface_alt: rgb(0x1a1b21).into(),
            border: rgb(0x24252e).into(),
            text: rgb(0xE6E6E6).into(),
            muted: rgb(0xA8A8B3).into(),
            accent: rgb(0x6df2a5).into(),
        }
    }
}

/// App model: just a name + theme for now.
struct Model {
    name: SharedString,
    theme: Theme,
}

impl Render for Model {
    fn render(&mut self, _win: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let t = &self.theme; // borrow, don't move

        // --- Header (thin bar) ---
        let header = div()
            .h(px(48.0))
            .px_4()
            .flex()
            .items_center()
            .justify_between()
            .bg(t.surface)
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(t.muted)
                    .child("ANGEL MEDINA"),
            )
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded(px(8.0))
                    .bg(t.surface_alt)
                    .border(px(1.0))
                    .border_color(t.border)
                    .text_size(px(12.0))
                    .text_color(t.muted)
                    .child("⌘K  Search / Command Palette"),
            );

        // Divider under header (avoid border_b methods for compatibility)
        let header_divider = div().h(px(1.0)).bg(t.border);

        // --- Helper: a simple card panel ---
        fn card(title: &str, body: &str, t: &Theme) -> impl IntoElement {
            div()
                .p_4()
                .rounded(px(12.0))
                .bg(t.surface)
                .border(px(1.0))
                .border_color(t.border)
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(16.0))
                        .text_color(t.text)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(t.muted)
                        .child(body.to_string()),
                )
        }

        // --- Landing content (static) ---
        let hero = div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_size(px(28.0))
                    .text_color(t.text)
                    .child(format!("Hi, I’m {}", self.name)),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .text_color(t.muted)
                    .child("Rust-native builder • Zed enjoyer • Systems thinker"),
            );

        let hero_divider = div().h(px(1.0)).bg(t.border);

        let features = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(card(
                "Now",
                "Learning GPUI + Rust. This landing page is a static, minimal sandbox to understand layout and styling.",
                t,
            ))
            .child(card(
                "Projects (preview)",
                "Exuro/Hologram sketches • Zed theme experiments • Small Rust crates.",
                t,
            ))
            .child(card(
                "Contact",
                "GitHub: github.com/angel-rust   •   Email: add your public email",
                t,
            ));

        let footer = div()
            .mt(px(12.0))
            .text_size(px(11.0))
            .text_color(t.muted)
            .child("© 2025 — Built with Rust + GPUI");

        // --- Frame (Zed-like rounded container) ---
        div()
            .bg(t.bg)
            .text_color(t.text)
            .size(px(1120.0)) // demo window size; remove to fill entire OS window if you prefer
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(t.border)
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(header)
            .child(header_divider)
            .child(
                div()
                    .p_8()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(hero)
                    .child(hero_divider)
                    .child(features)
                    .child(footer),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1120.0), px(720.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_win, cx| {
                cx.new(|_| Model {
                    name: "Angel Medina".into(),
                    theme: Theme::default(),
                })
            },
        )
        .unwrap();
    });
}
