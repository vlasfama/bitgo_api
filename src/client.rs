use std::fs::File;

use crate::config::Config;
use crate::error::{Error, Result};
use log::trace;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Certificate, Client, ClientBuilder, RequestBuilder};
use std::io::Read;

#[derive(Debug, Clone)]
pub struct BitGoClient {
    pub endpoint: String,
    pub token: String,
    pub bitgo_cert_path: Option<String>,
}

impl BitGoClient {
    pub fn new(endpoint: String, token: String, bitgo_cert_path: Option<String>) -> Result<Self> {
        Ok(BitGoClient {
            endpoint,
            token,
            bitgo_cert_path,
        })
    }

    pub async fn get_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let cert = match self.bitgo_cert_path.as_ref() {
            Some(path) => path.to_string(),
            None => "".to_string(),
        };
        let builder;
        if !cert.is_empty() {
            let client = BitGoClient::get_ssl_certificate(cert)?;
            builder = ClientBuilder::build(client)?.get(request_url);
        } else {
            builder = reqwest::Client::new().get(request_url);
        }

        self.call_api(builder, params).await
    }

    pub async fn post_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let cert = match self.bitgo_cert_path.as_ref() {
            Some(path) => path.to_string(),
            None => "".to_string(),
        };
        let builder;
        if !cert.is_empty() {
            let client = BitGoClient::get_ssl_certificate(cert)?;
            builder = ClientBuilder::build(client)?.post(request_url);
        } else {
            builder = reqwest::Client::new().post(request_url);
        }
        self.call_api(builder, params).await
    }

    pub async fn delete_api<T: serde::Serialize>(
        &self,
        request_url: &str,
        params: &T,
    ) -> Result<serde_json::Value> {
        log::trace!("request url {:?}", request_url);
        let cert = match self.bitgo_cert_path.as_ref() {
            Some(path) => path.to_string(),
            None => "".to_string(),
        };
        let builder;
        if !cert.is_empty() {
            let client = BitGoClient::get_ssl_certificate(cert)?;
            builder = ClientBuilder::build(client)?.delete(request_url);
        } else {
            builder = reqwest::Client::new().delete(request_url);
        }

        self.call_api(builder, params).await
    }

    async fn call_api<T: serde::Serialize>(
        &self,
        builder: RequestBuilder,
        params: &T,
    ) -> Result<serde_json::Value> {
        let response = builder
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .json(params)
            .send()
            .await?;
        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            trace!("bitgo api response {:?}", response_json);
            Ok(response_json)
        } else {
            let err_json: serde_json::Value = response.json().await?;
            let msg = match err_json.get("message") {
                Some(value) => value.to_string(),
                None => "Unknown Error".to_string(),
            };
            Err(Error::BitgoError { msg })
        }
    }

    pub fn get_ssl_certificate(path: String) -> Result<ClientBuilder> {
        let mut buf = Vec::new();
        let mut f = File::open(path).expect("Unable to open file");
        f.read_to_end(&mut buf).expect("Unable to read data");
        let cert = reqwest::Certificate::from_pem(&buf)?;
        let client = ClientBuilder::new().add_root_certificate(cert);
        Ok(client)
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        BitGoClient::new(
            config.endpoint.clone(),
            config.token.clone(),
            config.bitgo_cert_path.clone(),
        )
    }
}
