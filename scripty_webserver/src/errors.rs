use axum::{
    body,
    http::{self, header::CONTENT_TYPE, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::{Display, Formatter};

/// All possible errors that can occur when handling a request.
#[derive(Debug)]
pub enum WebServerError {
    /// Token provided in the request was invalid.
    ///
    /// Code `1`, sub-code is the inner integer of this variant.
    ///
    /// Sub-code `1`: No token was provided in the `Authorization` header.
    /// Sub-code `2`: The token was not valid UTF-8.
    /// Sub-code `3`: The token was not a valid token.
    AuthenticationFailed(u32),
}

impl Display for WebServerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            WebServerError::AuthenticationFailed(_) => write!(f, "Authentication failed"),
        }
    }
}

/// Private helper type for error serialization
#[derive(Serialize)]
struct ErrorJson {
    code: u32,
    sub_code: u32,
}

impl IntoResponse for WebServerError {
    fn into_response(self) -> Response {
        warn!("web server error: {:?}", self);

        let (body, code) = match self {
            WebServerError::AuthenticationFailed(sub_code) => {
                (ErrorJson { code: 1, sub_code }, StatusCode::FORBIDDEN)
            }
        };

        let bytes = match simd_json::to_vec(&body) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!("Error serializing error response: {}", e);
                return Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .header(CONTENT_TYPE, HeaderValue::from_static("text/plain"))
                    .body(body::boxed(body::Full::from(e.to_string())))
                    .expect("failed to convert static data to a valid request");
            }
        };

        Response::builder()
            .status(code)
            .body(body::boxed(body::Full::from(bytes)))
            .expect("json serialization returned invalid data")
    }
}
