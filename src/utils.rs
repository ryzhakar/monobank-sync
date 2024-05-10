use serde_with::chrono::{DateTime, Utc};

pub fn datetime_from(timestamp: u32) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(timestamp as i64, 0)
}
