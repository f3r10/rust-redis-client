use std::fmt;
use serde::{de, ser};


#[derive(Debug)]
pub enum RedisError {
    Message(String),
}

pub type Result<T> = std::result::Result<T, RedisError>;

impl fmt::Display for RedisError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RedisError::Message(msg) => formatter.write_str(msg),
            /* and so forth */
        }
    }
}

impl ser::Error for RedisError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        RedisError::Message(msg.to_string())
    }
}
impl de::Error for RedisError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        RedisError::Message(msg.to_string())
    }
}

impl std::error::Error for RedisError {}
