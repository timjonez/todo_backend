use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        Self {
            message,
        }
    }
}
