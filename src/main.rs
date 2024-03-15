use crate::state::AppState;
use tokio::net::TcpListener;

mod api;
mod db;
mod domains;
mod state;

#[tokio::main]
async fn main() {
    let pool = db::create_connection_pool().await;

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to migrate database");
    let state = AppState::new(pool);

    let api = api::routes().with_state(state);

    let tcp_listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(tcp_listener, api).await.unwrap();
}
