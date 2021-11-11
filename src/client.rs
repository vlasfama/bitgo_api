use crate::config::Config;
use crate::error::{Error, Result};
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
        let response = builder
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, format!("Bearer {}", self.token))
        .json(params)
        .send()
        .await?;
        if response.status().is_success(){
            let response_json: serde_json::Value = response
            .json()
            .await?;
        trace!("bitgo api response {:?}", response_json);
        Ok(response_json)
        }else{
           let err_json :serde_json::Value = response.json().await?;
           let msg = match err_json.get("message") {
            Some(value) => value.to_string(),
            None => "Unknown Error".to_string(),
        };
           Err(Error::BitgoError{msg})
        }
    
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
