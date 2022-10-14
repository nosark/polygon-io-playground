use dotenv;
use reqwest::Client;
use serde_json;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct Response<T> {
    next_url: String,
    results: Vec<T>,
    exchange: i64,
    id: String,
    participant_timestamp: i64,
    price_number: f64,
    size: f64
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("../config.env").ok();
    let reqwest_client = Client::new();
    let coin_type = "X:BTC-USD";
    let base_url: String = "https://api.polygon.io/v3/trades/".to_owned();
    let polygon_api_key = std::env::var("POLYGON_API_KEY").expect("Must load Polygon api key into environment!");
    let complete_url = format!("{}{}?apiKey={}", base_url, coin_type, polygon_api_key);
    let res = reqwest_client.get(complete_url.clone())
        .send()
        .await?;
    let body = res.text().await?;

    println!("complete url {:?}" , complete_url.clone());
    println!("response: {:?}", body);
    Ok(())
}
