use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct NickGroup {
    id: i64,
    parent_group_id: i64,
    name: String,
    color_name: String,
    color: String,
    visible: bool,
    groups: Vec<NickGroup>,
    nicks: Vec<Nick>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Nick {
    id: i64,
    parent_group_id: i64,
    prefix: String,
    prefix_color_name: String,
    prefix_color: String,
    name: String,
    color_name: String,
    color: String,
    visible: bool,
}
