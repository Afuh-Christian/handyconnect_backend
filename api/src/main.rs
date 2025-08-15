use std::sync::Arc;
use actix_web::{ web, App, HttpServer};
use database::postgres::db_postgres_pool;
use repositories::app_user_repo::AppUserRepository;
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
                app_user_repo : Arc::new(AppUserRepository::new(pool.clone()))
            })) // Pass pool as shared state
            .configure(routes::app_routes::app_routes_config)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
