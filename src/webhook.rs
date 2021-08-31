use crate::client::BitGoClient;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::json;

#[async_trait]
pub trait BitGoWebhookAPI {
    async fn add_wallet_webhook(
        &self,
        wallet_id: &str,
        identifier: &str,
        webhook_label: &str,
        webhook_type: &str,
        webhook_url: &str,
    ) -> Result<serde_json::Value>;

    async fn add_block_webhook(
        &self,
        identifier: &str,
        webhook_type: &str,
        webhook_label: &str,
        webhook_url: &str,
    ) -> Result<serde_json::Value>;

    async fn list_webhook(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value>;

    async fn remove_webhook(
        &self,
        wallet_id: &str,
        identifier: &str,
        webhook_type: &str,
        webhook_id: &str,
    ) -> Result<serde_json::Value>;
}
#[async_trait]
impl BitGoWebhookAPI for BitGoClient {
    async fn add_wallet_webhook(
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

    async fn add_block_webhook(
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

    async fn list_webhook(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/webhooks",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.get_api(&request_url, &json!({})).await
    }

    async fn remove_webhook(
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
}
