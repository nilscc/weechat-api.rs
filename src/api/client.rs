#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

use async_trait::async_trait;
use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::Client as ClientImpl;
// use reqwest_mock::client;
use serde::de::DeserializeOwned;
use url;

pub mod http;
pub mod websocket;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Error {
    InvalidUrl(url::ParseError),
    ReqwestError(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ReqwestError(value)
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Error::InvalidUrl(value)
    }
}

pub struct Credentials {
    pub host: String,
    pub port: i32,
    password: String,
}

impl Credentials {
    pub fn new(host: String, port: i32, password: String) -> Self {
        Credentials {
            host: host,
            port: port,
            password: password,
        }
    }

    /// Render base64 encoded `Authorization` header value
    pub fn authorization(&self) -> String {
        let b64 = BASE64_STANDARD.encode(format!("plain:{}", self.password));
        format!("Basic {b64}")
    }
}

pub struct ClientSettings {
    with_reqwest_client_builder: Box<dyn FnOnce(reqwest::ClientBuilder) -> reqwest::ClientBuilder>,
}

impl Default for ClientSettings {
    #[cfg(target_family = "wasm")]
    fn default() -> Self {
        ClientSettings {
            with_reqwest_client_builder: Box::new(|b| b),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn default() -> Self {
        ClientSettings {
            with_reqwest_client_builder: Box::new(|b| {
                b
                    // enable timeout
                    .connect_timeout(Duration::from_secs(5))
                    // enable https only
                    .https_only(true)
            }),
        }
    }
}

#[async_trait]
pub trait ApiClient {
    type Error;

    async fn get<T>(&mut self, path: &str) -> Result<T, Self::Error>
    where
        T: DeserializeOwned;
}
