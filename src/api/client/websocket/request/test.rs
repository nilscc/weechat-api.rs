use std::time::Duration;

use futures::stream;
use reqwest_websocket::Message;
use serde::Deserialize;
use serde_json::json;
use tokio::time::timeout;

use super::{Error, Handler};

#[derive(Deserialize, Debug, Eq, PartialEq)]
struct Body {
    pub a: bool,
    pub b: i32,
}

#[tokio::test]
async fn test_handler() {
    let mut handler = Handler::new();

    // get receiver for request 1234 and 5678
    let receiver_1234 = handler.request::<()>(1234);
    let receiver_5678 = handler.request::<Body>(5678);

    // before handling incoming
    let messages = vec![
        Message::Text(
            json!({
                // no body in response
                "request_id": "1234",
                "other_members": {},
            })
            .to_string(),
        ),
        Message::Binary(
            json!({
                // response with body
                "request_id": "5678",
                "body": {
                    "a": true,
                    "b": 5,
                },
            })
            .to_string()
            .as_bytes()
            .into(),
        ),
    ];

    // handle defined messages
    let stream = stream::iter(messages.iter().map(|msg| Ok::<Message, Error>(msg.clone())));
    let handling_result = timeout(Duration::from_secs(1), handler.handle(stream))
        .await
        .expect("Message handling timeout");

    println!("{handling_result:?}");
    assert!(handling_result.is_ok());

    // get results
    let receiver_1234_result = timeout(Duration::from_secs(1), receiver_1234)
        .await
        .expect("Timeout failed.");
    let receiver_5678_result = timeout(Duration::from_secs(1), receiver_5678)
        .await
        .expect("Timeout failed.");

    println!("{receiver_1234_result:?}");
    println!("{receiver_5678_result:?}");

    // check results
    assert_eq!(receiver_1234_result, Ok(None));
    assert_eq!(receiver_5678_result, Ok(Some(Body { a: true, b: 5 })));
}
