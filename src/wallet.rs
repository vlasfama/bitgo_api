use crate::bitgo_api::BitGoAPI;
use crate::error::{Error, Result};
use serde_json::json;
pub struct Wallet {
    pub bitgo: BitGoAPI,
}

impl Wallet {
    pub fn new(bit_go: BitGoAPI) -> Result<Self> {
        Ok(Wallet { bitgo: bit_go })
    }

    pub async fn generate_wallet(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/generate",
            url = self.bitgo.endpoint,
            coin_type = identifier,
        );

        self.bitgo
            .post_api(
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.bitgo.post_api(&request_url, &json!({})).await
    }
}
