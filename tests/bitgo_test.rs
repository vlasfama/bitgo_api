use bitgo_api::wallet::BitGoWallet;
use bitgo_api::webhook::BitGoWebhook;

#[tokio::test]
async fn test_bitgo_wallet_integrations() {
    let bg =
        bitgo_api::bitgo_api::BitGoAPI::new("endpoint".to_string(), "token".to_string()).unwrap();

    bg.generate_wallet("wallet", "tbtc", "12345").await.unwrap();
    bg.create_address("wallet_id", "tbtc").await.unwrap();
}

#[tokio::test]
async fn test_bitgo_webhook_integrations() {
    let bg =
        bitgo_api::bitgo_api::BitGoAPI::new("endpoint".to_string(), "token".to_string()).unwrap();

    bg.add_wallet_webhook("wallet", "tbtc", "12345", "", "")
        .await
        .unwrap();
    bg.add_block_webhook("tbtc", "transfer", "wallet", "")
        .await
        .unwrap();
}
