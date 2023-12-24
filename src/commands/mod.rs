use crate::commands::choice::handle_choice_command;
use crate::commands::flip::handle_flip_command;
use crate::discord_types::{Interaction, InteractionResponse};
use crate::errors::InteractionError;

mod choice;
mod flip;

pub async fn handle_command(
    interaction: &Interaction,
) -> Result<InteractionResponse, InteractionError> {
    match interaction.data.as_ref().unwrap().name.as_str() {
        "flip" => handle_flip_command(interaction).await,
        "choice" => handle_choice_command(interaction).await,
        _ => Err(InteractionError {
            message: format!(
                "Unknown command: {}",
                interaction.data.as_ref().unwrap().name
            ),
        }),
    }
}
