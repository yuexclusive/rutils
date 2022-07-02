use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    BasicError(String),
    ValidationError(String),
    OtherError(String),
}

pub fn basic(msg: &str) -> ErrorKind {
    ErrorKind::BasicError(msg.to_string())
}

pub fn validation(msg: &str) -> ErrorKind {
    ErrorKind::ValidationError(msg.to_string())
}

impl<'a> Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::BasicError(msg) => f.write_fmt(format_args!("business error: {}", msg)),
            ErrorKind::ValidationError(msg) => {
                f.write_fmt(format_args!("validation error: {}", msg))
            }
            ErrorKind::OtherError(msg) => f.write_fmt(format_args!("other error: {}", msg)),
        }
    }
}

impl std::error::Error for ErrorKind {}

impl From<sqlx::Error> for ErrorKind {
    fn from(err: sqlx::Error) -> Self {
        ErrorKind::OtherError(err.as_database_error().unwrap().to_string())
    }
}

impl From<redis::RedisError> for ErrorKind {
    fn from(err: redis::RedisError) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

impl From<bincode::Error> for ErrorKind {
    fn from(err: bincode::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for ErrorKind {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for ErrorKind {
    fn from(err: std::time::SystemTimeError) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}
