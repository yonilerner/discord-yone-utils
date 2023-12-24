use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;

pub type Snowflake = String;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u32)]
pub enum MessageFlags {
    Ephemeral = 1 << 6,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ApplicationIntegrationType {
    GuildInstall = 0,
    UserInstall = 1,
}
impl ApplicationIntegrationType {
    pub fn all() -> Vec<ApplicationIntegrationType> {
        vec![
            ApplicationIntegrationType::GuildInstall,
            ApplicationIntegrationType::UserInstall,
        ]
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionCallbackType {
    Pong = 1,
    Acknowledge = 2,
    ChannelMessage = 3,
    ChannelMessageWithSource = 4,
    AcknowledgeWithSource = 5,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionResponse {
    pub r#type: InteractionCallbackType,
    pub data: Option<InteractionApplicationCommandCallbackData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InteractionApplicationCommandCallbackData {
    pub content: Option<String>,
    pub embeds: Option<Vec<Value>>,
    // pub allowed_mentions: Option<Value>,
    pub flags: Option<u32>,
    // pub components: Option<Vec<Value>>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ApplicationCommandType {
    Chat = 1,
    User = 2,
    Message = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateApplicationCommand {
    pub r#type: ApplicationCommandType,
    pub name: String,
    pub description: Option<String>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub contexts: Vec<ApplicationCommandContext>,
    pub integration_types: Vec<ApplicationIntegrationType>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ApplicationCommandContext {
    Guild = 0,
    BotDM = 1,
    PrivateChannel = 2,
}

impl ApplicationCommandContext {
    pub fn all() -> Vec<ApplicationCommandContext> {
        vec![
            ApplicationCommandContext::Guild,
            ApplicationCommandContext::BotDM,
            ApplicationCommandContext::PrivateChannel,
        ]
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandOption {
    pub name: String,
    pub description: String,
    pub r#type: ApplicationCommandOptionType,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub value: Value,
    pub name_localizations: Option<HashMap<String, String>>,
}

#[derive(Clone, Eq, PartialEq)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i64),
    Number(Number),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    pub id: Snowflake,
    pub application_id: Snowflake,
    pub r#type: InteractionType,
    pub data: Option<ApplicationCommandInteractionData>,
    pub guild_id: Option<Snowflake>,
    pub channel_id: Option<Snowflake>,
    // pub member: Option<GuildMember>,
    // pub user: Option<User>,
    pub token: String,
    // pub message: Option<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionData {
    pub id: Snowflake,
    pub name: String,
    pub r#type: ApplicationCommandType,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApplicationCommandInteractionDataOption {
    pub r#type: ApplicationCommandOptionType,
    pub name: String,
    pub value: Option<Value>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
}
