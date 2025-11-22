use chrono_tz::Tz;
use dotenv::dotenv;
use serde_with::chrono::{Datelike, TimeZone, Utc};
use std::env;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_timezone() -> Tz {
    let tz_str = env::var("TIMEZONE").unwrap_or_else(|_| "Europe/Kyiv".to_string());
    tz_str.parse().unwrap_or_else(|_| {
        tracing::warn!("Invalid timezone '{}', falling back to Europe/Kyiv", tz_str);
        "Europe/Kyiv".parse().expect("Default timezone is valid")
    })
}

fn parse_comma_separated(value: &str) -> Vec<String> {
    value.split(',').map(|s| s.trim().to_string()).collect()
}

pub fn get_multiple_monobank_tokens() -> Vec<String> {
    let raw_tokens = env::var("MULTIPLE_MONOBANK_TOKENS").expect("MONOBANK_TOKEN must be set");
    parse_comma_separated(&raw_tokens)
}

pub fn get_all_allowed_card_types() -> Vec<String> {
    let raw_types = env::var("ALLOWED_CARD_TYPES").unwrap_or_else(|_| "black,white".to_string());
    parse_comma_separated(&raw_types)
}

pub fn get_database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_sync_start_timestamp() -> u32 {
    let raw_timestamp = env::var("SYNC_START_TIMESTAMP");
    match raw_timestamp {
        Err(_) => {
            let now = Utc::now();
            let this_month = Utc
                .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
                .unwrap();
            this_month.timestamp() as u32
        }
        Ok(string_timestamp) => string_timestamp
            .parse::<u32>()
            .expect("SYNC_START_TIMESTAMP must be a number"),
    }
}
