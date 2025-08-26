use std::sync::Arc;
use actix_web::{ web, App, HttpServer};
use database::postgres::db_postgres_pool;
use repositories::{app_user_repo::AppUserRepository, handyman_repo::HandymanRepository, location_repo::LocationRepository, lookup_data_repo::LookupDataRepository, lookup_table_repo::LookupTableRepository, rating_repo::RatingRepository, transaction_repo::TransactionRepository};
use utils::app_state::AppState;
mod routes;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let port: u16 = (*utils::constants::PORT).clone();
    let address: String = (*utils::constants::ADDRESS).clone();

    let pool =  db_postgres_pool().await;
    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{
                app_user_repo : Arc::new(AppUserRepository::new(pool.clone())),
                handyman_repo : Arc::new(HandymanRepository::new(pool.clone())),
                location_repo : Arc::new(LocationRepository::new(pool.clone())),
                lookup_data_repo : Arc::new(LookupDataRepository::new(pool.clone())),
                lookup_table_repo : Arc::new(LookupTableRepository::new(pool.clone())),
                rating_repo : Arc::new(RatingRepository::new(pool.clone())),
                transaction_repo : Arc::new(TransactionRepository::new(pool.clone())),
            })) // Pass pool as shared state
            .configure(routes::app_routes::app_routes_config)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
