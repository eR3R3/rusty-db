use thiserror::Error;
use std::io;

#[derive(Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] io::Error),

    #[error("Bincode Serialize and Deserialize Error: {0}")]
    BincodeError(#[from] bincode::Error),
}

pub enum ParserError {
    ParseSelectError(String),

}

pub enum StorageEngineError {
    IOError(String),

}