extern crate dotenv;

use reqwest::Client;
use rust_decimal::prelude::*;
use serde::Deserialize;


#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Trade {
    conditions: Vec<i64>,
    exchange: i64,
    id: String,
    participant_timestamp: i64,
    price: Decimal,
    size: Decimal,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct PolygonResponse {
    results: Vec<Trade>,
    status: String,
    request_id: String,
    next_url: Option<String>,
    previous_url: Option<String>,
}

/// This function is used to make asynchronous requests to specified Polygon.io API endpoints
/// for query variables that are not being used , fill with and empty "" for the formatter
///
/// an example query for bitcoin would appear as follows:
///
///     request_trade_data(
///         &some_client,
///         "https://api.polygon.io/v3/trades/",
///         "X:BTC-USD",
///         "",
///         "",
///         "",
///         api_key
///     )?;
///
///
///
/// This call is made using no attributes for order, limit, or sort. This is a generic
/// request. This will return either Ok(PolygonResponse) or an Err().
async fn request_trade_data(
    client: &reqwest::Client,
    base_url: &str,
    coin_type: &str,
    order: &str,
    limit: &str,
    sort: &str,
    api_key: String,
) -> Result<PolygonResponse, reqwest::Error> {
    let full_url = format!(
        "{}{}{}{}{}?apiKey={}",
        base_url, coin_type, order, limit, sort, api_key
    );

    // after formatting string make request
    // if request is successful Deserialize data and return the full response.
    let res = client.get(full_url).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let deserialized_data = res.json::<PolygonResponse>().await?;
            return Ok(deserialized_data);
        }
        Err(err) => {
            println!("Error: {}", err);
            return Err(err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("config.env").ok();
    let reqwest_client = Client::new();
    let coin_type = "X:BTC-USD";
    let polygon_api_key = std::env::var("POLYGON_API_KEY").expect("dotenv broke again... WTF");
    let deser_response = request_trade_data(
        &reqwest_client,
        "https://api.polygon.io/v3/trades/",
        coin_type,
        "",
        "",
        "",
        polygon_api_key,
    )
    .await?;

    println!("{:?}", deser_response.next_url);

    Ok(())
}
