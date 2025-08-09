// use std::marker::PhantomData;
// use actix_web::http::StatusCode;
// use sqlx::{FromRow, PgPool};
// use utils::api_errors::ApiError;
// use async_trait::async_trait;
// use crate::base_repo_trait::BaseRepoTrait;

// pub struct BaseRepo<T, IdType> {
//     table_name: String,
//     id_name: String,
//     db_pool: PgPool,
//     _id: PhantomData<IdType>,
//     _model: PhantomData<T>,
// }

// impl<T, IdType> BaseRepo<T, IdType>
// where
//     T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin,
// {
//     pub fn new(db_pool: PgPool, table_name: String,id_name: String) -> Self {
//         Self {
//             db_pool,
//             table_name,
//             id_name,
//             _id: PhantomData,
//             _model: PhantomData,
//         }
//     }
// }





// #[async_trait]
// impl<T, IdType> BaseRepoTrait<T, IdType> for BaseRepo<T, IdType>
// {
   
// fn add_or_update(&self, item: T) -> Result<T, ApiError>{


//     // Implement the logic to add or update the item in the database
//     // This is a placeholder implementation
//     Ok(item)
// }
   
// }






use std::marker::PhantomData;
use actix_web::http::StatusCode;
use sqlx::{FromRow, PgPool, postgres::PgQueryResult};
use utils::api_errors::ApiError;
use async_trait::async_trait;
use crate::base_repo_trait::BaseRepoTrait;

pub trait HasId<IdType> {
    fn id(&self) -> &IdType;
}

pub struct BaseRepo<T, IdType> {
    table_name: String,
    id_name: String,
    db_pool: PgPool,
    _id: PhantomData<IdType>,
    _model: PhantomData<T>,
}

impl<T, IdType> BaseRepo<T, IdType>
where
    T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin,
{
    pub fn new(db_pool: PgPool, table_name: String, id_name: String) -> Self {
        Self {
            db_pool,
            table_name,
            id_name,
            _id: PhantomData,
            _model: PhantomData,
        }
    }
}

#[async_trait]
impl<T, IdType> BaseRepoTrait<T, IdType> for BaseRepo<T, IdType>
where
    T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + HasId<IdType> , 
    IdType: Send + Sync +  for<'r> sqlx::Encode<'r, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>, 

    // IdType: Send + Sync + sqlx::Type<sqlx::Postgres> + Clone + Unpin,
{

async fn get_by_id<'a>(
    &self,
    id: IdType
) -> Result<T, ApiError>
where
    for<'q> &'q IdType: sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>
{
   let data =  sqlx::query_as::<_, T>(format!(
        "SELECT * FROM {} WHERE {} = $1", self.table_name, self.id_name
    ).as_str())
        .bind(&id) // reference here
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| ApiError { message: "Error".to_owned(), error_code: Some(44), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;


    Ok(data)

}



    // async fn add_or_update(&self, item: T) -> Result<T, ApiError> {
    //     let id_value = item.id().clone();

    //     let exists = sqlx::query_scalar::<_, bool>(&format!(
    //         "SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1)",
    //         self.table_name, self.id_name
    //     ))
    //     .bind(&id_value)
    //     .fetch_one(&self.db_pool)
    //     .await
    //     .map_err(|e| ApiError { message: "Error".to_owned(), error_code: Some(44), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;

    //     if exists {
    //         // Update logic (this example assumes you fill the fields manually)
    //         sqlx::query(&format!(
    //             "UPDATE {} SET /* your fields here */ WHERE {} = $1",
    //             self.table_name, self.id_name
    //         ))
    //         .bind(&id_value)
    //         .execute(&self.db_pool)
    //         .await
    //     .map_err(|e| ApiError { message: "Error".to_owned(), error_code: Some(44), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;
    //     } else {
    //         // Insert logic (you'll have to bind each field from `item`)
    //         sqlx::query(&format!(
    //             "INSERT INTO {} (/* columns */) VALUES (/* values */)",
    //             self.table_name
    //         ))
    //         .execute(&self.db_pool)
    //         .await
    //     .map_err(|e| ApiError { message: "Error".to_owned(), error_code: Some(44), status_code: StatusCode::INTERNAL_SERVER_ERROR })?;
    //     }

    //     Ok(item)
    // }
}
