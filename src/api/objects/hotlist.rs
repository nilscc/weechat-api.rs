use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Hotlist {
    pub priority: i32,
    pub date: String,
    pub buffer_id: i64,
    pub count: Vec<i32>,
}
