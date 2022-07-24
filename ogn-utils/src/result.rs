use std::error::Error as StdError;
use std::fmt::{Display, Formatter};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Error {
    inner: anyhow::Error,
}

impl Error {
    pub fn new(anyhow_err: anyhow::Error) -> Self {
        Error { inner: anyhow_err }
    }
}

trait ErrorWrapper {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl<T> ErrorWrapper for T
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        StdError::source(self)
    }
}

impl<T: ErrorWrapper + Sync + Send + 'static> From<T> for Error {
    fn from(e: T) -> Self {
        Self {
            inner: if let Some(x) = e.source() {
                anyhow!("{}", x)
            } else {
                anyhow!("text extraction error")
            },
        }
    }
}

impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        e.inner
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl actix_web::ResponseError for Error {}

pub type Result<T> = std::result::Result<T, Error>;
