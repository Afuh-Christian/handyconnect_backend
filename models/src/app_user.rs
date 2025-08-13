use derive_columns_and_placeholders::ColumnsAndPlaceholders;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;




#[derive(Serialize , Deserialize , Debug , Clone , FromRow , ColumnsAndPlaceholders )]
 pub struct AppUser {
     pub app_user_id: uuid::Uuid,
     pub username: String,
    }


