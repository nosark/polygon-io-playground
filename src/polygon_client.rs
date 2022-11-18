use crate::crypto_data::Crypto;
use actix_web::{cookie::time::Time, web::Query};
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::ops::Index;
use std::string::String;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Route {
    AggregatesBars,
    GroupedDaily,
    DailyOpenClose,
    PrevClose,
    Trades,
    LastTradeForPairCrypto,
    SnapShots,
    TechnicalIndicators,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Timespan {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct QueryParams<'a> {
    /// url for polygon feature endpoint
    pub base_url: &'a str,
    /// coin to currency conversion tyoe that you'd like data for
    pub crypto_ticker: &'a str,
    /// The size of the timespan multiplier,
    pub multiplier: &'a str,
    /// the size of the time window
    pub timespan: Timespan,
    /// the start of the time window formatted YYYY-MM-DD
    pub from: &'a str,
    /// the end of the time window formatted YYYY-MM-DD
    pub to: &'a str,
    /// beginning date of grouped daily bars window formatted YYYY-MM-DD
    pub date: &'a str,
    /// date timestamp YYYY-MM-DD
    pub timestamp: &'a str,
    /// whether or not the results are adjusted for splits
    pub adjusted: &'a str,
    /// order the data in ascending (asc) or descending (desc)
    pub order: &'a str,
    /// the number of results per paginated response
    pub limit: &'a str,
    /// sort the results by X
    pub sort: &'a str,
    /// from currency symbol for price conversion 
    pub from_symbol: &'a str,
    /// to currency symbol to be converted
    pub to_symbol: &'a str,
}

impl<'a> QueryParams<'a> {
    pub fn new() -> Self {
        QueryParams {
            base_url: "",
            crypto_ticker: "", 
            multiplier: "1",
            timespan: Timespan::Day, 
            from: "", 
            to: "",
            date: "", 
            timestamp: "", 
            adjusted: "", 
            order: "", 
            limit: "", 
            sort: "", 
            from_symbol: "", 
            to_symbol: "", 
        }
    }

    pub fn add_base_url(mut self, url: &'a str) -> QueryParams {
        self.base_url = url;
        self
    }

    pub fn add_crypto_ticker(mut self, ticker: &'a str) -> QueryParams {
        self.crypto_ticker = ticker;
        self
    }

    pub fn add_multiplier(mut self, mutliplier: &'a str) -> Self {
        self.multiplier = mutliplier;
        self
    }

    pub fn add_timespan(mut self, timespan: Timespan) -> Self {
        self.timespan = timespan;
        self
    }

    pub fn add_from(mut self, from_date: &'a str) -> Self {
        self.from = from_date;
        self
    }

    pub fn add_to(mut self, to_date: &'a str) -> Self {
        self.to = to_date;
        self
    }

    pub fn add_date(mut self, date: &'a str) -> Self {
        self.date = date;
        self
    }

    pub fn add_timestamp(mut self, timestamp: &'a str) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn add_adjusted(mut self, adj: &'a str) -> Self {
        self.adjusted = adj;
        self
    }

    pub fn add_order(mut self, ordering: &'a str) -> Self {
        self.order = ordering;
        self
    }

    pub fn add_limit(mut self, results_per_page: &'a str) -> Self {
        self.limit = results_per_page;
        self
    }
    
    pub fn add_sort(mut self, sort_method: &'a str) -> Self {
        self.sort = sort_method;
        self
    }
    
    pub fn add_from_symbol(mut self, symbol: &'a str) -> Self {
        self.from_symbol = symbol;
        self
    }

    pub fn add_to_symbol(mut self, symbol: &'a str) -> Self {
        self.to_symbol = symbol;
        self
    }
}

impl Index<usize> for QueryParams<'_> {
    type Output = str;

    fn index(&self, index: usize) -> &str {
        match index {
            0 => self.base_url,
            1 => self.crypto_ticker,
            2 => self.multiplier,
            3 => {
                match self.timespan {
                    Timespan::Day =>  "day",
                    Timespan::Hour => "hour",
                    Timespan::Minute => "minute",
                    Timespan::Week => "week",
                    Timespan::Month => "month",
                    Timespan::Quarter => "quarter",
                    Timespan::Year => "year",
                }
            },
            4 => self.from,
            5 => self.to,
            6 => self.date,
            7 => self.timestamp,
            8 => self.adjusted,
            9 => self.order,
            10 => self.limit,
            11 => self.sort,
            12 => self.from_symbol,
            13 => self.to_symbol,
            n => panic!("Invalid QueryParams index: {}", n),
        }
    }
}

#[allow(dead_code)]
pub struct Polygon {
    polygon_api_key: Option<String>,
    client: reqwest::Client,
}

impl Polygon {
    pub fn new(polygon_api_key: Option<String>) -> Self {
        Polygon {
            polygon_api_key,
            client: reqwest::Client::new(),
        }
    }

    #[allow(dead_code)]
    pub async fn get(&self, query_params: &QueryParams<'_>) -> Result<Response, reqwest::Error> {
        let full_url = Self::querify_paramters(self, query_params);
        let res = self.client.get(full_url).send().await?;

        match res.error_for_status() {
            Ok(res) => Ok(res),
            Err(err) => {
                println!("Error: {}", err);
                Err(err)
            }
        }
    }

    pub fn querify_paramters(&self, params: &QueryParams<'_>) -> String {
        let mut full_url = String::from("");
        for i in 0..5 {
            if i == 2 {
                full_url.push('?');
            }

            full_url.push_str(&params[i]);

            if i > 1 && params[i].len() > 0 {
                full_url.push('&');
            }
        }

        full_url.push_str("apiKey=");
        full_url.push_str(&self.polygon_api_key.clone().unwrap());

        full_url
    }
}

impl Crypto for Polygon {}
