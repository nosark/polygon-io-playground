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
#[derive(Serialize, Deserialize, Debug)]
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
        close: trades[trades.len() - 1 as usize].price,
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

#[allow(dead_code)]
pub async fn request_trade_data(
    client: &reqwest::Client,
    query_params: QueryParams<'_>,
    api_key: &String,
) -> Result<PolygonResponse, reqwest::Error> {
    let full_url = querify_paramters(query_params, api_key);
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

#[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_candle_from_trades() {
        println!("Starting candle test...");        
        let sample_trades: Vec<Trade> = vec![ 
            Trade { conditions: vec![1], exchange: 1, id: Some(String::from("123")), participant_timestamp: 9237019287301928370, price: Decimal::from(1), size: Decimal::from(1)},
            Trade { conditions: vec![1], exchange: 1, id: Some(String::from("124")), participant_timestamp: 9237019287301928380, price: Decimal::from(2), size: Decimal::from(1)},
            Trade { conditions: vec![1], exchange: 1, id: Some(String::from("125")), participant_timestamp: 9237019287301928390, price: Decimal::from(3), size: Decimal::from(1)}
        ];
        let test_candle = create_candle_from_trades(sample_trades);
        println!("{:?}", test_candle);

        assert_eq!(test_candle.open, Decimal::from(1));
        assert_eq!(test_candle.close, Decimal::from(3));
        assert_eq!(test_candle.low, Decimal::from(1));
        assert_eq!(test_candle.high, Decimal::from(3));
    }

    #[actix_rt::test]
    async fn test_get_trades_for_trading_window() -> Result<(), reqwest::Error> { 
        dotenv::from_filename("config.env").ok();
        let client = reqwest::Client::new();

        let polygon_api_key = std::env::var("POLYGON_API_KEY")
            .expect("Something went wrong while parsing the key from config file!");
        let query_params = QueryParams {
            base_url: "https://api.polygon.io/v3/trades/",
            coin_type: "X:BTC-USD",
            timestamp: "timestamp=2021-09-03",
            order: "",
            limit: "limit=50000",
            sort: "",
        };

        let res: PolygonResponse = request_trade_data(&client, query_params, &polygon_api_key)
            .await?;
        let trades = get_trades_for_trading_window(30, res);
        assert_eq!(trades.len(), 125); 
        Ok(())
    }
}
