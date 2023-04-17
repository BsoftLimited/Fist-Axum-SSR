use std::fmt::{Formatter, Display};

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ErrorHandler {
    status_code: u16,
    errors: Vec<String>,
}

impl Display for ErrorHandler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Err {} ",&self.status_code).to_string())
    }
}

impl ErrorHandler {
    pub fn new(status_code: u16, err: String) -> Self {
        let mut errors: Vec<String> = Vec::new();
        errors.push(err);
        ErrorHandler { status_code: status_code, errors: errors }
    }

    pub fn new_internal(err: String) -> Self {
        let mut errors: Vec<String> = Vec::new();
        errors.push(err);
        ErrorHandler { status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), errors: errors }
    }
    pub fn new_bad_request(err: String) -> Self {
        let mut errors: Vec<String> = Vec::new();
        errors.push(err);
        ErrorHandler { status_code: StatusCode::BAD_REQUEST.as_u16(), errors: errors }
    }

    pub fn append_error(&mut self, err: String) {
        self.errors.push(err);
    }
}

impl IntoResponse for ErrorHandler {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status_code).unwrap(), serde_json::to_string(&self).unwrap()).into_response()
    }
}