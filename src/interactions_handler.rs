use crate::commands::handle_command;
use crate::discord_types::{
    Interaction, InteractionApplicationCommandCallbackData, InteractionCallbackType,
    InteractionResponse, InteractionType, MessageFlags,
};
use crate::errors::InteractionError;
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{debug_handler, Json};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use std::sync::Arc;

fn verify_interaction_signature(
    headers: HeaderMap,
    body_string: &String,
    public_key: &String,
) -> Result<(), StatusCode> {
    let signature = headers
        .get("x-signature-ed25519")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let timestamp = headers
        .get("x-signature-timestamp")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .to_str()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let public_key = hex::decode(public_key).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let public_key: &[u8; PUBLIC_KEY_LENGTH] = &public_key
        .try_into()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let public_key = VerifyingKey::from_bytes(public_key).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let message = format!("{timestamp}{body_string}");

    let signature = hex::decode(signature).map_err(|_| StatusCode::UNAUTHORIZED)?;
    let signature =
        Signature::from_slice(signature.as_slice()).map_err(|_| StatusCode::UNAUTHORIZED)?;

    public_key
        .verify(message.as_bytes(), &signature)
        .map_err(|_| StatusCode::UNAUTHORIZED)
}

#[debug_handler]
pub async fn handle_interaction(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    body_string: String,
) -> Result<Json<InteractionResponse>, StatusCode> {
    verify_interaction_signature(headers, &body_string, &state.globals.discord_public_key)?;

    let interaction: Interaction = serde_json::from_str(&body_string).map_err(|e| {
        eprintln!("Error parsing interaction: {}", e);
        StatusCode::BAD_REQUEST
    })?;

    let result = match interaction.r#type {
        InteractionType::Ping => {
            return Ok(Json(InteractionResponse {
                r#type: InteractionCallbackType::Pong,
                data: None,
            }))
        }
        InteractionType::ApplicationCommand => handle_command(&interaction).await,
        InteractionType::MessageComponent => Err(InteractionError {
            message: "Message components are not supported".to_string(),
        }),
        InteractionType::ApplicationCommandAutocomplete => Err(InteractionError {
            message: "Autocomplete not supported".to_string(),
        }),
        InteractionType::ModalSubmit => Err(InteractionError {
            message: "Modals are not supported".to_string(),
        }),
    };

    match result {
        Ok(response) => Ok(Json(response)),
        Err(error) => {
            eprintln!("Error handling interaction: {}", error.message);
            Ok(Json(InteractionResponse {
                r#type: InteractionCallbackType::ChannelMessageWithSource,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: Some(format!("Error handling interaction: {}", error.message)),
                    embeds: None,
                    flags: Some(MessageFlags::Ephemeral as u32),
                }),
            }))
        }
    }
}
