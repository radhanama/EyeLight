use sqlx::{Pool, Postgres};
use std::env;

pub async fn connect() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");

    // Attempt to connect to the database
    match Pool::connect(&database_url).await {
        Ok(pool) => pool, // Return the pool if connection is successful
        Err(err) => panic!("Failed to connect to the database: {}", err), // Panic with error message if connection fails
    }
}
