extern crate dotenv;
use std::collections::VecDeque;
use std::time::Duration;

use reqwest::Client;
use rust_decimal::prelude::*;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Trade {
    conditions: Vec<i64>,
    exchange: i64,
    id: String,
    participant_timestamp: u64,
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

#[allow(dead_code)]
struct Candle {
    open: Decimal,
    close: Decimal,
    low: Decimal,
    high: Decimal,
}

fn create_candle_from_trades(trades: Vec<Trade>) -> Candle {
    let mut high = Decimal::MIN;
    let mut low = Decimal::MAX;

    for trade in &trades {
        if trade.price > high {
            high = trade.price;
        }

        if trade.price < low {
            low = trade.price;
        }
    }

    Candle {
        open: trades[0 as usize].price,
        close: trades[trades.len() as usize].price,
        low,
        high,
    }
}
/// This function will return trades within an N second(s) window.
fn get_trades_for_trading_window(
    trading_window: u64,
    deserilaized_res: PolygonResponse,
) -> Vec<Trade> {
    let mut trades_in_window = Vec::<Trade>::new();
    let intial_time_stamp = deserilaized_res.results[0].participant_timestamp;
    for trade in deserilaized_res.results {
        let time_elapsed = Duration::from_nanos(intial_time_stamp - trade.participant_timestamp);
        if time_elapsed.as_secs() <= trading_window {
            trades_in_window.push(trade);
        }
    }

    trades_in_window
}

fn get_candles_for_trading_day() -> Vec<Candle> {
    unimplemented!()
}

// calc_time_elapsed: timestamp b - timestamp a

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
    timestamp: &str,
    order: &str,
    limit: &str,
    sort: &str,
    api_key: &String,
) -> Result<PolygonResponse, reqwest::Error> {
    let full_url = format!(
        "{}{}?{}{}{}apiKey={}",
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

async fn get_next_page_data(
    client: &reqwest::Client,
    page_url: &str,
) -> Result<PolygonResponse, reqwest::Error> {
    let res = client.get(page_url).send().await?;

    match res.error_for_status() {
        Ok(res) => {
            let deserialized_res = res.json::<PolygonResponse>().await?;
            return Ok(deserialized_res);
        }

        Err(err) => {
            println!("Something went wrong, Error: {}", err);
            return Err(err);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename("config.env").ok();
    let reqwest_client = Client::new();
    let coin_type = "X:BTC-USD";
    let polygon_api_key = std::env::var("POLYGON_API_KEY").expect("dotenv broke again...");
    let deserialized_response = request_trade_data(
        &reqwest_client,
        "https://api.polygon.io/v3/trades/",
        coin_type,
        "?timestamp=2021-09-03",
        "",
        "&limit=50000",
        "&sort=timestamp&",
        &polygon_api_key,
    )
    .await?;

    //now push next url to page_gathering_queue
    //
    let mut page_gathering_queue = VecDeque::<String>::new();
    page_gathering_queue.push_back(deserialized_response.next_url.clone().unwrap());

    let mut page_count = 0;

    let current_trading_window = get_trades_for_trading_window(30, deserialized_response);
    //  println!("breaks printing trades below!");
    println!(
        "{:?} {}",
        current_trading_window,
        current_trading_window.len()
    );
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
