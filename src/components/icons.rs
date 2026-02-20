use gpui::*;

#[derive(Debug, Clone)]
pub enum Icon {
    Dashboard,
    Users,
    ShoppingCart,
    CreditCard,
    User,
    Logo,
    Menu,
}

impl Icon {
    pub fn render(&self, cx: &mut WindowContext) -> impl IntoElement {
        let fg = cx.theme().foreground;
        
        // Placeholder implementations for icons
        match self {
            Icon::Dashboard => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M3 4a1 1 0 011-1h16a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 12a1 1 0 011-1h8a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM15 8a1 1 0 011-1h4a1 1 0 011 1v10a1 1 0 01-1 1h-4a1 1 0 01-1-1V8z")
                        .fill(fg)
                )
            },
            Icon::Users => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M12 4.75a2.75 2.75 0 100 5.5 2.75 2.75 0 000-5.5zM8 4.75a2.75 2.75 0 100 5.5 2.75 2.75 0 000-5.5zM17 12a5 5 0 11-10 0 5 5 0 0110 0z")
                        .fill(fg)
                )
            },
            Icon::ShoppingCart => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M3 3h2l1.222 6.111L14 10.25l-1.25 6.5H5.25a1.75 1.75 0 01-1.743-1.606L3.5 15h11.763l-.617 3.2a1.5 1.5 0 001.486 1.8H17a1.5 1.5 0 001.5-1.5v-10A1.5 1.5 0 0017 4.5h-1.277l-.83-4H3V3zm14 3.5v-2h1.5v2H17zm-1.222 2L17 9.5h-1.222l-1.25-6h7.995l.977 4.889L20.75 8.5H15.778z")
                        .fill(fg)
                )
            },
            Icon::CreditCard => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M3 4a1 1 0 011-1h16a1 1 0 011 1v3a1 1 0 01-1 1H4a1 1 0 01-1-1V4zm0 7a1 1 0 011-1h16a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6z")
                        .fill(fg)
                )
            },
            Icon::User => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M12 12a5 5 0 100-10 5 5 0 000 10zM12 14c-4.41 0-8 3.59-8 8h16c0-4.41-3.59-8-8-8z")
                        .fill(fg)
                )
            },
            Icon::Logo => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5")
                        .fill(fg)
                )
            },
            Icon::Menu => {
                svg().size_4().text_color(fg).child(
                    Path::new()
                        .data("M3 4h18M3 8h18M3 12h18")
                        .stroke(fg)
                        .stroke_width(2)
                        .fill("none")
                )
            },
        }
    }
}