use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("get metadata fail")]
    Metadata(io::Error),
    #[error("get file name fail: {path:?}")]
    FileName { path: String },
    #[error("get file parent fail: {path:?}")]
    FileParent { path: String },
    #[error("connect fail")]
    Reqwest(#[from] reqwest::Error),
    #[error("io fail")]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
