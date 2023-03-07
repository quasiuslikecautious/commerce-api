use axum::Json;
use http::StatusCode;
use serde::Serialize;

pub type NonceResponse = (StatusCode, Json<NoncePayload>);

#[derive(Serialize)]
pub struct NoncePayload {
    pub nonce: String,
}

impl NoncePayload {
    pub fn new(nonce: String) -> Self {
        Self {
            nonce: nonce,
        }
    }

    pub fn as_response<S: Into<String>>(nonce: S) -> NonceResponse {
        (StatusCode::UNAUTHORIZED, Json::from(Self::new(nonce.into())))
    }
}
