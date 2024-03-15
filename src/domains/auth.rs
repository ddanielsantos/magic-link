use crate::state::{AppState, User};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use magic_link::email::{EmailSender, SESWrapper};
use magic_link::user_mailing;
use sqlx::query_as;

#[derive(serde::Deserialize)]
struct AuthPayload {
    email_address: String,
}

async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>,
) -> impl IntoResponse {
    if payload.email_address.is_empty() {
        return (StatusCode::BAD_REQUEST, "Email address is required").into_response();
    }

    let user = query_as!(
        User,
        "SELECT * FROM users WHERE email = ?",
        payload.email_address
    )
    .fetch_optional(&state.db)
    .await;

    let user = user.unwrap_or(None);

    match user {
        Some(user) => {
            let email = user_mailing::build_login_email(user.email);
            match SESWrapper::send_email(&email).await {
                Ok(_) => (StatusCode::OK).into_response(),
                Err(a) => a.into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Failed to proceed with the login").into_response(),
    }
}

#[derive(serde::Deserialize)]
struct AuthParams {
    token: String,
}

async fn token_handler(
    State(state): State<AppState>,
    Query(params): Query<AuthParams>,
) -> impl IntoResponse {
    (StatusCode::OK, format!("your token is {}", params.token)).into_response()
}
pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(token_handler))
        .route("/login", post(login_handler))
}
