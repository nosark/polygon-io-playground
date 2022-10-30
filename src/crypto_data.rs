extern crate dotenv;
use reqwest::Client;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Aggregator {
    candles: Vec<Candle>,
}

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

pub trait Crypto {
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
            close: trades[trades.len() - 1 as usize].price,
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
            let time_elapsed =
                Duration::from_nanos(intial_time_stamp - trade.participant_timestamp);
            if time_elapsed.as_secs() <= trading_window {
                trades_in_window.push(trade);
            }
        }

        trades_in_window
    }

    fn get_candles_for_trading_day(num_seconds: i32, res: PolygonResponse) -> Vec<Candle> {
        let mut candles_for_day = Vec::<Candle>::new();
        let trades_for_day = res.results;

        //now iterate through all results grabbing for num seconds
        for _i in 0..trades_for_day.len() {
            // process trades for N sec window
            // stop pointer
            // create candle
            //continue loop until end
            // process next page with recursion or iteration...?
        }
        candles_for_day
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polygon_client::{Polygon, QueryParams};

    #[test]
    fn test_create_candle_from_trades() {
        println!("Starting candle test...");
        let sample_trades: Vec<Trade> = vec![
            Trade {
                conditions: vec![1],
                exchange: 1,
                id: Some(String::from("123")),
                participant_timestamp: 9237019287301928370,
                price: Decimal::from(1),
                size: Decimal::from(1),
            },
            Trade {
                conditions: vec![1],
                exchange: 1,
                id: Some(String::from("124")),
                participant_timestamp: 9237019287301928380,
                price: Decimal::from(2),
                size: Decimal::from(1),
            },
            Trade {
                conditions: vec![1],
                exchange: 1,
                id: Some(String::from("125")),
                participant_timestamp: 9237019287301928390,
                price: Decimal::from(3),
                size: Decimal::from(1),
            },
        ];
        let test_candle = Polygon::create_candle_from_trades(sample_trades);
        println!("{:?}", test_candle);

        assert_eq!(test_candle.open, Decimal::from(1));
        assert_eq!(test_candle.close, Decimal::from(3));
        assert_eq!(test_candle.low, Decimal::from(1));
        assert_eq!(test_candle.high, Decimal::from(3));
    }

    #[actix_rt::test]
    async fn test_get_trades_for_trading_window() -> Result<(), reqwest::Error> {
        dotenv::from_filename("config.env").ok();

        let polygon_api_key = std::env::var("POLYGON_API_KEY")
            .expect("dotenv failed to load config variable API_KEY");
        let polygon = Polygon::new(Some(polygon_api_key));
        let query_params = QueryParams {
            base_url: "https://api.polygon.io/v3/trades/",
            coin_type: "X:BTC-USD",
            timestamp: "timestamp=2021-09-03",
            order: "",
            limit: "limit=50000",
            sort: "",
        };

        let res: PolygonResponse = polygon.request_trade_data(query_params).await?;
        let trades = Polygon::get_trades_for_trading_window(30, res);
        assert_eq!(trades.len(), 125);
        Ok(())
    }
}
