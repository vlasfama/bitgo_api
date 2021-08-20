use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

pub struct BitGoAPI {
    pub endpoint: String,
    pub token: String,
}

pub fn value_or_error(value: serde_json::Value, name: &str) -> Result<serde_json::Value> {
    match value.get(name) {
        Some(value) => Ok(value.to_owned()),
        None => {
            let e = value.get("error").unwrap();
            Err(Error::BitGoError { msg: e.to_string() })
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
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        let response_json: serde_json::Value = reqwest::Client::new()
            .post(request_url)
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .json(params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response_json)
    }

    pub async fn generate_wallet(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/generate",
            url = self.endpoint,
            coin_type = identifier,
        );

        self.call_api(
            &request_url,
            &json!({
                "label": name,
                "passphrase": passphrase,
            }),
        )
        .await
    }

    pub async fn generate_address(
        &self,
        wallet_id: &str,
        identifier: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/address",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.call_api(&request_url, &json!({})).await
    }
}
