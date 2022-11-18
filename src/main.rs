
use polygonio_rs::crypto_data::Crypto;
use polygonio_rs::polygon_client::Polygon;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("config.env").ok();
    let polygon_api_key = std::env::var("POLYGON_API_KEY")
        .expect("Something went wrong while parsing the key from config file!");
    let polygon = Polygon::new(Some(polygon_api_key));

    let last_trade = polygon
        .last_trade_for_crypto_pair(&polygon, "BTC", "USD")
        .await?;
    println!("{:#?}", last_trade);

    let previous_close = polygon
        .previous_close(&polygon, String::from("X:BTCUSD"), false)
        .await?;
    println!("{:#?}", previous_close);
    Ok(())
}
