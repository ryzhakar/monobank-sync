use serde_with::chrono::{DateTime, NaiveDateTime};

pub fn datetime_from(timestamp: u32) -> NaiveDateTime {
    DateTime::from_timestamp(timestamp as i64, 0)
        .expect("Failed to convert timestamp to NaiveDateTime")
        .naive_utc()
}
