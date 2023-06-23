use std::error::Error;
use std::fmt;

use http::StatusCode;

pub trait HttpError: Error {
    fn status_code(&self) -> StatusCode;

    fn reason(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

pub struct Reason<E>(pub E);

impl<E> From<E> for Box<dyn HttpError>
where
    E: HttpError + 'static,
{
    fn from(err: E) -> Self {
        Box::new(err)
    }
}

impl<E> fmt::Display for Reason<E>
where
    E: HttpError,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.reason(f)
    }
}

impl fmt::Display for Reason<Box<dyn HttpError>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.reason(f)
    }
}
