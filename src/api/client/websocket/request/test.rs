use std::time::Duration;

use futures::stream;
use reqwest_websocket::Message;
use serde_json::{json, Value};
use tokio::time::timeout;

use super::{Error, Handler};

#[tokio::test]
async fn test_handler() {
    let mut handler = Handler::<Value>::new();

    // get receiver for request 1234 and 5678
    let receiver_1234 = handler.request(1234);
    let receiver_5678 = handler.request(5678);

    // before handling incoming
    let messages = vec![
        Message::Text(
            json!({
                // no body in response
                "request_id": "1234",
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

    // iter stream
    let stream = stream::iter(messages.iter().map(|msg| Ok::<Message, Error>(msg.clone())));

    let handling_result = timeout(Duration::from_secs(1), handler.handle(stream)).await;

    let receiver_1234_result = timeout(Duration::from_secs(1), receiver_1234).await;
    let receiver_5678_result = timeout(Duration::from_secs(1), receiver_5678).await;

    println!("{handling_result:?}");
    println!("{receiver_1234_result:?}");
    println!("{receiver_5678_result:?}");

    assert!(handling_result.is_ok());
    assert!(handling_result.unwrap().is_ok());

    assert!(receiver_1234_result.is_ok());
    assert!(receiver_1234_result.as_ref().unwrap().is_ok());
    assert!(receiver_1234_result.unwrap().unwrap().is_none());

    assert!(receiver_5678_result.is_ok());
    assert!(receiver_5678_result.as_ref().unwrap().is_ok());

    let body_5678 = receiver_5678_result.unwrap().unwrap();
    assert!(body_5678.is_some());
    assert_eq!(
        body_5678.unwrap(),
        json!({
            "a": true,
            "b": 5,
        })
    );
}
