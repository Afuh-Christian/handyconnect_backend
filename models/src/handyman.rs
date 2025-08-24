use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;


#[derive(Serialize , Deserialize , Debug  , Clone , FromRow , ColumnsAndPlaceholders )]
pub struct Handyman {
    pub id: Uuid,
    pub app_user_id: Uuid,
    pub location_id: Uuid,
    pub profession_ids : Option<String>,
    pub contact_infos: Option<String>,
    pub payment_method_ids: Option<String>
}

    


