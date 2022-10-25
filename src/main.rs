use polygonio_rs::crypto_data::{get_trades_for_trading_window, request_trade_data, QueryParams};

use reqwest::Client;
use std::collections::VecDeque;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("config.env").ok();
    let reqwest_client = Client::new();
    let query_params = QueryParams {
        base_url: "https://api.polygon.io/v3/trades/",
        coin_type: "X:BTC-USD",
        timestamp: "timestamp=2021-09-03",
        order: "",
        limit: "limit=50000",
        sort: "",
    };
    let polygon_api_key = std::env::var("POLYGON_API_KEY")
        .expect("Something went wrong while parsing the key from config file!");
    let deserialized_response =
        request_trade_data(&reqwest_client, query_params, &polygon_api_key).await?;

    //now push next url to page_gathering_queue
    //
    let mut page_gathering_queue = VecDeque::<String>::new();
    page_gathering_queue.push_back(deserialized_response.next_url.clone().unwrap());

    let mut page_count = 0;

    let current_trading_window = get_trades_for_trading_window(30, deserialized_response);
    println!(
        "{:?} {}",
        current_trading_window,
        current_trading_window.len()
    );

    //TODO: create Aggregator and move pagination logic to crypto_data.rs
    /*while !page_gathering_queue.is_empty() {
        // make the request
        // do something with trades[todo]
        // add next url to queue
        // continue
        page_count += 1;
        let current_page_url = format!(
            "{}&apiKey={}",
            page_gathering_queue.pop_front().clone().unwrap(),
            &polygon_api_key
        );
        let current_page_res = get_next_page_data(&reqwest_client, &current_page_url).await?;

        println!(
            "page count: {} \n  next_url: {}",
            page_count, &current_page_url
        );
        // this is where we would manipulate trade data.
        page_gathering_queue.push_back(current_page_res.next_url.clone().unwrap());
    }*/

    Ok(())
}
