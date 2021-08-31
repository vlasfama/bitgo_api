use crate::client::BitGoClient;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::json;


#[async_trait]
pub trait BitGoTransferAPI {
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
impl BitGoTransferAPI for BitGoClient {
    async fn get_transaction(
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

    async fn transfer_list(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/transfer",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.get_api(&request_url, &json!({})).await
    }

    async fn get_fee(
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

    async fn change_fee(
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
