use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};
use uuid::Uuid;
use types::column_place_holder_trait::ColumnsAndPlaceholdersTrait;
use derive_columns_and_placeholders::ColumnsAndPlaceholders;

#[derive(Serialize , Deserialize , Debug  , Clone , FromRow , ColumnsAndPlaceholders )]
pub struct Handyman {
    pub id: Uuid,
    pub app_user_id: Uuid,
    pub location_id: Uuid,
    pub profession_ids : Option<Json<Vec<u32>>>,
    pub ratings : Option<Json<Vec<RatingInfo>>>,
    pub contact_infos: Option<Json<Vec<RatingInfo>>>,
    pub payment_methods: Option<Json<Vec<PaymentMethod>>>
}


#[derive(Serialize, Deserialize, Debug, Clone )]
pub struct ContactInfo {
    pub type_id: u32,
    pub value: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RatingInfo {
    pub average: f32,
    pub reviews: u32,
}



#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PaymentMethod {
    Cash {
        // Cash usually just exists, no extra fields needed
        preferred: bool,
    },
    Card {
        card_number_last4: String, // last 4 digits for security
        cardholder_name: String,
        expiry_month: u8,
        expiry_year: u16,
        preferred: bool,
    },
    MobileMoney {
        provider_id: u32,       // references your provider table
        account_number: String,
        preferred: bool,
    },
    BankTransfer {
        bank_id: u32,           // references bank table
        account_number: String,
        account_name: String,
        preferred: bool,
    },
}
