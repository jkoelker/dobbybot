//

use super::client::Client;
use super::stock::Quote;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Market {
    async fn losers(&self) -> Result<Vec<Quote>>;
    async fn gainers(&self) -> Result<Vec<Quote>>;
    async fn most_active(&self) -> Result<Vec<Quote>>;
}

#[async_trait]
impl Market for Client {
    async fn losers(&self) -> Result<Vec<Quote>> {
        Ok(self.get::<Vec<Quote>>("stock/market/list/losers").await?)
    }

    async fn gainers(&self) -> Result<Vec<Quote>> {
        Ok(self.get::<Vec<Quote>>("stock/market/list/gainers").await?)
    }

    async fn most_active(&self) -> Result<Vec<Quote>> {
        Ok(self
            .get::<Vec<Quote>>("stock/market/list/mostactive")
            .await?)
    }
}
