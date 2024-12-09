use std::time::Duration;

use serde_json::json;

use crate::api::client::test::credentials_from_dotenv;

use super::WebsocketClient;

use tokio::{spawn, sync::oneshot, time::sleep};

//#[test]
//fn test_to_request() {
//    {
//        let req = WebsocketClient::to_request::<()>(0, "GET /nothing", None);
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "GET /nothing",
//            })
//        );
//    }
//    {
//        let req = WebsocketClient::to_request::<()>(0, "GET /nothing", Some(()));
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "GET /nothing",
//                "body": None::<()>, // null
//            })
//        );
//    }
//    {
//        let req =
//            WebsocketClient::to_request::<String>(0, "GET /nothing", Some("hello world".into()));
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "GET /nothing",
//                "body": "hello world",
//            })
//        );
//    }
//    {
//        let req =
//            WebsocketClient::to_request::<Value>(0, "GET /nothing", Some(json!("hello world")));
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "GET /nothing",
//                "body": "hello world",
//            })
//        );
//    }
//    {
//        let req = WebsocketClient::to_request::<Value>(0, "GET /nothing", Some(json!([1, 2, 3])));
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "GET /nothing",
//                "body": [1,2,3],
//            })
//        );
//    }
//    {
//        let input = Input {
//            buffer_ref: BufferRef::BufferName("test/test".into()),
//            command: "hello world".into(),
//        };
//        let req = WebsocketClient::to_request::<Input>(0, "POST /api/input", Some(input));
//        assert!(req.is_object());
//        assert_eq!(
//            req,
//            json!({
//                "request_id": 0,
//                "request": "POST /api/input",
//                "body": {
//                    "buffer": "test/test",
//                    "command": "hello world",
//                },
//            })
//        );
//    }
//}

#[tokio::test]
async fn test_request_response_future() {
    // create channel
    let (s, r) = oneshot::channel::<()>();

    // sleep for 1s before sending
    let join = spawn(async move {
        sleep(Duration::from_secs(1)).await;
        assert!(s.send(()).is_ok());
    });

    // wait for receiver
    assert!(r.await.is_ok());

    // join child task
    assert!(join.await.is_ok());
}

#[tokio::test]
async fn test_connect() {
    let credentials = credentials_from_dotenv();
    let mut client = WebsocketClient::new(credentials, None);

    let res = client.connect().await;
    println!("{res:?}");
    assert!(res.is_ok());
}
