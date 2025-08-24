use crate::{base_repo::BaseRepo, delegate_base_repo};
use models::transaction::Transaction;
use sqlx::PgPool;
use uuid::Uuid;
// use crate::delegate_base_repo;


pub struct TransactionRepository {
    base: BaseRepo<Transaction, Uuid>,
}

impl TransactionRepository {
    pub fn new(db: PgPool) -> Self {
        let base = BaseRepo::<Transaction, Uuid>::new(db, "transactions", "id");
        Self { base }
    }
}

// Delegate BaseRepoTrait methods using the macro
delegate_base_repo!(TransactionRepository, Transaction, Uuid);