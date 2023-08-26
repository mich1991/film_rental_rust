use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct GenericResponse<T, U>{
    status: String,
    data: T,
    message: U
}

impl<T, U> GenericResponse<T, U> {
    pub fn success(data: T, message: U) -> Self {
        Self {
            status: "Success".to_string(),
            message,
            data,
        }
    }
    pub fn error(data: T, message: U) -> Self {
        Self {
            status: "Error".to_string(),
            message,
            data,
        }
    }
}
