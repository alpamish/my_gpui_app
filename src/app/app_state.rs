use std::sync::Arc;
use tokio::sync::RwLock;

use crate::db::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<RwLock<Option<Database>>>,
}

impl AppState {
    pub fn new(db: Arc<RwLock<Option<Database>>>) -> Self {
        Self { db }
    }
}