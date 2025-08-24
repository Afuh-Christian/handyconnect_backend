use std::sync::Arc;
use repositories::{app_user_repo::AppUserRepository, handyman_repo::HandymanRepository, location_repo::LocationRepository, lookup_data_repo::LookupDataRepository, lookup_table_repo::LookupTableRepository};

pub struct AppState {
    pub app_user_repo : Arc<AppUserRepository>,
    pub handyman_repo : Arc<HandymanRepository>,
    pub location_repo : Arc<LocationRepository>,
    pub lookup_data_repo : Arc<LookupDataRepository>,
    pub lookup_table_repo : Arc<LookupTableRepository>,
}