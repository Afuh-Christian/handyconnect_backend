


use actix_web::web;

use crate::routes::user_routes;

    pub fn app_routes_config(config : &mut web::ServiceConfig){
        // Define your route setup logic here
        config.configure(user_routes::config);
    }
