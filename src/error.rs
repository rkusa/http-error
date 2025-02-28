use std::borrow::Cow;

use http::StatusCode;

use crate::option_ext::StatusError;

macro_rules! impl_fn {
    ($name:ident, $status:ident) => {
        pub fn $name(context: impl Into<Cow<'static, str>>) -> StatusError {
            StatusError::new(StatusCode::$status, context)
        }
    };
}

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
