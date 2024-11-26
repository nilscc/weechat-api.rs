use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct Handshake {
    pub password_hash_algo: String,
    pub password_hash_iterations: i32,
    pub totp: bool,
}
