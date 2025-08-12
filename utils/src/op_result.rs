use serde::{Deserialize, Serialize};


#[derive(Debug , Deserialize , Serialize , Clone)]
pub struct OperationResult {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}


impl OperationResult {
     pub  fn default() -> Self {
        Self {
            success: true,                       // default to success
            message: Some("Operation completed".to_string()),
            error: None,
        }
    }
    pub fn success(message: &str) -> Self {
        Self {
            success: true,
            message: Some(message.to_string()),
            error: None,
        }
    }

    pub fn failure(error: &str) -> Self {
        Self {
            success: false,
            message: None,
            error: Some(error.to_string()),
        }
    }
}
