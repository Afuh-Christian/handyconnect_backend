use models::app_user::AppUser;
use uuid::Uuid;






fn main() {

    let app_user = AppUser {
        app_user_id: Uuid::new_v4(),
        username: "test_user".to_string(),
    };
 
    println!("{:?}", AppUser::column_names()); 
    // ["id", "name", "age"]

    println!("{:?}", app_user.values());
    // ["$1", "$2", "$3"]

}
