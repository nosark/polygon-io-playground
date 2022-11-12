use crate::crypto_data::Crypto;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use std::ops::Index;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams<'a> {
    /// url for polygon feature endpoint
    pub base_url: &'a str,
    /// coin to currency conversion tyoe that you'd like data for
    pub coin_type: &'a str,
    /// date timestamp YYYY-MM-DD
    pub timestamp: &'a str,
    /// order the data in ascending (asc) or descending (desc)
    pub order: &'a str,
    /// the number of results per paginated response
    pub limit: &'a str,
    /// sort the results by X
    pub sort: &'a str,
}

impl<'a> Index<usize> for QueryParams<'a> {
    type Output = str;
    fn index(&self, index: usize) -> &'a str {
        match index {
            0 => self.base_url,
            1 => self.coin_type,
            2 => self.timestamp,
            3 => self.order,
            4 => self.limit,
            5 => self.sort,
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
    pub async fn get(&self, query_params: QueryParams<'_>) -> Result<Response, reqwest::Error> {
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

    pub fn querify_paramters(&self, params: QueryParams<'_>) -> String {
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
