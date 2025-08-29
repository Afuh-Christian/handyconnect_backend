use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::lookup_data::LookupData;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct LookupDataRepository {
    base: BaseRepo<LookupData, Uuid>,
}

impl LookupDataRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<LookupData, Uuid>::new(db, "lookup_data", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(LookupDataRepository, LookupData, Uuid);