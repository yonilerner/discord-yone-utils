use std::sync::Arc;

use crate::discord_types::{
    ApplicationCommandContext, ApplicationCommandOption, ApplicationCommandOptionType,
    ApplicationCommandType, ApplicationIntegrationType, CreateApplicationCommand,
};
use crate::AppState;

pub async fn update_commands(app_state: Arc<AppState>) {
    println!("Updating commands");

    let commands = vec![
        CreateApplicationCommand {
            r#type: ApplicationCommandType::Chat,
            name: String::from("flip"),
            description: Some(String::from("Flip a coin.")),
            options: None,
            contexts: ApplicationCommandContext::all(),
            integration_types: ApplicationIntegrationType::all(),
        },
        CreateApplicationCommand {
            r#type: ApplicationCommandType::Chat,
            name: String::from("choice"),
            description: Some(String::from(
                "Get a random choice from a comma-separated list of items.",
            )),
            options: Some(vec![ApplicationCommandOption {
                r#type: ApplicationCommandOptionType::String,
                name: String::from("items"),
                description: String::from("Comma-separated list of items."),
                required: Some(true),
                choices: None,
                options: None,
            }]),
            contexts: ApplicationCommandContext::all(),
            integration_types: ApplicationIntegrationType::all(),
        },
    ];

    let client = reqwest::Client::new();
    let response = client
        .put(format!(
            "{}/applications/{}/commands",
            app_state.globals.discord_api_endpoint, app_state.globals.discord_application_id
        ))
        .header(
            "Authorization",
            format!("Bot {}", app_state.globals.discord_bot_token),
        )
        .json(&commands)
        .send()
        .await;

    if let Ok(response) = response {
        if response.status() == reqwest::StatusCode::OK {
            match response.json::<Vec<CreateApplicationCommand>>().await {
                Ok(json) => {
                    println!("Updated commands: {:?}", json);
                }
                Err(error) => {
                    println!("Error parsing response: {:?}", error);
                }
            }
        } else {
            println!("Error updating commands: {:?}", response.text().await);
        }
    } else {
        println!("Error updating commands: {:?}", response);
    }
}
