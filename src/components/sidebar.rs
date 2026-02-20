use gpui::*;
use crate::components::icons::Icon;

#[derive(Debug, Clone)]
pub struct SidebarItem {
    pub id: String,
    pub label: String,
    pub icon: Icon,
}

pub struct Sidebar {
    items: Vec<SidebarItem>,
    active_item: SharedString,
    is_collapsed: bool,
    width: Pixels,
}

impl Sidebar {
    pub fn new() -> Self {
        let items = vec![
            SidebarItem {
                id: "dashboard".to_string(),
                label: "Dashboard".to_string(),
                icon: Icon::Dashboard,
            },
            SidebarItem {
                id: "customers".to_string(),
                label: "Customers".to_string(),
                icon: Icon::Users,
            },
            SidebarItem {
                id: "sales".to_string(),
                label: "Sales".to_string(),
                icon: Icon::ShoppingCart,
            },
            SidebarItem {
                id: "purchase".to_string(),
                label: "Purchase".to_string(),
                icon: Icon::CreditCard,
            },
            SidebarItem {
                id: "users".to_string(),
                label: "Users".to_string(),
                icon: Icon::User,
            },
        ];

        Self {
            items,
            active_item: "dashboard".into(),
            is_collapsed: false,
            width: px(260.0),
        }
    }

    pub fn toggle_collapse(&mut self) {
        self.is_collapsed = !self.is_collapsed;
        if self.is_collapsed {
            self.width = px(60.0);
        } else {
            self.width = px(260.0);
        }
    }

    pub fn set_active_item(&mut self, item_id: &str) {
        self.active_item = item_id.into();
    }
}

impl Render for Sidebar {
    fn render(&mut self, _window: &mut Window, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let bg_color = hsla(218.0, 25.0, 7.0, 1.0); // #0b1220
        let border_right_color = hsla(222.0, 23.0, 16.0, 1.0); // #1f2937
        let hover_bg_color = hsla(220.0, 25.0, 9.0, 1.0); // #111827
        let active_bg_color = hsla(220.0, 20.0, 14.0, 1.0); // #172033
        let active_accent_color = hsla(199.0, 93.0, 48.0, 1.0); // #38bdf8

        div()
            .w(self.width)
            .h_full()
            .bg(bg_color)
            .border_r_1()
            .border_color(border_right_color)
            .flex()
            .flex_col()
            .children(
                // Header
                div()
                    .h(px(60.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_3()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(Icon::Logo.render(cx))
                            .when(!self.is_collapsed, |this| {
                                this.child(
                                    div()
                                        .text_xl()
                                        .font_semibold()
                                        .text_color(cx.theme().foreground)
                                        .child("PlatformName"),
                                )
                            }),
                    )
                    .child(
                        IconButton::new("toggle-sidebar", Icon::Menu)
                            .on_click(cx.listener(|sidebar, _, cx| {
                                sidebar.toggle_collapse();
                                cx.notify();
                            })),
                    ),
            )
            .child(
                // Navigation Items
                div()
                    .flex_1()
                    .pt_2()
                    .children(self.items.iter().map(|item| {
                        let is_active = self.active_item == item.id;
                        
                        div()
                            .h(px(44.0))
                            .w_full()
                            .flex()
                            .items_center()
                            .cursor_pointer()
                            .border_l_2()
                            .border_color(if is_active { active_accent_color } else { transparent_black() })
                            .bg(if is_active { active_bg_color } else { transparent_black() })
                            .hover(|style| {
                                if !is_active {
                                    style.bg(hover_bg_color)
                                } else {
                                    style.bg(active_bg_color)
                                }
                            })
                            .transition(Duration::from_millis(150))
                            .on_mouse_down(MouseButton::Left, {
                                let item_id = item.id.clone();
                                cx.listener(move |sidebar, _, cx| {
                                    sidebar.set_active_item(&item_id);
                                    cx.notify();
                                })
                            })
                            .pl_4()
                            .pr_3()
                            .gap_3()
                            .child(item.icon.render(cx))
                            .when(!self.is_collapsed, |this| {
                                this.child(
                                    div()
                                        .text_sm()
                                        .text_color(cx.theme().foreground)
                                        .child(item.label.clone()),
                                )
                            })
                    })),
            )
    }
}