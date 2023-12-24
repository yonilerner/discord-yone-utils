use crate::discord_types::{
    ApplicationCommandContext, ApplicationCommandType, ApplicationIntegrationType,
    CreateApplicationCommand,
};
use crate::AppState;
use std::sync::Arc;

pub async fn update_commands(app_state: Arc<AppState>) {
    println!("Updating commands");

    let commands = vec![CreateApplicationCommand {
        r#type: ApplicationCommandType::Chat,
        name: String::from("flip"),
        description: Some(String::from("Flip a coin.")),
        options: None,
        contexts: ApplicationCommandContext::all(),
        integration_types: ApplicationIntegrationType::all(),
    }];

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
