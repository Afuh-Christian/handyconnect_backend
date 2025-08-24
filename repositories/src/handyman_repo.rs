use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::handyman::Handyman;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct HandymanRepository {
    base: BaseRepo<Handyman, Uuid>,
}

impl HandymanRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<Handyman, Uuid>::new(db, "handymen", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(HandymanRepository, Handyman, Uuid);