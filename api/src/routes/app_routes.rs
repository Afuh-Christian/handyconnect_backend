


use actix_web::web;

use crate::routes::{handyman_routes, location_routes, lookup_data_routes, lookup_table_routes, rating_routes, transaction_routes, user_routes};

    pub fn app_routes_config(config : &mut web::ServiceConfig){
        // Define your route setup logic here
        config
        .configure(user_routes::config)
        .configure(location_routes::config)
        .configure(transaction_routes::config)
        .configure(handyman_routes::config)
        .configure(rating_routes::config)
        .configure(lookup_data_routes::config)
        .configure(lookup_table_routes::config)
        

        
        ;
    }
