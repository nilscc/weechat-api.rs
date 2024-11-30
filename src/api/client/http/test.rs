use super::HttpClient;

use crate::api::{
    client::test::credentials_from_dotenv,
    objects::version::Version,
};

#[tokio::test]
async fn test_http_fetch_version() {
    let credentials = credentials_from_dotenv();
    let mut client = HttpClient::new(credentials, None);

    // perform simple fetch
    let version = client.get::<Version>("/api/version").await;

    println!("{version:?}");
    assert!(version.is_ok());
}
