use std::fmt::Display;

#[derive(Debug)]
pub enum RedisError {
    InvalidInteger(String),
    InvalidSimpleString(String),
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
        }
    }
}
