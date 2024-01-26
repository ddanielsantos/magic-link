use tokio::net::TcpListener;
use crate::state::AppState;

mod state;
mod domains;
mod api;

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let api = api::routes()
        .with_state(state);

    let tcp_listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(tcp_listener, api).await.unwrap();
}
