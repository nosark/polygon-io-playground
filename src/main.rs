extern crate dotenv;

use reqwest::Client;
use serde::{Serialize, Deserialize};
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
struct ParsedResponse {
    results: Vec<Trade>,
    status: String,
    request_id: String,
    next_url: Option<String>,
    previous_url: Option<String>
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
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => {
            //success process data
            let parsed_res = res.json::<ParsedResponse>().await?;
            println!("response: {:?}", parsed_res.results[0]);
        },

        reqwest::StatusCode::UNAUTHORIZED => {
            // either auth token subscription expired or dotenv failed to parse
            println!("Failed to authorize! Please make sure API Key is valid!");
        },

        _ => {
            // something is broken scream fire and look for extinguisher...
            panic!("PANIC : Something is clearly broken, lets investigate!");
        },
    }


    Ok(())
}
