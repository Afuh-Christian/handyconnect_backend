use serde::{Deserialize, Serialize};


#[derive(Debug , Deserialize , Serialize , Clone)]
pub struct OperationResult {
    pub success: bool,
    pub message: Option<String>,
    pub error: Option<String>,
}
