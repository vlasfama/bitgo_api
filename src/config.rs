use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Config {
    #[structopt(
        long = "bitgo-endpoint",
        env = "BITGO_ENDPOINT",
        about = "Bitgo REST API endpoint"
    )]
    pub endpoint: String,
    #[structopt(
        long = "bitgo-token",
        env = "BITGO_TOKEN",
        about = "API key for the Bitgo service"
    )]
    pub token: String,

    #[structopt(
        long = "bitgo-webhook-url",
        short = "u",
        env = "WEBHOOK_URL",
        about = "Webhook url  for the Bitgo service"
    )]
    pub webhook_url: String,
}
