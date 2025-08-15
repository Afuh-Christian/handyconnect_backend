use std::marker::PhantomData;
use actix_web::http::StatusCode;
use types::{column_place_holder_trait::ColumnsAndPlaceholdersTrait};
use sqlx::{ FromRow, PgPool };
use utils::{ api_errors::ApiError, op_result::OperationResult };
use async_trait::async_trait;
use crate::base_repo_trait::BaseRepoTrait;

pub struct BaseRepo<T, IdType> {
    table_name: &'static str,
    id_name: &'static str,
    db_pool: PgPool,
    _id: PhantomData<IdType>,
    _model: PhantomData<T>,
}

impl<T, IdType> BaseRepo<T, IdType>
    where
        T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin +  ColumnsAndPlaceholdersTrait,
        IdType: Send +Sync + for<'r> sqlx::Encode<'r, sqlx::Postgres> +sqlx::Type<sqlx::Postgres> +Copy +Clone
{
    pub fn new(db_pool: PgPool, table_name: &'static str, id_name: &'static str) -> Self {
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
impl<T, IdType> BaseRepoTrait<T, IdType>
    for BaseRepo<T, IdType>
                 where
        T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin +  ColumnsAndPlaceholdersTrait,
        IdType: Send +Sync + for<'r> sqlx::Encode<'r, sqlx::Postgres> +sqlx::Type<sqlx::Postgres> +Copy +Clone
{
    async fn get(&self, id: IdType) -> Result<T, ApiError>
        // where for<'q> &'q IdType: sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>
    {
        let data = sqlx
            ::query_as::<_, T>(
                format!("SELECT * FROM {} WHERE {} = $1", self.table_name, self.id_name).as_str()
            )
            .bind(&id) // reference here
            .fetch_one(&self.db_pool).await
            .map_err(|e| ApiError {
                message: "Error".to_owned(),
                error_code: Some(44),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
        Ok(data)
    }

    async fn add_or_update(&self, item: T) -> Result<T, ApiError> {
        let columns = T::column_names();
        let placeholders = T::placeholders();
        let values = item.values();

        // Build the update assignments: col1 = EXCLUDED.col1, col2 = EXCLUDED.col2, ...
        let update_assignments: Vec<String> = columns
            .iter()
            .map(|col| format!(r#""{}" = EXCLUDED."{}""#, col, col))
            .collect();

        // UPSERT query
        let query_str = format!(
            r#"
        INSERT INTO {} ({})
        VALUES ({})
        ON CONFLICT ({}) DO UPDATE
        SET {}
        "#,
            self.table_name,
            columns.join(", "),
            placeholders.join(", "),
            self.id_name,
            update_assignments.join(", ")
        );

        let mut query = sqlx::query(&query_str);

        // Bind all values in order
        for val in values {
            query = query.bind(val);
        }

        query.execute(&self.db_pool).await.map_err(|_| ApiError {
            message: "Error".to_owned(),
            error_code: Some(44),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

        Ok(item)
    }

    async fn get_all(&self) -> Result<Vec<T>, ApiError> {
        let data = sqlx
            ::query_as::<_, T>(format!("SELECT * FROM {}", self.table_name).as_str())
            .fetch_all(&self.db_pool).await
            .map_err(|e| ApiError {
                message: "Error".to_owned(),
                error_code: Some(44),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
        Ok(data)
    }

    async fn delete(&self, id: IdType) -> Result<OperationResult, ApiError> {
        sqlx
            ::query(format!("DELETE FROM {} WHERE {} = $1", self.table_name, self.id_name).as_str())
            .bind(&id)
            .execute(&self.db_pool).await
            .map_err(|_| ApiError {
                message: "Error".to_owned(),
                error_code: Some(44),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            })?;
        Ok(OperationResult::default())
    }




async fn delete_list(&self, ids: Vec<IdType>) -> Result<OperationResult, ApiError> {
    if ids.is_empty() {
        return Ok(OperationResult::default());
    }

    let placeholders: Vec<String> = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("${}", i + 1))
        .collect();

    let query_str = format!(
        "DELETE FROM {} WHERE {} IN ({})",
        self.table_name,
        self.id_name,
        placeholders.join(", ")
    );

    let mut query = sqlx::query(&query_str);
    for id in &ids {
        query = query.bind(id);
    }

    query
        .execute(&self.db_pool)
        .await
        .map_err(|_| ApiError {
            message: "Error".to_owned(),
            error_code: Some(44),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(OperationResult::default())
}







    async fn save_list(&self, items: Vec<T>) -> Result<OperationResult, ApiError> {
        if items.is_empty() {
            return Ok(OperationResult::default());
        }

        let columns = T::column_names();
        let placeholders = T::placeholders();

        // Build the update assignments: col1 = EXCLUDED.col1, col2 = EXCLUDED.col2, ...
        let update_assignments: Vec<String> = columns
            .iter()
            .map(|col| format!(r#""{}" = EXCLUDED."{}""#, col, col))
            .collect();

        // UPSERT query
        let query_str = format!(
            r#"
        INSERT INTO {} ({})
        VALUES {}
        ON CONFLICT ({}) DO UPDATE
        SET {}
        "#,
            self.table_name,
            columns.join(", "),
            items
                .iter()
                .map(|_| placeholders.join(", "))
                .collect::<Vec<_>>()
                .join(", "),
            self.id_name,
            update_assignments.join(", ")
        );

        let mut query = sqlx::query(&query_str);

        for item in items {
            for val in item.values() {
                query = query.bind(val);
            }
        }

        query.execute(&self.db_pool).await.map_err(|_| ApiError {
            message: "Error".to_owned(),
            error_code: Some(44),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

        Ok(OperationResult::default())
    }
}
