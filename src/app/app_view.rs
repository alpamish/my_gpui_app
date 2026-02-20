use gpui::*;

use crate::app::AppState;

pub struct AppView {
    app_state: AppState,
}

impl AppView {
    pub fn new(app_state: AppState) -> Self {
        Self { app_state }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = cx.theme().background;
        let fg = cx.theme().foreground;

        div()
            .size_full()
            .flex_col()
            .bg(bg)
            .text_color(fg)
            .child(TitleBar::new().child("My Application"))
            .child(div().flex_1().p_4().child("Hello, World!"))
    }
}