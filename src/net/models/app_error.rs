use axum::{ extract::Json, http::StatusCode };
use log::error;

use crate::net::models::error_json::ErrorJson;

pub type ErrorResponse = (StatusCode, Json<ErrorJson>);

pub struct AppError {
    status: StatusCode,
    err: ErrorJson,
}

impl AppError {
    pub fn new<S: Into<String>>(status: StatusCode, message: S) -> AppError {
        Self {
            status: status,
            err: ErrorJson::new(message.into()),
        }
    }

    pub fn to_response(&self) -> ErrorResponse {
        self.print_error();
        return (self.status, Json(self.err.clone()));
    }

    pub fn as_response<S: Into<String>>(status: StatusCode, message: S) -> ErrorResponse {
        return AppError::new(status, message.into()).to_response();
    }

    pub fn print_error(&self) {
        error!("Request failed with code {} and message: \"{}\"", self.status, self.err.message);
    }
}
