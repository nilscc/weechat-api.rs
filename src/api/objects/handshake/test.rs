use serde_json::json;

use super::Handshake;

#[test]
fn test_json_convertion() {
    // the main JSON target we compare everything against
    let target = json!({
        "password_hash_algo": "sha512",
        "password_hash_iterations": 100000,
        "totp": false
    });

    // build the rust object
    let obj = Handshake {
        password_hash_algo: "sha512".into(),
        password_hash_iterations: 100000,
        totp: false,
    };

    // convert rust object to value
    let jsn = serde_json::to_value(&obj).unwrap();
    assert_eq!(jsn, target);

    // convert json to rust object
    let rst = serde_json::from_value::<Handshake>(target).unwrap();
    assert_eq!(&obj, &rst);
}
