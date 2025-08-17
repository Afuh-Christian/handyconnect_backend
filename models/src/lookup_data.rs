use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;

#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct LookupData {
    pub lookup_table_id: i32,
    pub lookup_data_id: i32,
    pub lookup_data_name_by_languages : String,
}


#[derive(Serialize, Deserialize, Debug, Clone, FromRow, ColumnsAndPlaceholders)]
pub struct LookupDataView {
    pub lookup_table_id: i32,
    pub lookup_data_id: i32,
    pub lookup_data_name: String
}
