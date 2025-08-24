use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct Rating {
    pub id: Uuid,
    pub user_id: Uuid,
    pub handyman_id: Uuid,
    pub comment: Option<String>,
  pub created_at: NaiveDateTime,
    pub score: i32,
}