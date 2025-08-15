use crate::{base_repo::BaseRepo, base_repo_trait::BaseRepoTrait};
use utils::{api_errors::ApiError, op_result::OperationResult};
use models::app_user::AppUser;
use sqlx::PgPool;
use uuid::Uuid;
use crate::delegate_base_repo;


pub struct AppUserRepository {
    base: BaseRepo<AppUser, Uuid>,
}

impl AppUserRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<AppUser, Uuid>::new(db, "app_user".to_string(), "id".to_string());
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(AppUserRepository, AppUser, Uuid);