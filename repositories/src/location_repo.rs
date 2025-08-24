use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::location::Location;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct LocationRepository {
    base: BaseRepo<Location, Uuid>,
}

impl LocationRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<Location, Uuid>::new(db, "handymen", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(LocationRepository, Location, Uuid);