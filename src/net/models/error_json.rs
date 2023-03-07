use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ErrorJson {
    pub message: String,
}

impl ErrorJson {
    pub fn new(message: String) -> ErrorJson {
        Self {
            message: message,
        }
    }
}
