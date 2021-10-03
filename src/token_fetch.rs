use reqwest;
use serde::{Deserialize, Serialize};
use urlencoding::encode;

async fn client_key() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let body = reqwest::get("https://api.sys32.dev/v2/key")
        .await?
        .text()
        .await?;

    Ok(body)
}

#[derive(Deserialize, Serialize, Debug)]
struct MatchmakerToken {
    token: String,
    cfid: u32,
    sid: u32,
}

async fn matchmaker_token(
    client_key: &String,
) -> Result<MatchmakerToken, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .get("https://matchmaker.krunker.io/generate-token")
        .header("client-key", client_key)
        .send()
        .await?
        .json::<MatchmakerToken>()
        .await?;

    Ok(body)
}

type HashReturn = Vec<u32>;

async fn hask_token(
    token: &MatchmakerToken,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .post("https://api.sys32.dev/v2/token")
        .header("Content-Type", "application/json")
        .json(&token)
        .send()
        .await?
        .json::<HashReturn>()
        .await?;

    let b = body
        .iter()
        .map(|e| std::primitive::char::from_u32(*e).unwrap())
        .collect::<String>();

    Ok(b)
}

pub async fn token_arg() -> String {
    println!("fetching client key...");
    let client_key = client_key().await.unwrap();
    println!("fetching token key...");
    let token = matchmaker_token(&client_key).await.unwrap();
    println!("hashing key...");
    hask_token(&token).await.unwrap()
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WebsocketReturnData {
    pub clientId: String,
    pub gameId: String,
    pub host: String,
    pub port: u32,
    pub changeReason: Option<String>,
}

pub async fn get_websocket_info(
    token: &String,
) -> Result<WebsocketReturnData, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();

    let body = client
        .get(format!("https://matchmaker.krunker.io/seek-game?hostname=krunker.io&region=us-nj&autoChangeGame=false&validationToken={}", encode(token)))
        .header("Origin", "https://krunker.io")
        // .header("region", "us-nj")
        // .header("autoChangeGame", "false")
        // .header("validationToken", token)
        // .header("dataQuery", "%7B%22v%22%3A%22M9UCk%22%7D")
        .header("accept-language","en-US,en;q=0.6;")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/85.0.4183.0 Safari/537.36 Edg/85.0.564.0")
        .send()
        .await?
        // .text()
        .json::<WebsocketReturnData>()
        .await?;
    Ok(body)
}
