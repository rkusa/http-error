use std::{error, fmt};

use crate::HttpError;

#[derive(Debug)]
pub struct AnyHttpError(Box<dyn HttpError + Send + 'static>);

impl fmt::Display for AnyHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for AnyHttpError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.0.source()
    }
}

impl From<AnyHttpError> for Box<dyn HttpError + Send + 'static> {
    fn from(err: AnyHttpError) -> Self {
        err.0
    }
}

impl<E> From<E> for AnyHttpError
where
    E: HttpError + Send + 'static,
{
    fn from(err: E) -> Self {
        Self(Box::new(err))
    }
}
