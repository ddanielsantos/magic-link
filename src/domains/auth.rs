use std::collections::HashMap;
use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use sqlx::{query_as};
use magic_link::ses::send_email;
use crate::state::{AppState, User};

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

    let users = query_as!(User, "SELECT * FROM users WHERE email = ?", payload.email_address)
        .fetch_optional(&state.db)
        .await;

    let user = users.unwrap_or(None);

    match user {
        Some(user) => {
            send_email(&user.email).await;
            (StatusCode::OK, "email sent").into_response()
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
    Query(params): Query<AuthParams>
) -> impl IntoResponse {
    (StatusCode::OK, format!("your token is {}", params.token)).into_response()
}
pub fn auth_routes() -> Router<AppState> {
    Router::new().route("/", get(token_handler)).route("/login", post(login_handler))
}
