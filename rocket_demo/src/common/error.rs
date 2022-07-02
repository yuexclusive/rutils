use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    BasicError(String),
    ValidationError(String),
    OtherError(String),
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

impl From<regex::Error> for ErrorKind {
    fn from(err: regex::Error) -> Self {
        ErrorKind::OtherError(err.to_string())
    }
}

pub trait ToError {
    fn to_basic_error(&self) -> ErrorKind;
    fn to_validation_error(&self) -> ErrorKind;
}

impl ToError for &str
{
    fn to_basic_error(&self) -> ErrorKind {
        ErrorKind::BasicError(self.to_string())
    }

    fn to_validation_error(&self) -> ErrorKind {
        ErrorKind::ValidationError(self.to_string())
    }
}
