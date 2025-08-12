use std::marker::PhantomData;
use actix_web::http::StatusCode;
use sqlx::{ FromRow, PgPool, postgres::PgQueryResult };
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
        IdType: Send +
            Sync +
            for<'r> sqlx::Encode<'r, sqlx::Postgres> +
            sqlx::Type<sqlx::Postgres> +
            Copy +
            Clone
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
impl<T, IdType> BaseRepoTrait<T, IdType>
    for BaseRepo<T, IdType>
    where
        T: Send + Sync + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Unpin + HasId<IdType> + ColumnsAndPlaceholders,
        IdType: Send +
            Sync +
            for<'r> sqlx::Encode<'r, sqlx::Postgres> +
            sqlx::Type<sqlx::Postgres> +
            Copy +
            Clone


            
        // IdType: Send + Sync + sqlx::Type<sqlx::Postgres> + Clone + Unpin,
{
    async fn get_by_id(&self, id: IdType) -> Result<T, ApiError>
        where for<'q> &'q IdType: sqlx::Encode<'q, sqlx::Postgres> + sqlx::Type<sqlx::Postgres>
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

    query
        .execute(&self.db_pool)
        .await
        .map_err(|_| ApiError {
            message: "Error".to_owned(),
            error_code: Some(44),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(item)
}




}
