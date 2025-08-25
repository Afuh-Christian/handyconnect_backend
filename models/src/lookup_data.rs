use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct LookupData {
    pub lookup_data_id: i32, //pk
    pub lookup_table_id: i32, // fk ... 
    pub language_lookup_data_id: i32, // dual pk 
    pub lookup_data_name : String,
    pub description : String,
}


#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct LookupDataView {
    pub lookup_table_id: i32, 
    pub lookup_data_id: i32,
    pub lookup_data_name: String,
    pub description: String,
}