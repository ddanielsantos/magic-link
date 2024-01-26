use axum::Router;
use crate::state::AppState;
use crate::domains::auth;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/api", Router::new()
            .nest("/auth", auth::auth_routes())
        )
}
