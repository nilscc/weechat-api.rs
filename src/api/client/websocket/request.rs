use std::collections::BTreeMap;
use std::fmt::Debug;
use std::future::Future;

use futures::TryStream;
use futures::TryStreamExt;
use reqwest_websocket::Message;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::from_value;
use serde_json::Value;
use tokio::sync::oneshot;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Error {
    UnexpectedMessage(Message),
    InvalidResponse(String),
    FailedToSendResponse(i64, String),
    NoResponse,
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;
        match (self, other) {
            (InvalidResponse(r1), InvalidResponse(r2)) => r1 == r2,
            (FailedToSendResponse(i1, s1), FailedToSendResponse(i2, s2)) => i1 == i2 && s1 == s2,
            (NoResponse, NoResponse) => true,
            // messages cannot be compared
            (UnexpectedMessage(_), UnexpectedMessage(_)) => false,
            _ => false,
        }
    }
}

#[derive(Deserialize)]
struct Response {
    pub request_id: String,
    pub body: Option<Value>,
}

#[derive(Debug)]
pub struct Handler {
    requests: BTreeMap<i64, oneshot::Sender<Option<Value>>>,
}

impl Handler {
    pub fn new() -> Self {
        Handler {
            requests: BTreeMap::new(),
        }
    }

    fn handle_text_response(&mut self, response: String) -> Result<(), Error> {
        use Error::*;

        // parse json object
        let response: Response = serde_json::from_str(&response).or(Err(InvalidResponse(
            format!("Failed to convert to Response: {response:?}"),
        )))?;

        // parse request ID as i64
        let request_id: i64 = response.request_id.parse().or(Err(InvalidResponse(
            "Expected i64 parsable request_id value.".into(),
        )))?;

        // find sender for request ID
        let sender = self
            .requests
            .remove(&request_id)
            .ok_or(FailedToSendResponse(
                request_id,
                format!("Response ID not found"),
            ))?;

        // convert body into object of type T and send through oneshot channel
        sender.send(response.body).or(Err(FailedToSendResponse(
            request_id,
            "Something went wrong!".into(),
        )))
    }

    pub fn request<T>(&mut self, id: i64) -> impl Future<Output = Result<Option<T>, Error>>
    where
        T: DeserializeOwned,
    {
        // create channel
        let (s, r) = oneshot::channel::<Option<Value>>();

        self.requests.insert(id, s);

        // return receiver
        async {
            match r.await {
                Ok(Some(value)) => from_value(value).map_err(|e| {
                    Error::InvalidResponse(format!("Failed to deserialize reponse body: {e:?}"))
                }),
                Ok(None) => Ok(None),
                other => Err(Error::InvalidResponse(format!(
                    "Receiver failed with: {other:?}"
                ))),
            }
        }
    }

    pub async fn handle<S>(&mut self, mut stream: S) -> Result<(), Error>
    where
        S: TryStream<Ok = Message> + TryStreamExt + Unpin,
        Error: From<<S as TryStream>::Error>,
    {
        loop {
            match stream.try_next().await? {
                // text or binary messages
                Some(Message::Text(text)) => {
                    // ignore future
                    self.handle_text_response(text)?;
                }
                Some(Message::Binary(binary)) => {
                    // try decode binary
                    let text = String::from_utf8(binary.clone()).map_err(|e| {
                        Error::InvalidResponse(format!("Error: {e}\nInput: {binary:?}"))
                    })?;
                    // ignore future
                    self.handle_text_response(text)?;
                }

                // connection closed by remote
                None => return Ok(()),
                // explicit close
                Some(Message::Close {
                    code: _code,
                    reason: _reason,
                }) => return Ok(()),

                // other messages like ping/pong we currently do not handle
                Some(msg) => return Err(Error::UnexpectedMessage(msg)),
            }
        }
    }
}
