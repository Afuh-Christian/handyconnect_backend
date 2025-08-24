// use actix_web::{HttpResponse, Responder};
// use actix_web:: {get, post};

use uuid::Uuid;

use crate::base_handler; 
base_handler!( location , LocationRepository, Location , Uuid );













