use serde::{Deserialize, Serialize};
use time::{serde::rfc3339, OffsetDateTime};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Line {
    pub id: i64,
    pub y: i32,
    #[serde(with = "rfc3339")]
    pub date: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub date_printed: OffsetDateTime,
    pub displayed: bool,
    pub highlight: bool,
    pub notify_level: i32,
    pub prefix: String,
    pub message: String,
    pub tags: Vec<String>,
}
