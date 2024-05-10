use dotenv::dotenv;
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_multiple_monobank_tokens() -> Vec<String> {
    let raw_tokens = env::var("MULTIPLE_MONOBANK_TOKENS").expect("MONOBANK_TOKEN must be set");
    raw_tokens
        .split(',')
        .map(|section| section.to_string())
        .collect()
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_sync_start_timestamp() -> u32 {
    env::var("SYNC_START_TIMESTAMP")
        .expect("SYNC_START_TIMESTAMP must be set")
        .parse::<u32>()
        .expect("SYNC_START_TIMESTAMP must be a number")
}
