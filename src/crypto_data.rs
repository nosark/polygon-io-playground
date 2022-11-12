extern crate dotenv;
use crate::polygon_client::{Polygon, QueryParams};
use async_trait::async_trait;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
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
pub struct Trades {
    results: Vec<Trade>,
    status: String,
    request_id: String,
    pub next_url: Option<String>,
    previous_url: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Last {
    conditions: Vec<i64>,
    exchange: i64,
    price: Decimal,
    size: Decimal,
    timestamp: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct LastTradeCryptoPair {
    last: Last,
    request_id: Option<String>,
    status: Option<String>,
    symbol: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Candle {
    open: Decimal,
    close: Decimal,
    low: Decimal,
    high: Decimal,
}

#[async_trait]
pub trait Crypto {
    async fn last_trade_for_crypto_pair(
        &self,
        client: &Polygon,
        from: &str,
        to: &str,
    ) -> Result<LastTradeCryptoPair, reqwest::Error> {
        let base_url = format!("https://api.polygon.io/v1/last/crypto/{}/{}", from, to);
        let query_params = QueryParams {
            base_url: base_url.as_str(),
            coin_type: "",
            timestamp: "",
            order: "",
            limit: "",
            sort: "",
        };

        let result = client.get(query_params).await;

        match result {
            Ok(result) => {
                let deserialized_res = result.json::<LastTradeCryptoPair>().await?;
                Ok(deserialized_res)
            }
            Err(err) => {
                println!("Error: {}", err);
                Err(err)
            }
        }
    }

    fn create_candle_from_trades(&self, trades: &Vec<Trade>) -> Candle {
        let mut high = Decimal::MIN;
        let mut low = Decimal::MAX;

        for trade in trades {
            if trade.price > high {
                high = trade.price;
            }

            if trade.price < low {
                low = trade.price;
            }
        }

        Candle {
            open: trades[0].price,
            close: trades[trades.len() - 1].price,
            low,
            high,
        }
    }

    fn get_candles_for_trading_day(
        &self,
        num_seconds: u64,
        trades: &Trades,
    ) -> (Vec<Trade>, Vec<Candle>) {
        let mut candles_for_day = Vec::<Candle>::new();
        let trades_for_day = &trades.results;
        let mut trading_window = Vec::<Trade>::new();
        let mut initial_time_stamp = trades_for_day[0].participant_timestamp;

        //now iterate through all results grabbing for num seconds
        for i in 0..trades_for_day.len() {
            let time_elapsed =
                Duration::from_nanos(initial_time_stamp - trades_for_day[i].participant_timestamp);
            if time_elapsed.as_secs() <= num_seconds {
                trading_window.push(trades_for_day[i].clone());
            } else {
                //create candle from trading_window and reset time stamp for new time window
                let current_candle = self.create_candle_from_trades(&trading_window);
                candles_for_day.push(current_candle);

                initial_time_stamp = trades_for_day[i].participant_timestamp; // reset initial_time_stamp
                trading_window.clear(); // clear trade buffer
            }
        }

        (trading_window, candles_for_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::polygon_client::Polygon;

    #[test]
    fn test_create_candle_from_trades() {
        println!("Starting candle test...");
        let mut sample_trades = Vec::<Trade>::new();
        let trade_one = Trade {
            conditions: vec![1],
            exchange: 1,
            id: Some(String::from("123")),
            participant_timestamp: 9237019287301928370,
            price: Decimal::from(1),
            size: Decimal::from(1),
        };
        let trade_two = Trade {
            conditions: vec![1],
            exchange: 1,
            id: Some(String::from("124")),
            participant_timestamp: 9237019287301928380,
            price: Decimal::from(2),
            size: Decimal::from(1),
        };
        let trade_three = Trade {
            conditions: vec![1],
            exchange: 1,
            id: Some(String::from("125")),
            participant_timestamp: 9237019287301928390,
            price: Decimal::from(3),
            size: Decimal::from(1),
        };
        sample_trades.push(trade_one);
        sample_trades.push(trade_two);
        sample_trades.push(trade_three);

        let polygon = Polygon::new(Some(String::from("fake_api_key_no_reqeust_here")));
        let test_candle = polygon.create_candle_from_trades(&sample_trades);
        println!("{:?}", test_candle);

        assert_eq!(test_candle.open, Decimal::from(1));
        assert_eq!(test_candle.close, Decimal::from(3));
        assert_eq!(test_candle.low, Decimal::from(1));
        assert_eq!(test_candle.high, Decimal::from(3));
    }
}
