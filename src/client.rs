use crate::config::Config;
use crate::error::{Result};
use log::{trace};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::RequestBuilder;

#[derive(Debug, Clone)]
pub struct BitGoClient {
    pub endpoint: String,
    pub token: String,
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
        builder: RequestBuilder,
        params: &T,
    ) -> Result<serde_json::Value> {
        let response_json: serde_json::Value = builder
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .json(params)
            .send()
            .await?
            .json()
            .await?;
        trace!("bitgo api response {:?}", response_json);
        Ok(response_json)
    }

    pub async fn get_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().get(request_url);
        self.call_api(builder, params).await
    }

    pub async fn post_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().post(request_url);
        self.call_api(builder, params).await
    }

    pub async fn delete_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().delete(request_url);
        self.call_api(builder, params).await
    }
}
