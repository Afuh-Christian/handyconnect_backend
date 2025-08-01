

use actix_web::{web::{self, scope}, HttpResponse};

use crate::handlers::user_handlers::get_all;

    pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        scope("/users")
        .service(get_all)
        .route("/get_all", web::get().to(|| async {
            HttpResponse::Ok().body("Get all users")
        })),
    );
}



 
 