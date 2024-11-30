use std::collections::BTreeMap;
use std::fmt::Debug;

use futures::TryStream;
use futures::TryStreamExt;
use reqwest_websocket::Message;
use serde_json::Value;
use tokio::sync::oneshot;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum Error {
    UnexpectedMessage(Message),
    InvalidResponse(String),
    FailedToSendResponse(i64, String),
}

pub struct Handler<T> {
    requests: BTreeMap<i64, oneshot::Sender<Option<T>>>,
}

impl<T> Handler<T> {
    pub fn new() -> Self {
        Handler {
            requests: BTreeMap::new(),
        }
    }

    fn handle_text_response(&mut self, response: String) -> Result<(), Error>
    where
        T: From<Value> + Debug,
    {
        use Error::*;

        // parse json object
        let json: Value = serde_json::from_str(&response).or(Err(InvalidResponse(format!(
            "Failed to convert to value: {response:?}"
        ))))?;

        // find request ID in object
        let obj = json
            .as_object()
            .ok_or(Error::InvalidResponse(format!("Not an object: {json:?}")))?;
        let value = obj.get("request_id").ok_or(InvalidResponse(format!(
            "No key 'request_id' in object: {obj:?}"
        )))?;

        // parse as i64
        let request_id = {
            match value {
                Value::String(str) => str.parse::<i64>().or(Err(InvalidResponse(format!(
                    "Failed to parse request_id: {str:?}"
                )))),
                _ => Err(InvalidResponse(format!("Unexpected value: {value:?}"))),
            }
        }?;

        // find sender for request ID
        let sender = self
            .requests
            .remove(&request_id)
            .ok_or(FailedToSendResponse(
                request_id,
                format!("Response ID not found"),
            ))?;

        // convert body into object of type T and send through oneshot channel
        sender
            .send(obj.get("body").map(|value| value.clone().into()))
            .or(Err(FailedToSendResponse(
                request_id,
                "Something went wrong!".into(),
            )))?;

        // done
        Ok(())
    }

    pub fn request(&mut self, id: i64) -> oneshot::Receiver<Option<T>> {
        // create channel
        let (s, r) = oneshot::channel::<Option<T>>();

        self.requests.insert(id, s);

        // return receiver
        r
    }

    pub async fn handle<S>(&mut self, mut stream: S) -> Result<(), Error>
    where
        T: From<Value> + Debug,
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
