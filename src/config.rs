use dotenv::dotenv;
use serde_with::chrono::{Datelike, TimeZone, Utc};
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

pub fn get_all_allowed_card_types() -> Vec<String> {
    let raw_types = env::var("ALLOWED_CARD_TYPES").unwrap_or("black,white".to_string());
    raw_types
        .split(',')
        .map(|section| section.to_string())
        .collect()
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_sync_start_timestamp() -> u32 {
    let raw_timestamp = env::var("SYNC_START_TIMESTAMP");
    match raw_timestamp {
        Err(_) => {
            let now = Utc::now();
            let this_month = Utc.ymd(now.year(), now.month(), 1).and_hms(0, 0, 0);
            this_month.timestamp() as u32
        }
        Ok(string_timestamp) => string_timestamp
            .parse::<u32>()
            .expect("SYNC_START_TIMESTAMP must be a number"),
    }
}
