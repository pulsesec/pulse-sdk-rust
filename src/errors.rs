use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt;
use thiserror::Error;

use crate::types::APIErrorData;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TokenNotFoundError(pub APIErrorData);

impl fmt::Display for TokenNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token not found: {}", self.0.error)
    }
}

impl StdError for TokenNotFoundError {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TokenUsedError(pub APIErrorData);

impl fmt::Display for TokenUsedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token used: {}", self.0.error)
    }
}

impl StdError for TokenUsedError {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct TokenExpiredError(pub APIErrorData);

impl fmt::Display for TokenExpiredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Token expired: {}", self.0.error)
    }
}

impl StdError for TokenExpiredError {}

#[derive(Debug, Error, PartialEq)]
pub enum PulseError {
    #[error("{0}")]
    TokenNotFoundError(#[from] TokenNotFoundError),
    #[error("{0}")]
    TokenUsedError(#[from] TokenUsedError),
    #[error("{0}")]
    TokenExpiredError(#[from] TokenExpiredError),
    #[error("Unknown error")]
    UnknownError(String),
}
