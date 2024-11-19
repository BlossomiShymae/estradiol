use thiserror::Error;

use crate::models::media::Stream;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ureq error")]
    Ureq(#[from] Box<ureq::Error>),
    #[error("std::io error")]
    StdIo(#[from] std::io::Error),
    #[error("failed to get bytes {0}")]
    InvalidData(String),
    #[error("regex error")]
    Regex(regex::Error),
    #[error("unknown error")]
    Unknown,
}
