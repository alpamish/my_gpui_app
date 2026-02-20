use gpui::*;
use crate::app::AppState;
use crate::components::sidebar::Sidebar;

pub struct AppView {
    app_state: AppState,
    sidebar: View<Sidebar>,
}

impl AppView {
    pub fn new(app_state: AppState) -> Self {
        Self { 
            app_state,
            sidebar: View::new(Sidebar::new()),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = cx.theme().background;

        div()
            .size_full()
            .flex()
            .flex_row()
            .bg(bg)
            .text_color(cx.theme().foreground)
            .child(self.sidebar.clone())
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .child("Main Content Area")
            )
    }
}