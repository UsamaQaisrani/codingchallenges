use std::fmt::Display;

#[derive(Debug)]
pub enum RedisError {
    InvalidInteger(String),
}

impl Display for RedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedisError::InvalidInteger(val) => {
                write!(f, "Unable to parse integer: {}", val)
            }
        }
    }
}
