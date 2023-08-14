use std::fmt;

use crate::HttpError;

pub struct Reason<E>(pub E);

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

impl fmt::Display for Reason<Box<dyn HttpError + Send + 'static>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.reason(f)
    }
}
