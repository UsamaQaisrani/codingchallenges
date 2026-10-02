use std::fmt::Display;

#[derive(Debug)]
pub enum RedisError {
    InvalidInteger(String),
    InvalidSimpleString(String),
    InvalidSimpleError(String),
    InvalidBulkStringError(String),
    InvalidArrayError(String),
}

impl Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisError::InvalidInteger(s) => {
                write!(f, "Unable to parse integer: {}", s)
            }
            RedisError::InvalidSimpleString(s) => {
                write!(f, "Unable to parse bulk string: {}", s)
            }
            RedisError::InvalidSimpleError(s) => {
                write!(f, "Unable to parse error: {}", s)
            }
            RedisError::InvalidBulkStringError(s) => {
                write!(f, "Unable to parse bulk string: {}", s)
            }
            RedisError::InvalidArrayError(s) => {
                write!(f, "Unable to parse array: {}", s)
            }
        }
    }
}
