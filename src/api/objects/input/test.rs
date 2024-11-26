use serde_json::json;

use super::{BufferRef, Input};

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({"buffer": "irc.libera.#weechat", "command": "hello!"});

    // build the rust object
    let obj = Input {
        buffer_ref: BufferRef::BufferName("irc.libera.#weechat".into()),
        command: "hello!".into(),
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Input>(target).unwrap();
    assert_eq!(&obj, &rst);
}
