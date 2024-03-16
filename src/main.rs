// main.rs

use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, Pool, Postgres};

use thiserror::Error;
use warp::{http::StatusCode, Filter, Rejection, Reply};
mod api{
    pub mod handlers;
    pub mod routes;
}

mod config;
mod db;
mod utils{
    pub mod errors;
}

mod services{
    pub mod mock_analysis;
}
mod analysis;

#[derive(Debug, Error)]
enum ApiError {
    #[error("Internal Server Error")]
    InternalServerError,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
}

async fn get_user(pool: PgPool, id: i32) -> Result<User, ApiError> {
    let result = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(user) => Ok(user),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

async fn handle_get_user(id: i32, pool: PgPool) -> Result<impl Reply, Rejection> {
    match get_user(pool, id).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(_) => {
            let error_message = "Internal Server Error".to_string();
            let json_error = warp::reply::json(&error_message);
            Ok(json_error)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let pool = db::connect().await;
    let routes = api::routes::routes(pool.clone());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

fn with_db(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
