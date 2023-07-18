use std::{error, fmt};

use http::StatusCode;

use crate::HttpError;

#[derive(Debug)]
pub struct StatusError<E> {
    status: StatusCode,
    inner: E,
}

macro_rules! trait_fn {
    ($name:ident) => {
        fn $name(self) -> Result<T, StatusError<E>>;
    };
}
macro_rules! impl_fn {
    ($name:ident, $status:ident) => {
        fn $name(self) -> Result<T, StatusError<E>> {
            self.map_err(|inner| StatusError {
                status: StatusCode::$status,
                inner,
            })
        }
    };
}

pub trait ResultExt<T, E> {
    // 400 Bad Request
    trait_fn!(bad_request);
    // 401 Unauthorized
    trait_fn!(unauthorized);
    // 402 Payment Required
    trait_fn!(payment_required);
    // 403 Forbidden
    trait_fn!(forbidden);
    // 404 Not Found
    trait_fn!(not_found);
    // 405 Method Not Allowed
    trait_fn!(method_not_allowed);
    // 406 Not Acceptable
    trait_fn!(not_acceptable);
    // 407 Proxy Authentication Required
    trait_fn!(proxy_authentication_required);
    // 408 Request Timeout
    trait_fn!(request_timeout);
    // 409 Conflict
    trait_fn!(conflict);
    // 410 Gone
    trait_fn!(gone);
    // 411 Length Required
    trait_fn!(length_required);
    // 412 Precondition Failed
    trait_fn!(precondition_failed);
    // 413 Payload Too Large
    trait_fn!(payload_too_large);
    // 414 URI Too Long
    trait_fn!(uri_too_long);
    // 415 Unsupported Media Type
    trait_fn!(unsupported_media_type);
    // 416 Range Not Satisfiable
    trait_fn!(range_not_satisfiable);
    // 417 Expectation Failed
    trait_fn!(expectation_failed);
    // 418 I'm a teapot
    trait_fn!(im_a_teapot);

    // 421 Misdirected Request
    trait_fn!(misdirected_request);
    // 422 Unprocessable Entity
    trait_fn!(unprocessable_entity);
    // 423 Locked
    trait_fn!(locked);
    // 424 Failed Dependency
    trait_fn!(failed_dependency);

    // 426 Upgrade Required
    trait_fn!(upgrade_required);

    // 428 Precondition Required
    trait_fn!(precondition_required);
    // 429 Too Many Requests
    trait_fn!(too_many_requests);

    // 431 Request Header Fields Too Large
    trait_fn!(request_header_fields_too_large);

    // 451 Unavailable For Legal Reasons
    trait_fn!(unavailable_for_legal_reasons);

    // 500 Internal Server Error
    trait_fn!(internal_server_error);
    // 501 Not Implemented
    trait_fn!(not_implemented);
    // 502 Bad Gateway
    trait_fn!(bad_gateway);
    // 503 Service Unavailable
    trait_fn!(service_unavailable);
    // 504 Gateway Timeout
    trait_fn!(gateway_timeout);
    // 505 HTTP Version Not Supported
    trait_fn!(http_version_not_supported);
    // 506 Variant Also Negotiates
    trait_fn!(variant_also_negotiates);
    // 507 Insufficient Storage
    trait_fn!(insufficient_storage);
    // 508 Loop Detected
    trait_fn!(loop_detected);

    // 510 Not Extended
    trait_fn!(not_extended);
    // 511 Network Authentication Required
    trait_fn!(network_authentication_required);
}
impl<T, E> ResultExt<T, E> for Result<T, E> {
    impl_fn!(bad_request, BAD_REQUEST);
    impl_fn!(unauthorized, UNAUTHORIZED);
    impl_fn!(payment_required, PAYMENT_REQUIRED);
    impl_fn!(forbidden, FORBIDDEN);
    impl_fn!(not_found, NOT_FOUND);
    impl_fn!(method_not_allowed, METHOD_NOT_ALLOWED);
    impl_fn!(not_acceptable, NOT_ACCEPTABLE);
    impl_fn!(proxy_authentication_required, PROXY_AUTHENTICATION_REQUIRED);
    impl_fn!(request_timeout, REQUEST_TIMEOUT);
    impl_fn!(conflict, CONFLICT);
    impl_fn!(gone, GONE);
    impl_fn!(length_required, LENGTH_REQUIRED);
    impl_fn!(precondition_failed, PRECONDITION_FAILED);
    impl_fn!(payload_too_large, PAYLOAD_TOO_LARGE);
    impl_fn!(uri_too_long, URI_TOO_LONG);
    impl_fn!(unsupported_media_type, UNSUPPORTED_MEDIA_TYPE);
    impl_fn!(range_not_satisfiable, RANGE_NOT_SATISFIABLE);
    impl_fn!(expectation_failed, EXPECTATION_FAILED);
    impl_fn!(im_a_teapot, IM_A_TEAPOT);

    impl_fn!(misdirected_request, MISDIRECTED_REQUEST);
    impl_fn!(unprocessable_entity, UNPROCESSABLE_ENTITY);
    impl_fn!(locked, LOCKED);
    impl_fn!(failed_dependency, FAILED_DEPENDENCY);

    impl_fn!(upgrade_required, UPGRADE_REQUIRED);

    impl_fn!(precondition_required, PRECONDITION_REQUIRED);
    impl_fn!(too_many_requests, TOO_MANY_REQUESTS);

    impl_fn!(
        request_header_fields_too_large,
        REQUEST_HEADER_FIELDS_TOO_LARGE
    );

    impl_fn!(unavailable_for_legal_reasons, UNAVAILABLE_FOR_LEGAL_REASONS);

    impl_fn!(internal_server_error, INTERNAL_SERVER_ERROR);
    impl_fn!(not_implemented, NOT_IMPLEMENTED);
    impl_fn!(bad_gateway, BAD_GATEWAY);
    impl_fn!(service_unavailable, SERVICE_UNAVAILABLE);
    impl_fn!(gateway_timeout, GATEWAY_TIMEOUT);
    impl_fn!(http_version_not_supported, HTTP_VERSION_NOT_SUPPORTED);
    impl_fn!(variant_also_negotiates, VARIANT_ALSO_NEGOTIATES);
    impl_fn!(insufficient_storage, INSUFFICIENT_STORAGE);
    impl_fn!(loop_detected, LOOP_DETECTED);

    impl_fn!(not_extended, NOT_EXTENDED);
    impl_fn!(
        network_authentication_required,
        NETWORK_AUTHENTICATION_REQUIRED
    );
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
