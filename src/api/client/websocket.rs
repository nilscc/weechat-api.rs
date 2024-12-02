use std::{string::FromUtf8Error, sync::Arc};

#[cfg(not(target_family = "wasm"))]
use std::time::Duration;

use request::Handler;
use reqwest_websocket::RequestBuilderExt;
use tokio::sync::Mutex;
use url::Url;

use crate::api::client::{self, ClientImpl, ClientSettings, Credentials};

pub mod request;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum WebsocketError {
    GenericError(client::Error),
    WebsocketError(reqwest_websocket::Error),
    NotConnected,
    AlreadyConnected,
    NotImplemented,
    RequestError(request::Error),
    DecodingError(FromUtf8Error),
}

impl<T> From<T> for WebsocketError
where
    T: Into<client::Error>,
{
    fn from(value: T) -> Self {
        Self::GenericError(value.into())
    }
}

impl From<reqwest_websocket::Error> for WebsocketError {
    fn from(value: reqwest_websocket::Error) -> Self {
        Self::WebsocketError(value)
    }
}

impl From<request::Error> for WebsocketError {
    fn from(value: request::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<FromUtf8Error> for WebsocketError {
    fn from(value: FromUtf8Error) -> Self {
        Self::DecodingError(value)
    }
}

#[derive(Debug, Clone)]
pub struct WebsocketClient {
    /// Connection timeout
    #[cfg(not(target_family = "wasm"))]
    pub timeout: Duration,

    credentials: Credentials,
    client: reqwest::Client,
    websocket: Arc<Mutex<Option<Handler>>>,
}

impl WebsocketClient {
    pub fn new(credentials: Credentials, settings: Option<ClientSettings>) -> Self {
        let settings = settings.unwrap_or(ClientSettings::default());
        let client = (settings.with_reqwest_client_builder)(ClientImpl::builder())
            .build()
            .expect("reqwest client failed to construct");

        #[cfg(target_family = "wasm")]
        let ws = WebsocketClient {
            client,
            credentials,
            websocket: Arc::new(Mutex::new(None)),
        };

        #[cfg(not(target_family = "wasm"))]
        let ws = WebsocketClient {
            client,
            credentials,
            websocket: Arc::new(Mutex::new(None)),
            timeout: Duration::from_secs(10),
        };

        ws
    }

    pub async fn connect(&mut self) -> Result<(), WebsocketError> {
        let url = Url::parse(
            format!(
                "wss://{}:{}/api",
                self.credentials.host, self.credentials.port,
            )
            .as_str(),
        )?;

        // get and assign websocket
        let builder = self.client.get(url);

        #[cfg(not(target_family = "wasm"))]
        let builder = builder
            // configure timeout
            .timeout(self.timeout);

        let req = builder
            // headers
            .header("Authorization", self.credentials.authorization())
            .header("Origin", "*") // needs to be set!
            // perform websocket upgrade
            .upgrade()
            .send()
            .await?;

        // store request socket if successfull
        let _ws = req.into_websocket().await?;
        {
            //let mut lock = self.websocket.lock().await;
            //*lock = Some(RequestSocket::new(ws))
        }

        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), WebsocketError> {
        match self.websocket.lock().await.take() {
            Some(_ws) => {
                // ws.close(CloseCode::Normal, None).await?;
                Ok(())
            }
            None => Err(WebsocketError::NotConnected),
        }
    }

    pub async fn has_websocket(&self) -> bool {
        self.websocket.lock().await.is_some()
    }

    pub async fn handle_requests(&mut self) -> Result<(), WebsocketError> {
        Ok(())
    }

    /*
    fn to_request<B>(id: i64, request: &str, body: Option<B>) -> Value
    where
        B: Serialize,
    {
        let mut json = json!({
            "request_id": id,
            "request": request,
        });
        if let Some(body) = body {
            json.as_object_mut()
                .unwrap()
                .insert("body".into(), to_value(body).unwrap());
        }
        json
    }

    async fn request<B, T>(&mut self, request: &str, body: Option<B>) -> Result<T, WebsocketError>
    where
        B: Serialize,
        T: DeserializeOwned,
    {
        let _req = Self::to_request(0, request, body);

        Err(WebsocketError::NotImplemented)
    }

    async fn get<T>(&mut self, path: &str) -> Result<T, WebsocketError>
    where
        T: DeserializeOwned,
    {
        self.request::<(), T>(format!("GET {path}").as_str(), None)
            .await
    }
    */
}
