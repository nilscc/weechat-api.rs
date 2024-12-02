use reqwest::Client as ClientImpl;
use serde::de::DeserializeOwned;
use url::Url;

use super::{ClientSettings, Credentials, Error};

#[cfg(test)]
mod test;

#[derive(Debug, Clone)]
pub struct HttpClient {
    /// The actual [reqwest] client implementation
    client: ClientImpl,
    credentials: Credentials,
}

impl HttpClient {
    pub fn new(credentials: Credentials, settings: Option<ClientSettings>) -> Self {
        let settings = settings.unwrap_or(ClientSettings::default());
        let client = (settings.with_reqwest_client_builder)(ClientImpl::builder())
            .build()
            .expect("reqwest client failed to construct");
        HttpClient {
            client,
            credentials,
        }
    }

    pub async fn get<T>(&mut self, path: &str) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let url = Url::parse(
            format!(
                "https://{}:{}/{}",
                self.credentials.host,
                self.credentials.port,
                path.strip_prefix("/").unwrap_or(path)
            )
            .as_str(),
        )?;

        // fetch request
        let resp = self
            .client
            .get(url)
            // add auth header
            .header("Authorization", self.credentials.authorization())
            // perform request
            .send()
            .await?;

        // convert to json
        Ok(resp.json().await?)
    }
}
