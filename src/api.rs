use crate::domains::auth;
use crate::state::AppState;
use axum::Router;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/api", Router::new().nest("/auth", auth::auth_routes()))
}
