#[macro_export]
macro_rules! base_handler {
    ($module:ident, $repo:ident, $model_dto:ident,$id_type:ty) => {
        use actix_web::{ web::{Data , Path , Json}  , http::StatusCode ,get , post, delete};
        use types::{api_errors::ApiError, op_result::OperationResult};
        use utils::{ app_state::AppState };
        use models::$module::$model_dto;
        use paste::paste;
        use repositories::base_repo_trait::BaseRepoTrait;


         #[get("/get_all")]
        pub async fn get_all(
            app_state: Data<AppState>,
        ) ->  Result<Json<Vec<$model_dto>> , ApiError> {
         let items =  paste!{ app_state.[<$module _repo>] }
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
            let item = paste!{ app_state.[<$module _repo>] }
                .add_or_update(payload.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(item))
        }


        #[get("/get/{id}")]
        pub async fn get(
            app_state: Data<AppState>,
            id: Path<$id_type>,
        ) -> Result<Json<$model_dto> , ApiError>  {
            let result = paste!{ app_state.[<$module _repo>] }
                .get(id.into_inner())
                .await
                .map_err(|err| ApiError {
                    message: err.message.to_owned(),
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error_code: Some(04),
                })?;
            Ok(Json(result))
        }


        #[delete("/delete/{id}")]
        pub async fn delete(
            app_state: Data<AppState>,
            id: Path<$id_type>,
        ) -> Result<Json<OperationResult> , ApiError>  {
            let result = paste!{ app_state.[<$module _repo>] }
                .delete(id.into_inner())
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
            let result = paste!{ app_state.[<$module _repo>] }
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
            payload: Json<Vec<$id_type>>,
        ) -> Result<Json<OperationResult>, ApiError> {
              let result = paste!{ app_state.[<$module _repo>] }
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