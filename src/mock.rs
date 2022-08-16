use super::client::BitGoClient;
use crate::error::Result;
use async_trait::async_trait;
use mockall::mock;

use crate::{transfer::BitGoTransferAPI, wallet::BitGoWalletAPI, webhook::BitGoWebhookAPI};

mock! {
    pub BitGoClient {

    }
    #[async_trait]
    impl BitGoTransferAPI for BitGoClient {
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
            num_blocks: &i32,
            recipient: &str,
            data: &str,
            amount: &str,
            hop: bool,
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
    impl BitGoWalletAPI for BitGoClient {
        async fn generate_wallet(
            &self,
            name: &str,
            identifier: &str,
            passphrase: &str,
        ) -> Result<serde_json::Value>;
        async fn generate_enterprise_wallet(
            &self,
            name: &str,
            identifier: &str,
            passphrase: &str,
            enterprise_id:&str,
        ) -> Result<serde_json::Value>;
        async fn create_address(
            &self,
            wallet_id: &str,
            identifier: &str,
            forwarde_version: i32,
        ) -> Result<serde_json::Value>;
        async fn get_wallet_list(
            &self
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
            num_confirmation:i32,
            all_token: bool,
            listen_failure_states: bool,
        ) -> Result<serde_json::Value>;



        async fn add_block_webhook(
            &self,
            identifier: &str,
            webhook_type: &str,
            webhook_label: &str,
            webhook_url: &str,
            num_confirmation:i32
        ) -> Result<serde_json::Value>;
        async fn list_wallet_webhook(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value>;
        async fn list_block_webhook(&self,identifier: &str) -> Result<serde_json::Value>;
        async fn remove_wallet_webhook(
            &self,
            wallet_id: &str,
            identifier: &str,
            webhook_type: &str,
            webhook_url: &str,
            webhook_id:&str,
        ) -> Result<serde_json::Value>;

        async fn remove_block_webhook(
            &self,
            identifier: &str,
            webhook_type: &str,
            webhook_url: &str,
            webhook_id: &str,
        ) -> Result<serde_json::Value>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    #[ignore = "required bitgo server to be running"]
    async fn test_tls_mocking() {
        let client = BitGoClient::new(
            "https:://localhost:4000".to_string(),
            "".to_string(),
            Some("/Users/nagaraj/coinhaven/bitgo_api/certs/cert.pem".to_string()),
        );
        if let Ok(cl) = client {
            let res = cl
                .get_api("https://localhost:4000/api/v2/ping", &"".to_string())
                .await
                .unwrap();
            print!("{:?}", res)
        } else {
            todo!()
        }
    }
    #[tokio::test]
    #[ignore = "required bitgo server to be running"]
    async fn test_without_tls() {
        let client = BitGoClient::new("https:://localhost:4000".to_string(), "".to_string(), None);
        if let Ok(cl) = client {
            let res = cl
                .get_api("https://localhost:4000/api/v2/ping", &"".to_string())
                .await;
            assert!(res.is_err())
        } else {
            todo!()
        }
    }

    #[tokio::test]
    #[ignore = "required bitgo server to be running"]
    async fn test_connect_local_bitgo_express() {
        let client = BitGoClient::new("http:://localhost:4000".to_string(), "".to_string(), None);
        if let Ok(cl) = client {
            let res = cl
                .get_api("http://localhost:4000/api/v2/ping", &"".to_string())
                .await
                .unwrap();
            print!("{:?}", res)
        } else {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_mocking() {
        let mut mock = MockBitGoClient::new();
        mock.expect_create_address().return_const(Ok(
            json!({ "address": "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS" }),
        ));

        let v = mock.create_address("any", " any", 0).await.unwrap();

        assert_eq!(
            v.get("address").unwrap().to_owned(),
            "2MvrwRYBAuRtPTiZ5MyKg42Ke55W3fZJfZS"
        );
    }
}
