use base64::{prelude::BASE64_STANDARD, Engine};
use reqwest::Url;
use serde_json::Value;

#[tokio::test]
async fn test() {
    let _env = dotenv::dotenv().expect(".env file not found, see README.md");

    let host = dotenv::var("WEECHAT_DOMAIN").expect("WEECHAT_DOMAIN missing from .env file");
    let port = dotenv::var("WEECHAT_PORT")
        .expect("WEECHAT_PORT missing from .env file")
        .parse::<i32>()
        .unwrap();
    let passwd = dotenv::var("WEECHAT_PASSWORD").expect("WEECHAT_PASSWORD missing from .env file");

    let client = reqwest::Client::builder().https_only(true).build().unwrap();

    let url = Url::parse(&format!("https://{host}:{port}")).unwrap();

    // fetch result
    let res = {
        // set correct path for request
        let mut requ = url.clone();
        requ.set_path("/api/version".into());

        // perform request
        client
            .get(requ)
            .header(
                "Authorization",
                format!(
                    "Basic {}",
                    BASE64_STANDARD.encode(format!("plain:asd{passwd}"))
                ),
            )
            .send()
            .await
            .unwrap()
    };

    let status = res.status();

    println!("{res:?}");
    println!("{:?}", res.json::<Value>().await.unwrap());

    assert_eq!(200, status);
}
