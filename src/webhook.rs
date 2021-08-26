use crate::bitgo_api::BitGoAPI;
use crate::error::{Error, Result};
use serde_json::json;
pub struct Webhook {
    pub bitgo: BitGoAPI,
}

impl Webhook {
    pub fn new(bit_go: BitGoAPI) -> Result<Self> {
        Ok(Webhook { bitgo: bit_go })
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.bitgo
            .post_api(
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
        );

        self.bitgo
            .post_api(
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.bitgo.get_api(&request_url, &json!({})).await
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );

        self.bitgo
            .delete_api(
                &request_url,
                &json!({
                    "type": webhook_type,
                    "webhook_id":webhook_id
                }),
            )
            .await
    }
}
