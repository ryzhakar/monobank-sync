mod api;
mod config;
mod crud;
mod db;
mod db_types;
mod logger;
mod models;
mod schema;
mod utils;
use reqwest::blocking::Client;
use serde_with::chrono::NaiveDateTime;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    logger::initialize_logging();
    config::load_env();
    let time_floor = utils::datetime_from(config::get_sync_start_timestamp());
    let pool = db::initialize(&config::get_database_url()).await;
    let client = Client::new();
    let tokens = config::get_multiple_monobank_tokens();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    for token in &tokens {
        let raw_client_info = api::fetch_client_info(&client, token).unwrap();
        let client_info = models::ClientInfo {
            client_id: raw_client_info.client_id,
            name: raw_client_info.name,
            token: token.to_string(),
        };
        let _ = crud::insert_client_info(&pool, client_info.clone()).await;
        let relevant_accounts = raw_client_info
            .accounts
            .clone()
            .iter()
            .filter(|&ac| config::get_all_allowed_card_types().contains(&ac.account_type))
            .cloned()
            .collect::<Vec<schema::Account>>();
        for raw_account in relevant_accounts {
            let account_id = raw_account.id.clone();
            let maybe_success_time = crud::get_last_sync_time(&pool, account_id)
                .await
                .expect("Failed to get last sync time");
            let last_sync_time: NaiveDateTime;
            if let Some(lst) = maybe_success_time {
                last_sync_time = lst;
            } else {
                last_sync_time = time_floor;
            }
            let account = models::Account {
                id: raw_account.id,
                client_id: client_info.client_id.clone(),
                send_id: raw_account.send_id,
                iban: raw_account.iban,
                account_type: raw_account.account_type,
                currency_code: raw_account.currency_code,
                balance: raw_account.balance,
                credit_limit: raw_account.credit_limit,
                cashback_type: raw_account.cashback_type,
                last_sync_at: Some(last_sync_time),
            };
            let _ = crud::insert_account(&pool, account.clone()).await;
            let card_statements = api::FetchingStatementsIterator {
                client: &client,
                token: token.to_string(),
                account_id: account.id.clone(),
                last_success_time: (last_sync_time.and_utc().timestamp() - 1) as u32,
                end_time: now,
                wait_length_sec: api::WAIT_TIME_SEC,
                wait_jitter_sec: api::WAIT_JITTER_SEC,
            };
            for statement_response in card_statements {
                let raw_statements: Vec<schema::StatementItem>;
                match statement_response {
                    Ok((timestamp, s)) => {
                        raw_statements = s;
                        let last_success = utils::datetime_from(timestamp);
                        let _ = crud::update_last_sync_time(
                            &pool,
                            account.id.clone(),
                            Some(last_success),
                        )
                        .await;
                    }
                    Err((timestamp, e)) => {
                        tracing::error!("Error: {:?}", e);
                        let last_success = utils::datetime_from(timestamp);
                        let _ = crud::update_last_sync_time(
                            &pool,
                            account.id.clone(),
                            Some(last_success),
                        )
                        .await;
                        break;
                    }
                }
                let statements = raw_statements
                    .into_iter()
                    .map(|s| models::StatementItem {
                        id: s.id,
                        account_id: account.id.clone(),
                        time: utils::datetime_from_utc_to_tz(s.time),
                        description: s.description,
                        mcc: s.mcc,
                        original_mcc: s.original_mcc,
                        hold: s.hold,
                        amount: s.amount,
                        operation_amount: s.operation_amount,
                        currency_code: s.currency_code,
                        commission_rate: s.commission_rate,
                        cashback_amount: s.cashback_amount,
                        balance: s.balance,
                        comment: s.comment,
                        receipt_id: s.receipt_id,
                        invoice_id: s.invoice_id,
                        counter_iban: s.counter_iban,
                        counter_name: s.counter_name,
                        counter_edrpou: s.counter_edrpou,
                    })
                    .collect::<Vec<models::StatementItem>>();
                for statement_item in statements {
                    let _ = crud::insert_statement_item(&pool, statement_item).await;
                }
            }
        }
    }
}
