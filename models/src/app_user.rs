use serde::{Deserialize, Serialize};




#[derive(Serialize , Deserialize , Debug , Clone)]
 pub struct AppUser {
     pub app_user_id: uuid::Uuid,
     pub username: String,
 }