use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct ApiError {
    pub message: String,
    pub error_code: Option<i8>,
    pub status_code: StatusCode,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Optional: Use this for automatic response formatting
impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code)
            .content_type("application/json")
            .json(json!({
                "StatusCode": self.status_code.as_u16(),
                "ErrorCode": self.error_code,
                "Message": self.message,
            }))
    }
}
