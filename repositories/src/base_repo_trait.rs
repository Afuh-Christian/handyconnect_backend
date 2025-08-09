use async_trait::async_trait;
use sqlx::FromRow;
use utils::api_errors::ApiError;

#[async_trait]
pub trait BaseRepoTrait<T, IdType>
{
// async  fn add_or_update(&self, item: T) -> Result<T, ApiError>;
async fn get_by_id<'a>(
    &self,
    id: IdType
) -> Result<T, ApiError>;
}
