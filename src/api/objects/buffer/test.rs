use std::collections::BTreeMap;

use serde_json::json;

use super::{Buffer, BufferType};

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({
            "id": 1709932823238637i64,
            "name": "core.weechat",
            "short_name": "weechat",
            "number": 1,
            "type": "formatted",
            "hidden": false,
            "title": "WeeChat 4.2.0-dev (C) 2003-2023 - https://weechat.org/",
            "modes": "",
            "input_prompt": "",
            "input": "",
            "input_position": 0,
            "input_multiline": false,
            "nicklist": false,
            "nicklist_case_sensitive": false,
            "nicklist_display_groups": true,
            "time_displayed": true,
            "local_variables": {
                "plugin": "core",
                "name": "weechat"
            },
            "keys": []
    });

    // build the rust object
    let obj = Buffer {
        id: 1709932823238637,
        name: "core.weechat".into(),
        short_name: "weechat".into(),
        number: 1,
        buffer_type: BufferType::Formatted,
        hidden: false,
        title: "WeeChat 4.2.0-dev (C) 2003-2023 - https://weechat.org/".into(),
        modes: "".into(),
        input_prompt: "".into(),
        input: "".into(),
        input_position: 0,
        input_multiline: false,
        nicklist: false,
        nicklist_case_sensitive: false,
        nicklist_display_groups: true,
        time_displayed: true,
        local_variables: BTreeMap::from([
            ("plugin".into(), "core".into()),
            ("name".into(), "weechat".into()),
        ]),
        keys: vec![],
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst: Buffer = serde_json::from_value(target).unwrap();
    assert_eq!(&obj, &rst);
}
