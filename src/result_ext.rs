use std::{error, fmt};

use http::StatusCode;

use crate::HttpError;

#[derive(Debug)]
pub struct StatusError<E> {
    status: StatusCode,
    inner: E,
}

pub trait ResultExt<T, E> {
    fn not_found(self) -> Result<T, StatusError<E>>;
}
impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn not_found(self) -> Result<T, StatusError<E>> {
        self.map_err(|inner| StatusError {
            status: StatusCode::NOT_FOUND,
            inner,
        })
    }
}

impl<E> error::Error for StatusError<E>
where
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl<E> fmt::Display for StatusError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<E> HttpError for StatusError<E>
where
    E: error::Error + 'static,
{
    fn status_code(&self) -> StatusCode {
        self.status
    }
}
