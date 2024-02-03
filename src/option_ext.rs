use std::{error, fmt};

use http::StatusCode;

use crate::HttpError;

#[derive(Debug)]
pub struct StatusError {
    status: StatusCode,
    context: &'static str,
    #[cfg(feature = "tracing")]
    span: tracing::Span,
}

macro_rules! trait_fn {
    ($name:ident) => {
        fn $name(self, context: &'static str) -> Result<T, StatusError>;
    };
}
macro_rules! impl_fn {
    ($name:ident, $status:ident) => {
        fn $name(self, context: &'static str) -> Result<T, StatusError> {
            self.ok_or_else(|| StatusError {
                status: StatusCode::$status,
                context,
                #[cfg(feature = "tracing")]
                span: tracing::Span::current(),
            })
        }
    };
}

pub trait OptionExt<T> {
    // 400 Bad Request
    trait_fn!(ok_or_bad_request);
    // 401 Unauthorized
    trait_fn!(ok_or_unauthorized);
    // 402 Payment Required
    trait_fn!(ok_or_payment_required);
    // 403 Forbidden
    trait_fn!(ok_or_forbidden);
    // 404 Not Found
    trait_fn!(ok_or_not_found);
    // 405 Method Not Allowed
    trait_fn!(ok_or_method_not_allowed);
    // 406 Not Acceptable
    trait_fn!(ok_or_not_acceptable);
    // 407 Proxy Authentication Required
    trait_fn!(ok_or_proxy_authentication_required);
    // 408 Request Timeout
    trait_fn!(ok_or_request_timeout);
    // 409 Conflict
    trait_fn!(ok_or_conflict);
    // 410 Gone
    trait_fn!(ok_or_gone);
    // 411 Length Required
    trait_fn!(ok_or_length_required);
    // 412 Precondition Failed
    trait_fn!(ok_or_precondition_failed);
    // 413 Payload Too Large
    trait_fn!(ok_or_payload_too_large);
    // 414 URI Too Long
    trait_fn!(ok_or_uri_too_long);
    // 415 Unsupported Media Type
    trait_fn!(ok_or_unsupported_media_type);
    // 416 Range Not Satisfiable
    trait_fn!(ok_or_range_not_satisfiable);
    // 417 Expectation Failed
    trait_fn!(ok_or_expectation_failed);
    // 418 I'm a teapot
    trait_fn!(ok_or_im_a_teapot);

    // 421 Misdirected Request
    trait_fn!(ok_or_misdirected_request);
    // 422 Unprocessable Entity
    trait_fn!(ok_or_unprocessable_entity);
    // 423 Locked
    trait_fn!(ok_or_locked);
    // 424 Failed Dependency
    trait_fn!(ok_or_failed_dependency);

    // 426 Upgrade Required
    trait_fn!(ok_or_upgrade_required);

    // 428 Precondition Required
    trait_fn!(ok_or_precondition_required);
    // 429 Too Many Requests
    trait_fn!(ok_or_too_many_requests);

    // 431 Request Header Fields Too Large
    trait_fn!(ok_or_request_header_fields_too_large);

    // 451 Unavailable For Legal Reasons
    trait_fn!(ok_or_unavailable_for_legal_reasons);

    // 500 Internal Server Error
    trait_fn!(ok_or_internal_server_error);
    // 501 Not Implemented
    trait_fn!(ok_or_not_implemented);
    // 502 Bad Gateway
    trait_fn!(ok_or_bad_gateway);
    // 503 Service Unavailable
    trait_fn!(ok_or_service_unavailable);
    // 504 Gateway Timeout
    trait_fn!(ok_or_gateway_timeout);
    // 505 HTTP Version Not Supported
    trait_fn!(ok_or_http_version_not_supported);
    // 506 Variant Also Negotiates
    trait_fn!(ok_or_variant_also_negotiates);
    // 507 Insufficient Storage
    trait_fn!(ok_or_insufficient_storage);
    // 508 Loop Detected
    trait_fn!(ok_or_loop_detected);

    // 510 Not Extended
    trait_fn!(ok_or_not_extended);
    // 511 Network Authentication Required
    trait_fn!(ok_or_network_authentication_required);
}
impl<T> OptionExt<T> for Option<T> {
    impl_fn!(ok_or_bad_request, BAD_REQUEST);
    impl_fn!(ok_or_unauthorized, UNAUTHORIZED);
    impl_fn!(ok_or_payment_required, PAYMENT_REQUIRED);
    impl_fn!(ok_or_forbidden, FORBIDDEN);
    impl_fn!(ok_or_not_found, NOT_FOUND);
    impl_fn!(ok_or_method_not_allowed, METHOD_NOT_ALLOWED);
    impl_fn!(ok_or_not_acceptable, NOT_ACCEPTABLE);
    impl_fn!(
        ok_or_proxy_authentication_required,
        PROXY_AUTHENTICATION_REQUIRED
    );
    impl_fn!(ok_or_request_timeout, REQUEST_TIMEOUT);
    impl_fn!(ok_or_conflict, CONFLICT);
    impl_fn!(ok_or_gone, GONE);
    impl_fn!(ok_or_length_required, LENGTH_REQUIRED);
    impl_fn!(ok_or_precondition_failed, PRECONDITION_FAILED);
    impl_fn!(ok_or_payload_too_large, PAYLOAD_TOO_LARGE);
    impl_fn!(ok_or_uri_too_long, URI_TOO_LONG);
    impl_fn!(ok_or_unsupported_media_type, UNSUPPORTED_MEDIA_TYPE);
    impl_fn!(ok_or_range_not_satisfiable, RANGE_NOT_SATISFIABLE);
    impl_fn!(ok_or_expectation_failed, EXPECTATION_FAILED);
    impl_fn!(ok_or_im_a_teapot, IM_A_TEAPOT);

    impl_fn!(ok_or_misdirected_request, MISDIRECTED_REQUEST);
    impl_fn!(ok_or_unprocessable_entity, UNPROCESSABLE_ENTITY);
    impl_fn!(ok_or_locked, LOCKED);
    impl_fn!(ok_or_failed_dependency, FAILED_DEPENDENCY);

    impl_fn!(ok_or_upgrade_required, UPGRADE_REQUIRED);

    impl_fn!(ok_or_precondition_required, PRECONDITION_REQUIRED);
    impl_fn!(ok_or_too_many_requests, TOO_MANY_REQUESTS);

    impl_fn!(
        ok_or_request_header_fields_too_large,
        REQUEST_HEADER_FIELDS_TOO_LARGE
    );

    impl_fn!(
        ok_or_unavailable_for_legal_reasons,
        UNAVAILABLE_FOR_LEGAL_REASONS
    );

    impl_fn!(ok_or_internal_server_error, INTERNAL_SERVER_ERROR);
    impl_fn!(ok_or_not_implemented, NOT_IMPLEMENTED);
    impl_fn!(ok_or_bad_gateway, BAD_GATEWAY);
    impl_fn!(ok_or_service_unavailable, SERVICE_UNAVAILABLE);
    impl_fn!(ok_or_gateway_timeout, GATEWAY_TIMEOUT);
    impl_fn!(ok_or_http_version_not_supported, HTTP_VERSION_NOT_SUPPORTED);
    impl_fn!(ok_or_variant_also_negotiates, VARIANT_ALSO_NEGOTIATES);
    impl_fn!(ok_or_insufficient_storage, INSUFFICIENT_STORAGE);
    impl_fn!(ok_or_loop_detected, LOOP_DETECTED);

    impl_fn!(ok_or_not_extended, NOT_EXTENDED);
    impl_fn!(
        ok_or_network_authentication_required,
        NETWORK_AUTHENTICATION_REQUIRED
    );
}

impl error::Error for StatusError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.context)
    }
}

impl HttpError for StatusError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    #[cfg(feature = "tracing")]
    fn span(&self) -> Option<&tracing::Span> {
        Some(&self.span)
    }
}
