use crate::commands::flip::handle_flip_command;
use crate::discord_types::{CreateApplicationCommand, Interaction, InteractionResponse};
use crate::errors::InteractionError;
use anyhow::{anyhow, Result};

mod flip;

pub async fn handle_command(
    interaction: &Interaction,
) -> Result<InteractionResponse, InteractionError> {
    match interaction.data.as_ref().unwrap().name.as_str() {
        "flip" => handle_flip_command(interaction).await,
        _ => Err(InteractionError {
            message: format!(
                "Unknown command: {}",
                interaction.data.as_ref().unwrap().name
            ),
        }),
    }
}
