use super::Credentials;

pub fn credentials_from_dotenv() -> Credentials {
    // make sure .env file is available
    let _ = dotenv::dotenv().expect(".env file not found, see README.md");

    // get credentials
    let host = dotenv::var("WEECHAT_DOMAIN").expect("WEECHAT_DOMAIN missing from .env file");
    let port = dotenv::var("WEECHAT_PORT")
        .expect("WEECHAT_PORT missing from .env file")
        .parse::<i32>()
        .unwrap();
    let password =
        dotenv::var("WEECHAT_PASSWORD").expect("WEECHAT_PASSWORD missing from .env file");

    Credentials::new(host, port, password)
}
