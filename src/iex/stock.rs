//

use super::client::Client;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "companyName")]
    pub name: String,
    pub symbol: String,
    pub calculation_price: String,

    pub open: Option<f64>,
    pub open_time: Option<i64>,
    pub close: Option<f64>,
    pub close_time: Option<i64>,
    pub high: Option<f64>,
    pub low: Option<f64>,

    #[serde(rename = "latestPrice")]
    pub price: f64,
    #[serde(rename = "latestSource")]
    pub latest_source: String,
    #[serde(rename = "latestUpdate")]
    pub time: f64,
    #[serde(rename = "latestVolume")]
    pub volume: Option<i64>,
    pub change: f64,
    pub change_percent: f64,
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
