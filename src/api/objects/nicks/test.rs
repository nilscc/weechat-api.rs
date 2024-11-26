use serde_json::json;

use super::{Nick, NickGroup};

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({
        "id": 0i64,
        "parent_group_id": -1i64,
        "name": "root",
        "color_name": "",
        "color": "",
        "visible": false,
        "groups": [
            {
                "id": 1709932823649181i64,
                "parent_group_id": 0,
                "name": "000|o",
                "color_name": "weechat.color.nicklist_group",
                "color": "\\u001b[32m",
                "visible": true,
                "groups": [],
                "nicks": [
                    {
                        "id": 1709932823649184i64,
                        "parent_group_id": 1709932823649181i64,
                        "prefix": "@",
                        "prefix_color_name": "lightgreen",
                        "prefix_color": "\\u001b[92m",
                        "name": "alice",
                        "color_name": "bar_fg",
                        "color": "",
                        "visible": true
                    }
                ]
            }
        ],
        "nicks": [],
    });

    // build the rust object
    let obj = NickGroup {
        id: 0i64,
        parent_group_id: -1i64,
        name: "root".into(),
        color_name: "".into(),
        color: "".into(),
        visible: false,
        groups: vec![NickGroup {
            id: 1709932823649181i64,
            parent_group_id: 0,
            name: "000|o".into(),
            color_name: "weechat.color.nicklist_group".into(),
            color: "\\u001b[32m".into(),
            visible: true,
            groups: vec![],
            nicks: vec![Nick {
                id: 1709932823649184i64,
                parent_group_id: 1709932823649181,
                prefix: "@".into(),
                prefix_color_name: "lightgreen".into(),
                prefix_color: "\\u001b[92m".into(),
                name: "alice".into(),
                color_name: "bar_fg".into(),
                color: "".into(),
                visible: true,
            }],
        }],
        nicks: vec![],
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<NickGroup>(target).unwrap();
    assert_eq!(&obj, &rst);
}
