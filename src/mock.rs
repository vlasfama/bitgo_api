use crate::error::Result;
use async_trait::async_trait;
use mockall::mock;

use crate::{transfer::BitGoTransfer, wallet::BitGoWallet, webhook::BitGoWebhook};

mock! {
    pub BitGo {}


    #[async_trait]
    impl BitGoTransfer for BitGo {
        async fn get_transaction(
            &self,
            wallet_id: &str,
            identifier: &str,
            transfer_id: &str,
        ) -> Result<serde_json::Value>;

        async fn transfer_list(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value>;
        async fn get_fee(
            &self,
            identifier: &str,
            num_blocks: &str,
            recipient: &str,
            data: &str,
            amount: &str,
            hop: &str,
        ) -> Result<serde_json::Value>;
        async fn change_fee(
            &self,
            identifier: &str,
            wallet_id: &str,
            tx_id: &str,
            fee: &str,
        ) -> Result<serde_json::Value>;
    }

    #[async_trait]
    impl BitGoWallet for BitGo {
        async fn generate(
            &self,
            name: &str,
            identifier: &str,
            passphrase: &str,
        ) -> Result<serde_json::Value>;
        async fn address(
            &self,
            wallet_id: &str,
            identifier: &str,
        ) -> Result<serde_json::Value>;
    }

    #[async_trait]
    impl BitGoWebhook for BitGo {
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
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::bitgo_api::value_or_error;

    use super::*;

    #[tokio::test]
    async fn test_mocking() {
        let mut mock = MockBitGo::new();
        mock.expect_address().return_const(Ok(
            json!({ "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS" }),
        ));

        let v = mock.address("any", " any").await.unwrap();
        assert_eq!(
            value_or_error(v, "address").unwrap().to_owned(),
            "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS"
        );
    }
}
