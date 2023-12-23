use dotenv::dotenv;
use std::env;

pub struct Globals {
    pub port: String,
    pub discord_public_key: String,
}

pub fn init_globals() -> Globals {
    dotenv().ok();
    Globals {
        port: env::var("PORT").unwrap_or(String::from("2997")),
        discord_public_key: env::var("DISCORD_PUBLIC_KEY").unwrap_or(String::from("")),
    }
}
