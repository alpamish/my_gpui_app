mod config;
mod db;

use std::path::PathBuf;
use std::sync::Arc;

use gpui::*;
use gpui_component::{Theme, ThemeRegistry, TitleBar, *};
use tokio::sync::RwLock;

struct AppView {
    db: Arc<RwLock<Option<db::Database>>>,
}

impl AppView {
    fn new(db: Arc<RwLock<Option<db::Database>>>) -> Self {
        Self { db }
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::load_config().unwrap_or_default();
    let db_config = config.database.clone();

    let (db, _users) = match db::Database::connect(&db_config).await {
        Ok(database) => {
            database.init_schema().await?;

            let existing = database.get_all_users().await?;
            if existing.is_empty() {
                database
                    .create_user(db::CreateUser {
                        name: "John Doe".into(),
                        email: "john@example.com".into(),
                    })
                    .await?;
                database
                    .create_user(db::CreateUser {
                        name: "Jane Smith".into(),
                        email: "jane@example.com".into(),
                    })
                    .await?;
                database
                    .create_user(db::CreateUser {
                        name: "Bob Wilson".into(),
                        email: "bob@example.com".into(),
                    })
                    .await?;
            }

            let users = database.get_all_users().await?;
            println!("Connected to database: {} with {} users", db_config.name, users.len());
            (Some(database), users)
        }
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            (None, vec![])
        }
    };

    let db = Arc::new(RwLock::new(db));

    let app = Application::new().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let dark_theme = ThemeRegistry::global(cx).default_dark_theme().clone();
        Theme::global_mut(cx).apply_config(&dark_theme);

        let _ = ThemeRegistry::watch_dir(PathBuf::from("themes"), cx, move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx)
                .themes()
                .get("Tokyo Night")
                .cloned()
            {
                Theme::global_mut(cx).apply_config(&theme);
            }
        });

        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|_| AppView::new(db.clone()));
            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });

    Ok(())
}
