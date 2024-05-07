mod config;
mod models;
mod api;
mod logger;
use reqwest::blocking::Client;
use std::time::{SystemTime, UNIX_EPOCH};


fn main() {
    logger::initialize_logging();
    config::load_env();
    let client = Client::new();
    let tokens = config::get_multiple_monobank_tokens();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32;
    let before: u32 = now - 24 * 60 * 60 * 31 * 2;
    for token_index in 0..tokens.len() {
        let token = &tokens[token_index];
        let client_info = api::fetch_client_info(&client, &token).unwrap();
        println!("{:?}", client_info);
        for card in client_info.accounts {
            let card_statements = api::FetchingStatementsIterator {
                client: &client,
                token: token.to_string(),
                account_id: card.id,
                last_success_time: before - 1,
                end_time: now,
                wait_length_sec: api::WAIT_TIME_SEC,
                wait_jitter_sec: api::WAIT_JITTER_SEC,
            };
            for statement in card_statements {
                println!("{:?}", statement);
            }
        }
    }
}
