use std::path::PathBuf;



#[tokio::test]
async fn test() {
    let _env = dotenv::dotenv().expect(".env file not found, see README.md");

    let _client = reqwest::Client::new();

    // client.get("/api")
}