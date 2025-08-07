use custom_macros::FieldNames;
use serde::{Deserialize, Serialize};




#[derive(Serialize , Deserialize , Debug , Clone , FieldNames)]
 pub struct AppUser {
     pub app_user_id: uuid::Uuid,
     pub username: String,
 }