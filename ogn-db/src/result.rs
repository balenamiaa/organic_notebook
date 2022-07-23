use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Error {
    inner: anyhow::Error,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <anyhow::Error as std::fmt::Display>::fmt(&self.inner, f)?;

        Ok(())
    }
}

impl actix_web::error::ResponseError for Error {}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Error {
        Error { inner: err }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! str_err {
    ($($arg:tt)*) => {
        crate::result::Error::from(anyhow::anyhow!($($arg)*))
    };
}