use crate::models::{ClientInfo, StatementItem};
use std::{thread, time::Duration};
use reqwest::blocking::Client;
use tracing;
use rand::Rng;

const MONOBANK_API_URL: &str = "https://api.monobank.ua/personal";
const MAX_TIME_DIFF_SEC: u32 = 31 * 24 * 60 * 60;  // 31 days
pub const WAIT_TIME_SEC: u32 = 60;
pub const WAIT_JITTER_SEC: u32 = 5;

#[derive(Debug)]
pub struct FetchingStatementsIterator<'a> {
    pub client: &'a Client,
    pub token: String,
    pub account_id: String,
    pub last_success_time: u32,
    pub end_time: u32,
    pub wait_length_sec: u32,
    pub wait_jitter_sec: u32,
}

impl<'a> FetchingStatementsIterator<'a> {

    fn calculate_next_window(&self) -> (u32, u32) {
        let start = self.last_success_time + 1;
        let end = std::cmp::min(start + MAX_TIME_DIFF_SEC, self.end_time);
        (start, end)
    }


    fn sleep_with_jitter(&self) {
        let jitter = rand::thread_rng().gen_range(0..self.wait_jitter_sec*1000);
        let sleep_time = {
            Duration::from_secs(self.wait_length_sec as u64)
            + Duration::from_millis(jitter as u64)
        };
        thread::sleep(sleep_time);
    }


    fn fetch_next_batch(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<StatementItem>, reqwest::Error> {
        self.sleep_with_jitter();
        fetch_statements(
            self.client,
            &self.account_id,
            start,
            end,
            &self.token
        )
    }

    fn try_fetch(
        &self,
        start: u32,
        end: u32,
    ) -> Result<Vec<StatementItem>, reqwest::Error> {
        let result = self.fetch_next_batch(start, end);
        match result {
            Ok(data) if data.len() == 500 => {
                let delta = end - start;
                if delta < 2 {
                    return Ok(data)
                };
                self.try_fetch(start, start + delta / 2)
            },
            _ => result
        }
    }
}
impl<'a> Iterator for FetchingStatementsIterator<'a> {
    type Item = Result<Vec<StatementItem>, reqwest::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last_success_time >= self.end_time {
            return None
        }

        let (start, end) = self.calculate_next_window();
        match self.try_fetch(start, end) {
            Ok(data) => {
                self.last_success_time = std::cmp::min(end, self.end_time);
                Some(Ok(data))
            },
            Err(e) => Some(Err(e))
        }
    }
}


pub fn fetch_client_info(
    client: &Client,
    token: &str,
) -> Result<ClientInfo, reqwest::Error> {
    let url = format!("{}/client-info", MONOBANK_API_URL);
    tracing::info!(token=token, "Getting client data...");
    let response = client.get(url).header("X-Token", token).send()?;
    tracing::debug!("Deserializing client data...");
    let client_info = response.json::<ClientInfo>()?;
    Ok(client_info)
}


pub fn fetch_statements(
    client: &Client,
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
    let response = client.get(&url).header("X-Token", token).send()?;
    tracing::debug!("Deserializing statements...");
    let statement_items = response.json::<Vec<StatementItem>>()?;
    Ok(statement_items)
}
