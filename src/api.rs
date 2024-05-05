use crate::models::{ClientInfo, StatementItem};
use std::option::Option;
use reqwest;
use tracing;

const MONOBANK_API_URL: &str = "https://api.monobank.ua/personal";

pub fn get_client() -> reqwest::Client {
    reqwest::Client::new()
}


pub async fn fetch_client_info(
    client: &reqwest::Client,
    token: &str,
) -> Result<ClientInfo, reqwest::Error> {
    let url = format!("{}/client-info", MONOBANK_API_URL);
    tracing::info!(token=token, "Getting client data...");
    let response = client.get(url).header("X-Token", token).send().await?;
    tracing::debug!("Deserializing client data...");
    let client_info = response.json::<ClientInfo>().await?;
    Ok(client_info)
}


pub async fn fetch_statements(
    client: &reqwest::Client,
    resource_id: &str,
    from: u32,
    to: u32,
    token: &str,
) -> Result<Vec<StatementItem>, reqwest::Error> {
    let url = format!(
        "{}/statement/{}/{}/{}",
        MONOBANK_API_URL,
        resource_id,
        from,
        to,
    );
    tracing::info!(
        from_time = from,
        to_time = to,
        resource_id = resource_id,
        token = token,
        "Getting statements...",
    );
    let response = client.get(&url).header("X-Token", token).send().await?;
    tracing::debug!("Deserializing statements...");
    let statement_items = response.json::<Vec<StatementItem>>().await?;
    Ok(statement_items)
}
