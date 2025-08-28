use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;


#[derive(Serialize , Deserialize , Debug  , Clone , FromRow , ColumnsAndPlaceholders )]
 pub struct AppUser {

   
pub id: Uuid,
pub user_name: String,
pub normalized_user_name: Option<String>,


pub email: Option<String>,
pub normalized_email: Option<String>,
pub email_confirmed: bool,


pub password_hash: Option<String>,
pub security_stamp: Option<String>,
pub concurrency_stamp: Option<Uuid>,


pub phone_number: Option<String>,
pub phone_number_confirmed: bool,


pub two_factor_enabled: bool,


pub lockout_end: Option<NaiveDateTime>,
pub lockout_enabled: bool,
pub access_failed_count: i32,


pub created_at: NaiveDateTime,
pub updated_at: NaiveDateTime,


    }

