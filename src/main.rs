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
    let deserialized_response = polygon.request_trade_data(query_params).await?;
    //let candles_for_day = get_candles_for_trading_day(30, deserialized_response);
    //println!("{:?}", candles_for_day);
    Ok(())
}
