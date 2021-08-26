use crate::bitgo_api::BitGoAPI;
use crate::error::{Error, Result};
use serde_json::json;

pub struct Transfer {
    pub bitgo: BitGoAPI,
}

impl Transfer {
    pub fn new(bitgo: BitGoAPI) -> Result<Self> {
        Ok(Transfer { bitgo: bitgo })
    }

    pub async fn get_transaction(
        &self,
        wallet_id: &str,
        identifier: &str,
        transfer_id: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/transfer/{transfer_id}",
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
            transfer_id = transfer_id,
        );
        self.bitgo.get_api(&request_url, &json!({})).await
    }

    pub async fn transfer_list(
        &self,
        wallet_id: &str,
        identifier: &str,
    ) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/transfer",
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.bitgo.get_api(&request_url, &json!({})).await
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
        );
        self.bitgo
            .get_api(
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
            url = self.bitgo.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.bitgo
            .post_api(&request_url, &json!({"txid":tx_id,"fee":fee}))
            .await
    }
}
