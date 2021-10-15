use crate::config::Config;
use crate::error::{Error, Result};
extern crate pem;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
#[derive(Debug, Clone)]
pub struct BitGoClient {
    pub endpoint: String,
    pub token: String,
}

pub fn value_or_error(value: &serde_json::Value, name: &str) -> Result<serde_json::Value> {
    match value.get(name) {
        Some(value) => Ok(value.to_owned()),
        None => Err(Error::InvalidKey {
            key: name.to_owned(),
        }),
    }
}

impl BitGoClient {
    pub fn new(endpoint: String, token: String) -> Result<Self> {
        Ok(BitGoClient { endpoint, token })
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        BitGoClient::new(config.endpoint.clone(), config.token.clone())
    }

    async fn call_api<T: serde::Serialize>(
        &self,
        params: &T,
        request_url: &str,
    ) -> Result<serde_json::Value> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;

        let response_json: serde_json::Value = client
            .get(request_url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .json(params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response_json)
    }

    pub async fn get_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.call_api(params, request_url).await
    }

    pub async fn post_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.call_api(params, request_url).await
    }

    pub async fn delete_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        self.call_api(params, request_url).await
    }
}
