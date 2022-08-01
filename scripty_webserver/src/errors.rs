use axum::{
    body,
    http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
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
    AuthenticationFailed(i32),

    /// Bot cache was unavailable at request time.
    ///
    /// Code `2`, no sub-code.
    CacheUnavailable,

    /// The database returned an error.
    ///
    /// Code `3`, no sub-code.
    DatabaseError,
}

impl From<scripty_commands::CacheNotInitializedError> for WebServerError {
    fn from(_: scripty_commands::CacheNotInitializedError) -> Self {
        WebServerError::CacheUnavailable
    }
}

impl From<sqlx::Error> for WebServerError {
    fn from(_: sqlx::Error) -> Self {
        WebServerError::DatabaseError
    }
}

impl Display for WebServerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            WebServerError::AuthenticationFailed(_) => write!(f, "Authentication failed"),
            WebServerError::CacheUnavailable => write!(f, "Cache unavailable"),
            WebServerError::DatabaseError => write!(f, "Database error"),
        }
    }
}

/// Private helper type for error serialization
#[derive(Serialize)]
struct ErrorJson {
    code: u32,
    /// -1 means no sub-code
    sub_code: i32,
}

impl IntoResponse for WebServerError {
    fn into_response(self) -> Response {
        warn!("web server error: {:?}", self);

        let (body, code) = match self {
            WebServerError::AuthenticationFailed(sub_code) => {
                (ErrorJson { code: 1, sub_code }, StatusCode::FORBIDDEN)
            }
            WebServerError::CacheUnavailable => (
                ErrorJson {
                    code: 2,
                    sub_code: -1,
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
            WebServerError::DatabaseError => (
                ErrorJson {
                    code: 3,
                    sub_code: -1,
                },
                StatusCode::INTERNAL_SERVER_ERROR,
            ),
        };

        let bytes = match serde_json::to_vec(&body) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!("Error serializing error response: {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
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
