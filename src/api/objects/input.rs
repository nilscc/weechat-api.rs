use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum BufferRef {
    #[serde(rename = "buffer_id")]
    BufferId(i64),
    #[serde(rename = "buffer")]
    BufferName(String),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
struct Input {
    #[serde(flatten)]
    buffer_ref: BufferRef,
    command: String,
}
