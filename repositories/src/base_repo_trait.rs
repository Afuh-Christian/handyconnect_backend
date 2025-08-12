use async_trait::async_trait;
use utils::{api_errors::ApiError, op_result::OperationResult};

#[async_trait]
pub trait BaseRepoTrait<T, IdType> {
    async fn add_or_update(&self, item: T) -> Result<T, ApiError>;
    async fn get(&self, id: IdType) -> Result<T, ApiError>;
    async fn get_all(&self) -> Result<Vec<T>, ApiError>;
    async fn delete(&self, id: IdType) -> Result<OperationResult, ApiError>;
    async fn delete_list(&self, ids: Vec<IdType>) -> Result<OperationResult, ApiError>;
    async fn save_list(&self, items: Vec<T>) -> Result<OperationResult, ApiError>;
}
