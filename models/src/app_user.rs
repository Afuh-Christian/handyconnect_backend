use derive_columns_and_placeholders::ColumnsAndPlaceholders;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;



#[derive(Serialize , Deserialize , Debug , Clone , FromRow , ColumnsAndPlaceholders )]
 pub struct AppUser {
     pub id: Uuid,
     pub username: String,
    }


