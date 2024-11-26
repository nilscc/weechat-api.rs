use serde_json::json;

use super::Version;

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({
        "weechat_version": "4.2.0-dev",
        "weechat_version_git": "v4.1.0-143-g0b1cda1c4",
        "weechat_version_number": 67239936,
        "relay_api_version": "0.0.1",
        "relay_api_version_number": 1
    });

    // build the rust object
    let obj = Version {
        weechat_version: "4.2.0-dev".into(),
        weechat_version_git: "v4.1.0-143-g0b1cda1c4".into(),
        weechat_version_number: 67239936,
        relay_api_version: "0.0.1".into(),
        relay_api_version_number: 1,
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst: Version = serde_json::from_value(target).unwrap();
    assert_eq!(&obj, &rst);
}
