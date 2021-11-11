use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Invalid key: {key}")]
    InvalidKey { key: String },

    #[error("reqwest Error: {msg}")]
    ReqwestError { msg: String },

    #[error("bitgo Error: {msg}")]
    BitgoError { msg: String },
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError {
            msg: format!("{}", err),
        }
    }
}


pub type Result<T> = std::result::Result<T, Error>;
