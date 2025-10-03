//! TREZZA TERMINAL - GPUI-based Point of Sale
//!
//! A modern, fast point of sale system built with GPUI

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Hsla,
    SharedString, Window, WindowBounds, WindowOptions,
};
use log::info;
use shared::APP_NAME;

/// TREZZA TERMINAL theme
struct Theme {
    bg: Hsla,
    surface: Hsla,
    surface_alt: Hsla,
    border: Hsla,
    text: Hsla,
    muted: Hsla,
    accent: Hsla,
    error: Hsla,
    success: Hsla,
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
            error: rgb(0xf87171).into(),
            success: rgb(0x4ade80).into(),
        }
    }
}

struct MainView {
    theme: Theme,
    store_name: SharedString,
}

impl Render for MainView {
    fn render(&mut self, _win: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let t = &self.theme;

        // Header
        let header = div()
            .h(px(48.0))
            .px_4()
            .flex()
            .items_center()
            .justify_between()
            .bg(t.surface)
            .child(
                div()
                    .text_size(px(14.0))
                    .text_color(t.accent)
                    .child(self.store_name.clone()),
            )
            .child(
                div()
                    .px_3()
                    .py_1()
                    .rounded(px(8.0))
                    .bg(t.surface_alt)
                    .border(px(1.0))
                    .border_color(t.border)
                    .text_size(px(11.0))
                    .text_color(t.muted)
                    .child("Ready"),
            );

        // Product catalog (placeholder)
        let catalog = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(t.muted)
                    .child("Product Catalog"),
            )
            .child(
                div()
                    .grid()
                    .grid_cols(3)
                    .gap(px(8.0))
                    .child(product_tile("Espresso", "3.00", t))
                    .child(product_tile("Americano", "3.50", t))
                    .child(product_tile("Latte", "4.50", t))
                    .child(product_tile("Cappuccino", "4.25", t))
                    .child(product_tile("Mocha", "4.75", t))
                    .child(product_tile("Cold Brew", "4.00", t)),
            );

        // Cart panel (placeholder)
        let cart_panel = div()
            .p_4()
            .rounded(px(12.0))
            .bg(t.surface)
            .border(px(1.0))
            .border_color(t.border)
            .flex()
            .flex_col()
            .gap_3()
            .child(div().text_size(px(14.0)).text_color(t.accent).child("Cart"))
            .child(div().text_size(px(12.0)).text_color(t.muted).child("Empty"));

        // Main layout
        let body = div()
            .p_8()
            .flex()
            .gap_6()
            .child(div().flex_grow().child(catalog))
            .child(div().w(px(360.0)).child(cart_panel));

        // Root container
        div()
            .bg(t.bg)
            .text_color(t.text)
            .size(px(1280.0))
            .rounded(px(16.0))
            .border(px(1.0))
            .border_color(t.border)
            .overflow_hidden()
            .flex()
            .flex_col()
            .child(header)
            .child(div().h(px(1.0)).bg(t.border))
            .child(body)
    }
}

fn product_tile(name: &str, price: &str, t: &Theme) -> impl IntoElement {
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
                .text_size(px(14.0))
                .text_color(t.text)
                .child(name.to_string()),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(t.muted)
                .child(format!("${}", price)),
        )
}

fn main() {
    env_logger::init();
    info!("Starting {} v{}", APP_NAME, env!("CARGO_PKG_VERSION"));

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_win, cx| {
                cx.new(|_| MainView {
                    store_name: APP_NAME.into(),
                    theme: Theme::default(),
                })
            },
        )
        .unwrap();
    });
}
