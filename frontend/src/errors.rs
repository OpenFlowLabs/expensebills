use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("error during sending request {0}")]
    RequestError(String),
    #[error("error deserializing object {0}")]
    DeserializeError(String),
    #[error("{0}")]
    FrontendError(String)
}
