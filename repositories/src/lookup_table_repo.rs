use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::lookup_table::LookupTable;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct LookupTableRepository {
    base: BaseRepo<LookupTable, Uuid>,
}

impl LookupTableRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<LookupTable, Uuid>::new(db, "lookupTable", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(LookupTableRepository, LookupTable, Uuid);