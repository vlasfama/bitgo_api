use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::RequestBuilder;
use serde_json::json;

#[derive(Debug, Clone)]
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

    async fn get_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().get(request_url);
        self.call_api(builder, params).await
    }

    async fn post_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().post(request_url);
        self.call_api(builder, params).await
    }

    async fn delete_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::new().delete(request_url);
        self.call_api(builder, params).await
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

        self.post_api(
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

        self.post_api(&request_url, &json!({})).await
    }

    pub async fn add_wallet_webhook(
        &self,
        wallet_id: &str,
        identifier: &str,
        webhook_label: &str,
        webhook_type: &str,
        webhook_url: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/webhooks",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.post_api(
            &request_url,
            &json!({
                "type": webhook_type,
                "url": webhook_url,
                "label":webhook_label,
            }),
        )
        .await
    }
    pub async fn add_block_webhook(
        &self,
        identifier: &str,
        webhook_type: &str,
        webhook_label: &str,
        webhook_url: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/webhooks",
            url = self.endpoint,
            coin_type = identifier,
        );

        self.post_api(
            &request_url,
            &json!({
                "type": webhook_type,
                "url": webhook_url,
                "label":webhook_label,
            }),
        )
        .await
    }

    pub async fn list_webhook(
        &self,
        wallet_id: &str,
        identifier: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/webhooks",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.get_api(&request_url, &json!({})).await
    }

    pub async fn remove_webhook(
        &self,
        wallet_id: &str,
        identifier: &str,
        webhook_type: &str,
        webhook_id: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/webhooks",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.delete_api(
            &request_url,
            &json!({
                "type": webhook_type,
                "webhook_id":webhook_id
            }),
        )
        .await
    }

    pub async fn get_transaction(
        &self,
        wallet_id: &str,
        identifier: &str,
        transfer_id: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/transfer/{transfer_id}",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
            transfer_id = transfer_id,
        );
        self.get_api(&request_url, &json!({})).await
    }

    pub async fn transfer_list(
        &self,
        wallet_id: &str,
        identifier: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/transfer",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.get_api(&request_url, &json!({})).await
    }

    pub async fn get_fee(
        &self,
        identifier: &str,
        num_blocks: &str,
        recipient: &str,
        data: &str,
        amount: &str,
        hop: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/tx/fee",
            url = self.endpoint,
            coin_type = identifier,
        );
        self.get_api(
            &request_url,
            &json!({
                "numBlocks":num_blocks,
                "recipient":recipient,
                "data":data,
                "amount":amount,
                "hop":hop,
            }),
        )
        .await
    }

    pub async fn change_fee(
        &self,
        identifier: &str,
        wallet_id: &str,
        tx_id: &str,
        fee: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/tx/changeFee",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.post_api(&request_url, &json!({"txid":tx_id,"fee":fee}))
            .await
    }
}
