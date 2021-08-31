use crate::bitgo_api::BitGoAPI;
use crate::error::Result;
use async_trait::async_trait;
use serde_json::json;

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait BitGoWallet {
    async fn generate(
        &self,
        name: &str,
        identifier: &str,
        passphrase: &str,
    ) -> Result<serde_json::Value>;
    async fn address(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value>;
}
pub struct Wallet {
    pub bitgo: BitGoAPI,
}
#[async_trait]
impl BitGoWallet for BitGoAPI {
    /// This API call creates a new wallet. Under the hood, the SDK (or BitGo Express) does the following:
    ///
    /// 1.Creates the user keychain locally on the machine, and encrypts it with the provided passphrase (skipped if userKey is provided).
    /// 2.Creates the backup keychain locally on the machine.
    /// 3.Uploads the encrypted user keychain and public backup keychain.
    /// 4.Creates the BitGo key (and the backup key if backupXpubProvider is set) on the service.
    /// 5.Creates the wallet on BitGo with the 3 public keys above.
    async fn generate(
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

    /// This API call is used to create a new receive address for your wallet.
    /// You may choose to call this API whenever a deposit is made.
    /// The BitGo API supports millions of addresses.
    async fn address(&self, wallet_id: &str, identifier: &str) -> Result<serde_json::Value> {
        let request_url = format!(
            "{url}/api/v2/{coin_type}/wallet/{wallet_id}/address",
            url = self.endpoint,
            coin_type = identifier,
            wallet_id = wallet_id,
        );
        self.post_api(&request_url, &json!({})).await
    }
}
