use crate::config::Config;
use crate::error::{Error, Result};
use reqwest::header;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client, ClientBuilder, RequestBuilder};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct BitGoClient {
    pub endpoint: String,
    pub token: String,
}

pub fn value_or_error(value: &serde_json::Value, name: &str) -> Result<serde_json::Value> {
    match value.get(name) {
        Some(value) => Ok(value.to_owned()),
        None => Err(Error::InvalidKey {
            key: name.to_owned(),
        }),
    }
}

impl BitGoClient {
    pub fn new(endpoint: String, token: String) -> Result<Self> {
        Ok(BitGoClient { endpoint, token })
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        BitGoClient::new(config.endpoint.clone(), config.token.clone())
    }

    async fn call_api<T: serde::Serialize>(
        &self,
        builder: ClientBuilder,
        params: &T,
        request_url: &str,
    ) -> Result<serde_json::Value> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(self.token.as_str()).unwrap(),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        // read a local binary DER encoded certificate
        let pem = std::fs::read("./cert.pem").unwrap();
        let cert = reqwest::Certificate::from_pem(&pem)?;

        let client = builder
            .add_root_certificate(cert)
            .default_headers(headers)
            .build()?;

        let response_json: serde_json::Value = client
            .get(request_url)
            .json(params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response_json)
    }

    pub async fn get_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::builder();
        self.call_api(builder, params, request_url).await
    }

    pub async fn post_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::builder();
        self.call_api(builder, params, request_url).await
    }

    pub async fn delete_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let builder = reqwest::Client::builder();
        self.call_api(builder, params, request_url).await
    }
}
