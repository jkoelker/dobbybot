//

use super::client::Client;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub symbol: String,
    pub exchange: String,
    pub name: String,
    pub date: String,
    #[serde(rename = "isEnabled")]
    pub enabled: bool,
    #[serde(rename = "type")]
    pub issue_type: String,
    pub region: String,
    pub currency: String,
    #[serde(rename = "iexId")]
    pub iex_id: String,
    pub figi: Option<String>,
    pub cik: Option<String>,
}

#[async_trait]
pub trait Reference {
    async fn symbols(&self) -> Result<Vec<Symbol>>;
}

#[async_trait]
impl Reference for Client {
    async fn symbols(&self) -> Result<Vec<Symbol>> {
        Ok(self.get::<Vec<Symbol>>("ref-data/symbols").await?)
    }
}
