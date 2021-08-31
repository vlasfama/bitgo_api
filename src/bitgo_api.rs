use crate::config::Config;
use crate::error::{Error, Result};
use crate::transfer::BitGoTransfer;
use crate::wallet::BitGoWallet;
use crate::webhook::BitGoWebhook;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::RequestBuilder;

pub trait BitGo: BitGoWebhook + BitGoWallet + BitGoTransfer {}

#[derive(Debug, Clone)]
pub struct BitGoAPI {
    pub endpoint: String,
    pub token: String,
}

pub fn value_or_error(value: serde_json::Value, name: &str) -> Result<serde_json::Value> {
    match value.get(name) {
        Some(value) => Ok(value.to_owned()),
        None => {
            Err(Error::InvalidKey { key: name.to_owned() })
        }
    }
}

impl BitGoAPI {
    pub fn new(endpoint: String, token: String) -> Result<Self> {
        Ok(BitGoAPI { endpoint, token })
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        BitGoAPI::new(config.endpoint.clone(), config.token.clone())
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
