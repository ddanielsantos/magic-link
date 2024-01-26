use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::post;
use crate::state::AppState;

async fn register_handler(
    State(state): State<AppState>
) -> impl IntoResponse {

}

async fn login_handler(
    State(state): State<AppState>
) -> impl IntoResponse {

}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
}
