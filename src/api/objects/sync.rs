use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
enum Colors {
    #[serde(rename = "ansi")]
    Ansi,
    #[serde(rename = "weechat")]
    Weechat,
    #[serde(rename = "strip")]
    Strip,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
struct Sync {
    #[serde(skip_serializing_if = "Option::is_none")]
    sync: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    nicks: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    colors: Option<Colors>,
}
