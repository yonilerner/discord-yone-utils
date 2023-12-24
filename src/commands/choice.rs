use crate::discord_types::{
    Interaction, InteractionApplicationCommandCallbackData, InteractionCallbackType,
    InteractionResponse,
};
use crate::errors::InteractionError;
use rand::prelude::SliceRandom;
use serde_json::Value;

pub async fn handle_choice_command(
    interaction: &Interaction,
) -> Result<InteractionResponse, InteractionError> {
    let choices_value = interaction
        .data
        .as_ref()
        .ok_or(InteractionError {
            message: "No data provided".to_string(),
        })?
        .options
        .as_ref()
        .ok_or(InteractionError {
            message: "No options provided".to_string(),
        })?
        .first()
        .ok_or(InteractionError {
            message: "Options are empty".to_string(),
        })?
        .value
        .as_ref()
        .ok_or(InteractionError {
            message: "Option has no value".to_string(),
        })?;
    let choices_string = match choices_value {
        Value::String(s) => s,
        _ => {
            return Err(InteractionError {
                message: "Option value is not a string".to_string(),
            })
        }
    };

    let choices = choices_string.split(',').collect::<Vec<&str>>();
    let result = choices
        .choose(&mut rand::thread_rng())
        .ok_or(InteractionError {
            message: "No choices provided".to_string(),
        })?;

    Ok(InteractionResponse {
        r#type: InteractionCallbackType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: Some(format!("`{}` is the chosen one!", result.trim())),
            embeds: None,
            flags: None,
        }),
    })
}
