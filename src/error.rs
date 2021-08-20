use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Connection error: {msg}")]
    Connection { msg: String },

    #[error("BitGo Error: {msg}")]
    BitGoError { msg: String },

    #[error("reqwest Error: {msg}")]
    ReqwestError { msg: String },
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::ReqwestError {
            msg: format!("{}", err),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;