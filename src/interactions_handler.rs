use crate::discord_types::{InteractionResponse, InteractionResponseType};
use crate::AppState;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::{debug_handler, Json};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use std::sync::Arc;

fn verify_interaction_signature(
    headers: HeaderMap,
    body_string: String,
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
    verify_interaction_signature(headers, body_string, &state.globals.discord_public_key)?;

    Ok(Json(InteractionResponse {
        r#type: InteractionResponseType::Pong,
    }))
}
