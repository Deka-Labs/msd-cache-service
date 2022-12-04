use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseError {
    message: String,
}

impl ResponseError {
    pub fn new(error_msg: String) -> Self {
        Self { message: error_msg }
    }
}
