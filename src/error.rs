use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] io::Error),

    #[error("Bincode Serialize and Deserialize Error: {0}")]
    BincodeError(#[from] bincode::Error),

    #[error("parsing error")]
    ParseError(String),

    #[error("execution error")]
    ExecutionError(#[from] ExecutionError)
}

#[derive(Error, Debug)]
pub enum ExecutionError{
    #[error("read error")]
    ReadError(String)
}