use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Version {
    weechat_version: String,
    weechat_version_git: String,
    weechat_version_number: i32,
    relay_api_version: String,
    relay_api_version_number: i32,
}
