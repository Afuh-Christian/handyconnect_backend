use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use database::postgres::db_postgres_pool;
use utils::app_state::AppState;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


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
            .app_data(web::Data::new(AppState{db_pool : pool.clone()})) // Pass pool as shared state
            .configure(routes::app_routes::app_routes_config)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
