use reqwest;

use std::io;
use std::result;

pub type Result<T = ()> = result::Result<T, ZippydError>;

#[derive(Debug)]
pub enum ZippydError {
    InvalidFormula(meval::Error),
    Reqwest(reqwest::Error),
    Io(io::Error),
    InvalidUrl(String),
    NoneError,
}

impl From<std::option::NoneError> for ZippydError {
    fn from(_err: std::option::NoneError) -> ZippydError {
        ZippydError::NoneError
    }
}

impl From<reqwest::Error> for ZippydError {
    fn from(err: reqwest::Error) -> ZippydError {
        ZippydError::Reqwest(err)
    }
}

impl From<meval::Error> for ZippydError {
    fn from(err: meval::Error) -> ZippydError {
        ZippydError::InvalidFormula(err)
    }
}

impl From<io::Error> for ZippydError {
    fn from(err: io::Error) -> ZippydError {
        ZippydError::Io(err)
    }
}
