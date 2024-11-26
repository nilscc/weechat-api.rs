use serde_json::json;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::Line;

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!(    {
        "id": 0,
        "y": -1,
        "date": "2023-12-05T19:46:03.847625Z",
        "date_printed": "2023-12-05T19:46:03.847625Z",
        "displayed": true,
        "highlight": false,
        "notify_level": 0,
        "prefix": "-->",
        "message": "alice (~alice@example.com) has joined #test",
        "tags": [
            "irc_join",
            "irc_tag_account=alice",
            "irc_tag_time=2023-12-05T19:46:03.847Z",
            "nick_alice",
            "host_~alice@example.com",
            "log4"
        ]
    });

    // parse date time
    let date_time = OffsetDateTime::parse("2023-12-05T19:46:03.847625Z", &Rfc3339).unwrap();

    // build the rust object
    let obj = Line {
        id: 0i64,
        y: -1,
        date: date_time,
        date_printed: date_time,
        displayed: true,
        highlight: false,
        notify_level: 0,
        prefix: "-->".into(),
        message: "alice (~alice@example.com) has joined #test".into(),
        tags: vec![
            "irc_join".into(),
            "irc_tag_account=alice".into(),
            "irc_tag_time=2023-12-05T19:46:03.847Z".into(),
            "nick_alice".into(),
            "host_~alice@example.com".into(),
            "log4".into(),
        ],
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Line>(target).unwrap();
    assert_eq!(&obj, &rst);
}
