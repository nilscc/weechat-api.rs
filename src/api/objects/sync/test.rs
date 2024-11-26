use serde_json::json;

use super::{Colors, Sync};

#[test]
fn test_json_convertion_1() {
    // the main JSON target we compare everything against
    let target = json!({
        "nicks": false
    });

    // build the rust object
    let obj = Sync {
        sync: None,
        nicks: Some(false),
        input: None,
        colors: None,
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Sync>(target).unwrap();
    assert_eq!(&obj, &rst);
}

#[test]
fn test_json_convertion_2() {
    // the main JSON target we compare everything against
    let target = json!({
        "sync": true,
        "nicks": true,
        "input": false,
        "colors": "weechat",
    });

    // build the rust object
    let obj = Sync {
        sync: Some(true),
        nicks: Some(true),
        input: Some(false),
        colors: Some(Colors::Weechat),
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Sync>(target).unwrap();
    assert_eq!(&obj, &rst);
}
