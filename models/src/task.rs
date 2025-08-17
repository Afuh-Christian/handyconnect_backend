use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;
use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub handyman_id: Uuid,
    pub profession_id: Uuid, // Now references a profession instead of a specific job
    pub description: Option<String>,
    pub cost: f64,
    pub status: String,
    pub completed_at: DateTime<Utc>,
}