use serde_with::chrono::NaiveDateTime;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "online", derive(sqlx::FromRow))]
pub struct LastSync {
    pub last_sync_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct ClientInfo {
    pub client_id: String,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: String,
    pub client_id: String,
    pub send_id: String,
    pub balance: i64,
    pub credit_limit: i64,
    pub account_type: String,
    pub currency_code: u32,
    pub cashback_type: Option<String>,
    pub iban: Option<String>,
    pub last_sync_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct StatementItem {
    pub id: String,
    pub account_id: String,
    pub time: NaiveDateTime,
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
