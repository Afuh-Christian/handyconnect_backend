use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;




#[derive(Serialize , Deserialize , Debug , Clone , FromRow )]
 pub struct AppUser {
     pub app_user_id: uuid::Uuid,
     pub username: String,
 }