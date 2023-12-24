use crate::discord_types::{
    CreateApplicationCommand, Interaction, InteractionApplicationCommandCallbackData,
    InteractionCallbackType, InteractionResponse,
};
use crate::errors::InteractionError;
use rand::Rng;

pub async fn handle_flip_command(
    interaction: &Interaction,
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
