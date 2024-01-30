use sqlx::{Pool, Sqlite};
use sqlx::types::time::OffsetDateTime;

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}

impl AppState {
    pub fn new(db: Pool<Sqlite> ) -> Self {
        AppState {
            db
        }
    }
}

#[derive(Clone)]
pub struct User {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime
}
