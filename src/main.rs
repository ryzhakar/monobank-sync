mod config;
mod models;
mod api;
mod logger;
use std::time::{SystemTime, UNIX_EPOCH};


#[tokio::main]
async fn main() {
    logger::initialize_logging();
    config::load_env();
    let client = api::get_client();
    let tokens = config::get_multiple_monobank_tokens();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let before = now - 24 * 60 * 31;
    for token_index in 0..tokens.len() {
        let token = &tokens[token_index];
        let client_info = api::fetch_client_info(&client, &token).await.unwrap();
        println!("{:?}", client_info);
        let proper_type = "black".to_string();
        let proper_currency = 980;
        let main_card = client_info.clone().accounts.into_iter()
            .find(
                |acc|
                acc.account_type == proper_type
                && acc.currency_code == proper_currency
            ).unwrap();
        let statements = api::fetch_statements(
            &client,
            &main_card.id,
            before as u32,
            now as u32,
            &token
        ).await.unwrap();
        let _: Vec<_> = statements.into_iter()
            .map(move |stat| { println!("{:?}", stat); }).collect();
    }
}
