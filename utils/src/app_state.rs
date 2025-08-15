use std::sync::Arc;
use repositories::app_user_repo::AppUserRepository;

pub struct AppState {
    pub app_user_repo : Arc<AppUserRepository>
}