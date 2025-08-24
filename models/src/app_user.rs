use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;


#[derive(Serialize , Deserialize , Debug  , Clone , FromRow , ColumnsAndPlaceholders )]
 pub struct AppUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub phone_numbers: Option<String>,
    pub password: String,
    pub role_ids: String
    }

