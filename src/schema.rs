use serde::Deserialize;
use serde_with::chrono::{DateTime, Utc};
use serde_with::formats::Flexible;
use serde_with::TimestampSeconds;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub send_id: String,
    pub balance: i64,
    pub credit_limit: i64,
    #[serde(rename(deserialize = "type"))]
    pub account_type: String,
    pub currency_code: u32,
    pub cashback_type: Option<String>,
    pub iban: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jar {
    pub id: String,
    pub title: String,
    pub description: String,
    pub currency_code: u32,
    pub balance: i64,
    pub goal: Option<i64>,
}

#[serde_with::serde_as]
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatementItem {
    pub id: String,
    #[serde_as(as = "TimestampSeconds<i32, Flexible>")]
    pub time: DateTime<Utc>,
    pub description: String,
    pub mcc: u32,
    pub original_mcc: u32,
    pub hold: bool,
    pub amount: i64,
    pub operation_amount: i64,
    pub currency_code: u32,
    pub commission_rate: i64,
    pub cashback_amount: i64,
    pub balance: i64,
    pub comment: Option<String>,
    pub receipt_id: Option<String>,
    pub invoice_id: Option<String>,
    pub counter_edrpou: Option<String>,
    pub counter_iban: Option<String>,
    pub counter_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub client_id: String,
    pub name: String,
    pub accounts: Vec<Account>,
    jars: Option<Vec<Jar>>,
}
