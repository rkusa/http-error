mod reason;
pub mod result_ext;

use std::error::Error;
use std::fmt;

pub use http::StatusCode;
use http::{HeaderName, HeaderValue};
pub use reason::Reason;
pub use result_ext::ResultExt;

pub trait HttpError: Error {
    fn status_code(&self) -> StatusCode;

    fn reason(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(reason) = self.status_code().canonical_reason() {
            f.write_str(reason)?;
        }
        Ok(())
    }

    fn headers(&self) -> Option<Vec<(HeaderName, HeaderValue)>> {
        None
    }
}

impl<E> From<E> for Box<dyn HttpError>
where
    E: HttpError + 'static,
{
    fn from(err: E) -> Self {
        Box::new(err)
    }
}
