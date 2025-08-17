use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct Transaction {
    pub transaction_id: Uuid,
    pub transaction_number: String,
    pub sender_id: Option<Uuid>,
    pub recipient_id: Option<Uuid>,
    pub amount: f64,
    pub platform_fee: Option<f64>,
    pub recipient_earnings: Option<f64>,
    pub currency_id: i32,
    pub transaction_status_id: i32,
    pub payment_method_id: i32,
    pub provider_id: Option<i32>,
    pub reference: String,
    pub payer_account: Option<String>,
    pub recipient_account: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}