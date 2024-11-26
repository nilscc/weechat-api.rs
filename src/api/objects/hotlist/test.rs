use serde_json::json;

use super::Hotlist;

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({
        "priority": 0,
        "date": "2024-03-17T16:38:51.572834Z",
        "buffer_id": 1710693531508204i64,
        "count": [
            44,
            0,
            0,
            0
        ]
    });

    // build the rust object
    let obj = Hotlist {
        priority: 0,
        date: "2024-03-17T16:38:51.572834Z".into(),
        buffer_id: 1710693531508204i64,
        count: vec![44, 0, 0, 0],
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Hotlist>(target).unwrap();
    assert_eq!(&obj, &rst);
}
