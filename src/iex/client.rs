//

// TODO(jkoelker) switch to thiserror
use anyhow::{anyhow, Result};
use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;

use log::{debug, warn};

#[derive(Clone)]
pub struct Client {
    token: String,
    url: reqwest::Url,
    client: reqwest::Client,
}

fn handle_errors(body: &str, s: StatusCode, u: reqwest::Url) -> Result<()> {
    if s.is_client_error() || s.is_server_error() {
        Err(anyhow!("({:?}) {}: {}", s, body, u))
    } else {
        Ok(())
    }
}

impl Client {
    pub fn new(token: String) -> Self {
        let host = if token.starts_with('T') {
            "sandbox.iexapis.com"
        } else {
            "cloud.iexapis.com"
        };

        let base_url = format!("https://{}/stable/", host);

        Client {
            token,
            url: reqwest::Url::parse(&base_url)
                .expect("Unable to parse IEX host"),
            client: reqwest::Client::builder()
                .build()
                .expect("Unable to build client"),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        Ok(self.request::<T>(Method::GET, path).await?)
    }

    async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
    ) -> Result<T> {
        let url = self.url.join(path)?;
        let res = self
            .client
            .request(method, url.clone())
            .query(&[("token", self.token.clone())])
            .send()
            .await?;

        let status = res.status();
        let text = res.text().await?;

        debug!("Status = {:?} for {}", status, url);

        handle_errors(&text, status, url)?;

        serde_json::from_str(&text).map_err(|e| {
            warn!("{}, {:?}", text, e);
            anyhow!(e)
        })
    }
}
