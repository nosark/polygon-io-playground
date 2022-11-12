use std::collections::VecDeque;

use polygonio_rs::crypto_data::{Candle, Crypto, Trade};
use polygonio_rs::polygon_client::{Polygon, QueryParams};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("config.env").ok();
    let polygon_api_key = std::env::var("POLYGON_API_KEY")
        .expect("Something went wrong while parsing the key from config file!");
    let polygon = Polygon::new(Some(polygon_api_key));
    let query_params = QueryParams {
        base_url: "https://api.polygon.io/v3/trades/",
        coin_type: "X:BTC-USD",
        timestamp: "timestamp=2021-09-03",
        order: "",
        limit: "limit=50000",
        sort: "",
    };

    let last_trade = polygon
        .last_trade_for_crypto_pair(&polygon, "BTC", "USD")
        .await?;
    println!("{:#?}", last_trade);
    Ok(())
}
