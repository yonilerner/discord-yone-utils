use dotenv::dotenv;
use std::env;

pub struct Globals {
    pub port: String,
    pub discord_public_key: String,
    pub discord_api_endpoint: String,
    pub discord_bot_token: String,
    pub discord_application_id: String,
}

pub fn init_globals() -> Globals {
    dotenv().ok();
    Globals {
        port: env::var("PORT").unwrap_or(String::from("2997")),
        discord_public_key: env::var("DISCORD_PUBLIC_KEY").unwrap_or(String::from("")),
        discord_api_endpoint: env::var("DISCORD_API_ENDPOINT").unwrap_or(String::from("")),
        discord_bot_token: env::var("DISCORD_BOT_TOKEN").unwrap_or(String::from("")),
        discord_application_id: env::var("DISCORD_APPLICATION_ID").unwrap_or(String::from("")),
    }
}
