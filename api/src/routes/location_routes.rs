
use actix_web::{web::{self}, HttpResponse};

use crate::{base_route};

    pub fn config(config: &mut web::ServiceConfig) {

    config.service(
        base_route!(location)
        .route("/hey", web::get().to(|| async {
            HttpResponse::Ok().body("Get all locations")
        })),


    );
}



 
 