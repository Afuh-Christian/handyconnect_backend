use actix_web::{HttpResponse, Responder};
use actix_web:: {get, post, web};



#[get("/")]
async fn get_all() -> impl Responder {


    


    HttpResponse::Ok().body("Hello world!")
}