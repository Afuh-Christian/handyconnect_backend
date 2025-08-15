use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::app_user::AppUser;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct AppUserRepository {
    base: BaseRepo<AppUser, Uuid>,
}

impl AppUserRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<AppUser, Uuid>::new(db, "app_user", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(AppUserRepository, AppUser, Uuid);