use warp::Filter;
use super::handlers;
use sqlx::PgPool;

// A function to build our routes
pub fn routes(db_pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    photo_routes(db_pool)
}

// A route to handle GET requests for a specific post
fn photo_routes(db_pool: PgPool) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("upload")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 1024 * 10)) // Adjust limit as needed
        .and(warp::body::bytes())
        .and(with_db_pool(db_pool.clone()))
        .and_then(handlers::upload_photo)
}

fn with_db_pool(pool: PgPool) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
