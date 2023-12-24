use rand::Rng;

use crate::discord_types::{
    Interaction, InteractionApplicationCommandCallbackData, InteractionCallbackType,
    InteractionResponse,
};
use crate::errors::InteractionError;

pub async fn handle_flip_command(
    _interaction: &Interaction,
) -> Result<InteractionResponse, InteractionError> {
    let mut rng = rand::thread_rng();
    let result = rng.gen_bool(0.5);

    Ok(InteractionResponse {
        r#type: InteractionCallbackType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: Some(if result { "Heads" } else { "Tails" }.to_string()),
            embeds: None,
            flags: None,
        }),
    })
}
