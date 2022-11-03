use crate::crypto_data::{Crypto, PolygonResponse};
use serde::{Deserialize, Serialize};
use std::ops::Index;
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams<'a> {
    /// 
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
    pub async fn get(
        &self,
        query_params: QueryParams<'_>,
    ) -> Result<PolygonResponse, reqwest::Error> {
        let full_url = Self::querify_paramters(self, query_params);
        let res = self.client.get(full_url).send().await?;

        match res.error_for_status() {
            Ok(res) => {
                let deserialized_data = res.json::<PolygonResponse>().await?;
                Ok(deserialized_data)
            }
            Err(err) => {
                println!("Error: {}", err);
                Err(err)
            }
        }
    }

    #[allow(dead_code)]
    async fn get_next_page_data(&self, page_url: &str) -> Result<PolygonResponse, reqwest::Error> {
        let res = self.client.get(page_url).send().await?;

        match res.error_for_status() {
            Ok(res) => {
                let deserialized_res = res.json::<PolygonResponse>().await?;
                Ok(deserialized_res)
            }

            Err(err) => {
                println!("Something went wrong, Error: {}", err);
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
