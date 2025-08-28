
// use actix_web::{web::{self, scope}};

// use crate::handlers::user_handlers::{add_or_update, delete, delete_list, get, get_all, save_list};

//     pub fn config(config: &mut web::ServiceConfig) {

//     config.service(

//         scope("/users")
//         .service(get_all)
//         .service(get)
//         .service(add_or_update)
//         .service(delete)
//         .service(delete_list)
//         .service(save_list)

//         // .route("/get_all", web::get().to(|| async {
//         //     HttpResponse::Ok().body("Get all users")
//         // })),
//     );
// }


#[macro_export]
macro_rules! base_route {
    ($module:ident) => {
        paste::paste! {
            {
                use crate::handlers::[<$module _handlers>]::{
                    get_all, get, add_or_update, delete, delete_list, save_list
                };
                use actix_web::web::scope;

                scope(concat!("/", stringify!($module), "s"))
                    .service(get_all)
                    .service(get)
                    .service(add_or_update)
                    .service(delete)
                    .service(delete_list)
                    .service(save_list)
            }
        }
    };
}
