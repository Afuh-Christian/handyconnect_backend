use async_trait::async_trait;
use sqlx::FromRow;
use types::{column_place_holder_trait::ColumnsAndPlaceholdersTrait , api_errors::ApiError, op_result::OperationResult};

#[async_trait]
pub trait BaseRepoTrait<T, IdType> 
    where
        T : Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin +  ColumnsAndPlaceholdersTrait ,
        IdType : Send + Sync + for<'r> sqlx::Encode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres> + Copy + Clone
{
    async fn add_or_update(&self, item: T) -> Result<T, ApiError>;
    async fn get(&self, id: IdType) -> Result<T, ApiError>;
    async fn get_all(&self) -> Result<Vec<T>, ApiError>;
    async fn delete(&self, id: IdType) -> Result<OperationResult, ApiError>;
    async fn delete_list(&self, ids: Vec<IdType>) -> Result<OperationResult, ApiError>;
    async fn save_list(&self, items: Vec<T>) -> Result<OperationResult, ApiError>;
}


#[macro_export]
macro_rules! delegate_base_repo {
    ($target:ident, $view:ty, $primary:ty) => {
        use types::{ api_errors::ApiError, op_result::OperationResult};
        #[async_trait::async_trait]
        impl crate::base_repo_trait::BaseRepoTrait<$view, $primary> for $target {
           async fn add_or_update(&self, item: $view) -> Result<$view, ApiError> {
                self.base.add_or_update(item).await
            }

            async fn get(&self, id: $primary) -> Result<$view, ApiError> {
                self.base.get(id).await
            }

            async fn get_all(&self) -> Result<Vec<$view>, ApiError> {
                self.base.get_all().await
            }

            async fn delete(&self, id: $primary) -> Result<OperationResult, ApiError> {
                self.base.delete(id).await
            }

            async fn delete_list(&self, ids: Vec<$primary>) -> Result<OperationResult, ApiError> {
                self.base.delete_list(ids).await
            }

            async fn save_list(&self, items: Vec<$view>) -> Result<OperationResult, ApiError> {
                self.base.save_list(items).await
            }
        }
    };
}