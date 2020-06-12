//

use super::client::Client;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub close: f64,
    pub change: f64,
    #[serde(rename = "changePercent")]
    pub change_percent: f64,
    pub low: f64,
    pub high: f64,
    #[serde(rename = "companyName")]
    pub name: String,
    pub open: f64,
    #[serde(rename = "latestPrice")]
    pub price: f64,
    pub symbol: String,
    #[serde(rename = "latestUpdate")]
    pub time: i64,
    pub volume: i64,
}

#[async_trait]
pub trait Stock {
    async fn quote(&self, symbol: &str) -> Result<Quote>;
}

#[async_trait]
impl Stock for Client {
    async fn quote(&self, symbol: &str) -> Result<Quote> {
        let path = format!("stock/{}/quote/", symbol);
        Ok(self.get::<Quote>(&path).await?)
    }
}
