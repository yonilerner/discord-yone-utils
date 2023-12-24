use std::sync::Arc;

use axum::{routing::post, Router};
use clap::{ArgGroup, Parser};

use crate::globals::{init_globals, Globals};
use crate::interactions_handler::handle_interaction;
use crate::update_commands::update_commands;

mod commands;
mod discord_types;
mod errors;
mod globals;
mod interactions_handler;
mod update_commands;

struct AppState {
    globals: Globals,
}

#[derive(Parser, Debug)]
#[command(about)]
#[clap(group(
    ArgGroup::new("default")
        .required(true)
        .args(&["serve", "commands"]),
))]
struct CliArgs {
    /// Run the server
    #[arg(
        exclusive = true,
        long,
        default_missing_value = "true",
        group = "default"
    )]
    serve: bool,

    /// Update commands without running the server
    #[arg(
        exclusive = true,
        long,
        default_missing_value = "true",
        group = "default"
    )]
    commands: bool,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let app_state = Arc::new(AppState {
        globals: init_globals(),
    });

    if args.commands {
        update_commands(app_state.clone()).await;
        return;
    }

    let app = Router::new()
        .route("/interactions", post(handle_interaction))
        .with_state(app_state.clone());

    println!("Listening on port {}", app_state.globals.port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_state.globals.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
