pub mod client;
pub mod config;
pub mod error;
pub mod transfer;
pub mod wallet;
pub mod webhook;
pub mod mock;

use crate::transfer::BitGoTransferAPI;
use crate::wallet::BitGoWalletAPI;
use crate::webhook::BitGoWebhookAPI;
pub trait BitGoAPI:
    BitGoWebhookAPI + BitGoWalletAPI + BitGoTransferAPI + Sync + Send + 'static
{
}
impl<T: BitGoWebhookAPI + BitGoWalletAPI + BitGoTransferAPI + Sync + Send + 'static> BitGoAPI
    for T
{
}

