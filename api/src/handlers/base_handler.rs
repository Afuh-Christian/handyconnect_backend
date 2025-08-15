#[macro_export]
macro_rules! base_handler {
    ($module:ident, $repo:ident, $model_dto:ident) => {
        use actix_web::{ web::{Data , Path , Json}  , http::StatusCode ,get , post, delete};
        use uuid::Uuid;
        use utils::{api_errors::ApiError, app_state::AppState , op_result::OperationResult};
        use models::$module::$model_dto;
        use repositories::base_repo_trait::BaseRepoTrait;
       paste::paste! {
         use repositories::[<$module _repo>]::$repo;
       }
        

         #[get("/get_all")]
        pub async fn get_all(
            app_state: Data<AppState>,
        ) ->  Result<Json<Vec<$model_dto>> , ApiError> {
            let items = $repo::new(app_state.db_pool.clone())
                .get_all()
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(items))
        }
        #[post("/add_or_update")]
        pub async fn add_or_update(
            app_state: Data<AppState>,
            payload: Json<$model_dto>,
        ) ->  Result<Json<$model_dto> , ApiError> {
            let item = $repo::new(app_state.db_pool.clone())
                .add_or_update(payload.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(item))
        }


        #[get("/get/{uuid}")]
        pub async fn get(
            app_state: Data<AppState>,
            uuid: Path<Uuid>,
        ) -> Result<Json<$model_dto> , ApiError>  {
            let result = $repo::new(app_state.db_pool.clone())
                .get(uuid.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(result))
        }


        #[delete("/delete/{uuid}")]
        pub async fn delete(
            app_state: Data<AppState>,
            uuid: Path<Uuid>,
        ) -> Result<Json<OperationResult> , ApiError>  {
            let result = $repo::new(app_state.db_pool.clone())
                .delete(uuid.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(result))
        }

        #[post("/save_list")]
        pub async fn save_list(
            app_state: Data<AppState>,
            payload: Json<Vec<$model_dto>>,
        ) -> Result<Json<OperationResult> , ApiError> {
            let result = $repo::new(app_state.db_pool.clone())
                .save_list(payload.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(result))
        }

        #[post("/delete_list")]
        pub async fn delete_list(
            app_state: Data<AppState>,
            payload: Json<Vec<Uuid>>,
        ) -> Result<Json<OperationResult>, ApiError> {
              let result = $repo::new(app_state.db_pool.clone())
                .delete_list(payload.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(result))
        }
    };
}