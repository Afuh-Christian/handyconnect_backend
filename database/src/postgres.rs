use sqlx::{migrate, postgres::PgPoolOptions, PgPool};


pub async fn db_postgres_pool() -> sqlx::PgPool {

       let database_url: String = (*utils::constants::DATABASE_URL).clone();

    // Connect to the database
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    pool

}