use crate::bitgo_api::BitGoAPI;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::json;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BitGoWallet {
    async fn generate_wallet(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
    ) -> Result<serde_json::Value>;
    async fn create_address(
        &self,
        wallet_id: &str,
        identifier: &str,
    ) -> Result<serde_json::Value>;
}
pub struct Wallet {
    pub bitgo: BitGoAPI,
}
#[async_trait]
impl BitGoWallet for BitGoAPI {
    async fn generate_wallet(
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

    async fn create_address(
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
}
