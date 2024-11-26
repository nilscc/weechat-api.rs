use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub enum BufferType {
    #[serde(rename = "formatted")]
    Formatted,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Buffer {
    pub id: i64,
    pub name: String,
    pub short_name: String,
    pub number: i32,
    #[serde(rename = "type")]
    pub buffer_type: BufferType,
    pub hidden: bool,
    pub title: String,
    pub modes: String,
    pub input_prompt: String,
    pub input: String,
    pub input_position: i32,
    pub input_multiline: bool,
    pub nicklist: bool,
    pub nicklist_case_sensitive: bool,
    pub nicklist_display_groups: bool,
    pub time_displayed: bool,
    pub local_variables: BTreeMap<String, String>,
    pub keys: Vec<String>,
}
