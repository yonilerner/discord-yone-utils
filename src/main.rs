mod discord_types;
mod globals;
mod interactions_handler;

use crate::globals::{init_globals, Globals};
use crate::interactions_handler::handle_interaction;
use axum::{routing::post, Router};
use std::sync::Arc;

struct AppState {
    globals: Globals,
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        globals: init_globals(),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/interactions", post(handle_interaction))
        .with_state(app_state.clone());

    println!("Listening on port {}", app_state.globals.port);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_state.globals.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
