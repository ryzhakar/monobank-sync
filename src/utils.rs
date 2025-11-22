use crate::config;
use chrono_tz::Tz;
use serde_with::chrono::{DateTime, NaiveDateTime, Utc};

pub fn datetime_from(timestamp: u32) -> NaiveDateTime {
    let tz: Tz = config::get_timezone();
    let utc_dt = DateTime::from_timestamp(timestamp as i64, 0)
        .expect("Failed to convert timestamp to DateTime");

    // Convert UTC to configured timezone and return as naive
    utc_dt.with_timezone(&tz).naive_local()
}

pub fn datetime_from_utc_to_tz(utc_dt: DateTime<Utc>) -> NaiveDateTime {
    let tz: Tz = config::get_timezone();
    utc_dt.with_timezone(&tz).naive_local()
}
