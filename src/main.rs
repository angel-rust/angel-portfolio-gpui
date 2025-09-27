use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, Hsla,
    SharedString, Window, WindowBounds, WindowOptions,
};

/// Zed-like theme (dark graphite, thin borders, soft radii).
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

struct Model {
    store_name: SharedString,
    theme: Theme,
}

impl Render for Model {
    fn render(&mut self, _win: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let t = &self.theme;

        // ---------- Header ----------
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
                    .text_size(px(12.0))
                    .text_color(t.muted)
                    .child("Cmd+K  Command Palette"),
            );
        let header_divider = div().h(px(1.0)).bg(t.border);

        // ---------- Helpers ----------
        fn tag(label: &str, t: &Theme, active: bool) -> impl IntoElement {
            let base = div()
                .px_3()
                .py_1()
                .rounded(px(8.0))
                .border(px(1.0))
                .text_size(px(12.0))
                .child(label.to_string());
            if active {
                base.bg(t.surface_alt)
                    .border_color(t.accent)
                    .text_color(t.accent)
            } else {
                base.bg(t.surface)
                    .border_color(t.border)
                    .text_color(t.muted)
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

        fn section_label(label: &str, t: &Theme) -> impl IntoElement {
            div()
                .text_size(px(12.0))
                .text_color(t.muted)
                .child(label.to_string())
        }

        fn card_panel(title: &str, body: impl IntoElement, t: &Theme) -> impl IntoElement {
            div()
                .p_4()
                .rounded(px(12.0))
                .bg(t.surface)
                .border(px(1.0))
                .border_color(t.border)
                .flex()
                .flex_col()
                .gap_3()
                .child(div().text_size(px(14.0)).text_color(t.accent).child(title.to_string()))
                .child(body)
        }

        // ---------- Left: Catalog ----------
        let category_bar = div()
            .flex()
            .gap_2()
            .child(tag("All", t, true))
            .child(tag("Coffee", t, false))
            .child(tag("Tea", t, false))
            .child(tag("Pastry", t, false))
            .child(tag("Bottled", t, false));

        let catalog_grid = div()
            .grid()
            .grid_cols(3)
            .gap(px(8.0))
            .child(product_tile("Espresso", "3.00", t))
            .child(product_tile("Americano", "3.50", t))
            .child(product_tile("Latte", "4.50", t))
            .child(product_tile("Cappuccino", "4.25", t))
            .child(product_tile("Mocha", "4.75", t))
            .child(product_tile("Cold Brew", "4.00", t))
            .child(product_tile("Tea", "2.75", t))
            .child(product_tile("Croissant", "3.25", t))
            .child(product_tile("Muffin", "2.95", t));

        let catalog = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(section_label("Catalog", t))
            .child(category_bar)
            .child(catalog_grid);

        // ---------- Right: Cart + Receipt ----------
        fn cart_line(name: &str, qty: u32, each: &str, t: &Theme) -> impl IntoElement {
            div()
                .py_2()
                .flex()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(t.text)
                        .child(format!("{} x{}", name, qty)),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(t.muted)
                        .child(format!("${}", each)),
                )
        }

        let cart_list = div()
            .flex()
            .flex_col()
            .gap_1()
            .child(cart_line("Latte", 2, "9.00", t))
            .child(cart_line("Croissant", 1, "3.25", t))
            .child(div().h(px(1.0)).bg(t.border));

        let totals = {
            let subtotal = "12.25";
            let tax = "1.01";
            let total = "13.26";

            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .child(div().text_size(px(12.0)).text_color(t.muted).child("Subtotal"))
                        .child(div().text_size(px(12.0)).text_color(t.muted).child(format!("${}", subtotal))),
                )
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .child(div().text_size(px(12.0)).text_color(t.muted).child("Tax"))
                        .child(div().text_size(px(12.0)).text_color(t.muted).child(format!("${}", tax))),
                )
                .child(div().h(px(1.0)).bg(t.border))
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .child(div().text_size(px(14.0)).text_color(t.text).child("Total"))
                        .child(div().text_size(px(14.0)).text_color(t.text).child(format!("${}", total))),
                )
        };

        let pay_button = div()
            .px_4()
            .py_2()
            .rounded(px(10.0))
            .bg(t.accent)
            .text_size(px(14.0))
            .text_color(rgb(0x0b0b0c)) // dark label on accent
            .border(px(1.0))
            .border_color(t.border)
            .child("Pay");

        let cart_panel = card_panel(
            "Cart",
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(cart_list)
                .child(totals)
                .child(pay_button),
            t,
        );

        let receipt_panel = {
            let body = div()
                .flex()
                .flex_col()
                .gap_1()
                .child(div().text_size(px(12.0)).text_color(t.muted).child("Order #1029"))
                .child(div().h(px(1.0)).bg(t.border))
                .child(div().text_size(px(12.0)).text_color(t.text).child("Latte x2 - $9.00"))
                .child(div().text_size(px(12.0)).text_color(t.text).child("Croissant x1 - $3.25"))
                .child(div().h(px(1.0)).bg(t.border))
                .child(div().text_size(px(12.0)).text_color(t.muted).child("Note: Thanks!"));
            card_panel("Receipt Preview", body, t)
        };

        let right_column = div()
            .flex()
            .flex_col()
            .gap_3()
            .child(cart_panel)
            .child(receipt_panel);

        // ---------- Main two-column layout ----------
        let body = div()
            .p_8()
            .flex()
            .gap_6()
            .child(div().flex_grow().child(catalog))
            .child(div().w(px(360.0)).child(right_column));

        // ---------- Frame ----------
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
            .child(header_divider)
            .child(body)
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_win, cx| {
                cx.new(|_| Model {
                    store_name: "ANGEL POS".into(),
                    theme: Theme::default(),
                })
            },
        )
        .unwrap();
    });
}
