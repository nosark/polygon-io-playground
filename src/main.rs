extern crate dotenv;

use reqwest::Client;
use serde_json;
use serde::{Serialize, Deserialize};
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;


#[derive(Deserialize, Debug)]
struct Trade {
    conditions: Vec<i64>,
    exchange: i64,
    id: String,
    participant_timestamp: i64,
    price: Decimal,
    size: Decimal
}


#[derive(Deserialize, Debug)]
struct Response {
    results: Vec<Trade>,
    status: String,
    request_id: String,
    next_url: String
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("../config.env").ok();
    let reqwest_client = Client::new();
    let coin_type = "X:BTC-USD";
    let base_url: String = "https://api.polygon.io/v3/trades/".to_owned();
    let polygon_api_key = std::env::var("POLYGON_API_KEY").expect("dotenv broke again... WTF");
    let complete_url = format!("{}{}?apiKey={}", base_url, coin_type, polygon_api_key);
    let res = reqwest_client.get(complete_url.clone())
        .send()
        .await?
        .json::<Response>()
        .await?;

    println!("complete url {:?}" , complete_url.clone());
    println!("response: {:?}", res);
    Ok(())
}
