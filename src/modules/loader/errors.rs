use thiserror::Error;
use std::io::Error as IOError;
use std::io;

use serde_json::Error as SerdeError;

pub type LoaderResult<T> = Result<T, LoaderErrors>;

#[derive(Debug, Error)]
pub enum LoaderErrors {
    #[error("Error reading file: {0}")]
    IOError(String),
    #[error("Another Error: {0}")]
    AnotherError(String)
}


impl From<IOError> for LoaderErrors {
    fn from(err: IOError) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound => LoaderErrors::IOError(format!("File Doesn't exist")),
            _ => LoaderErrors::AnotherError(err.to_string())

        }
    }
}

impl From<SerdeError> for LoaderErrors {
    fn from(err: SerdeError) -> Self {
        match err {
            _ => LoaderErrors::AnotherError(err.to_string())

        }
    }
}