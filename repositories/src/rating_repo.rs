use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::rating::Rating;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct RatingRepository {
    base: BaseRepo<Rating, Uuid>,
}

impl RatingRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<Rating, Uuid>::new(db, "rating", "user_id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(RatingRepository, Rating, Uuid);