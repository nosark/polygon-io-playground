extern crate dotenv;
use reqwest::Client;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Trade {
    conditions: Vec<i64>,
    exchange: i64,
    id: Option<String>,
    participant_timestamp: u64,
    price: Decimal,
    size: Decimal,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PolygonResponse {
    results: Vec<Trade>,
    status: String,
    request_id: String,
    pub next_url: Option<String>,
    previous_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Candle {
    open: Decimal,
    close: Decimal,
    low: Decimal,
    high: Decimal,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams<'a> {
    pub base_url: &'a str,
    pub coin_type: &'a str,
    pub timestamp: &'a str,
    pub order: &'a str,
    pub limit: &'a str,
    pub sort: &'a str,
}

impl<'a> Index<usize> for QueryParams<'a> {
    type Output = str;
    fn index(&self, index: usize) -> &'a str {
        match index {
            0 => &self.base_url,
            1 => &self.coin_type,
            2 => &self.timestamp,
            3 => &self.order,
            4 => &self.limit,
            5 => &self.sort,
            n => panic!("Invalid QueryParams index: {}", n),
        }
    }
}

pub fn create_candle_from_trades(trades: Vec<Trade>) -> Candle {
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
pub fn get_trades_for_trading_window(
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

pub fn get_candles_for_trading_day() -> Vec<Candle> {
    unimplemented!()
}

fn querify_paramters(params: QueryParams<'_>, api_key: &String) -> String {
    let mut full_url = String::from("");
    for i in 0..5 {
        if i == 2 {
            full_url.push_str("?");
        }

        full_url.push_str(&params[i]);

        if i > 1 && params[i].len() > 0 {
            full_url.push_str("&");
        }
    }

    full_url.push_str("apiKey=");
    full_url.push_str(api_key);

    full_url
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
pub async fn request_trade_data(
    client: &reqwest::Client,
    query_params: QueryParams<'_>,
    api_key: &String,
) -> Result<PolygonResponse, reqwest::Error> {
    let full_url = querify_paramters(query_params, api_key);
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
