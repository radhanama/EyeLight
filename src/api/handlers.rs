// routes/photo.rs

use warp::{Rejection, Reply};
use crate::db;
use crate::utils::errors::PhotoError;
use crate::services::mock_analysis;
use bytes::Bytes;
use sqlx::PgPool;

pub async fn upload_photo(data: Bytes, db_pool: PgPool) -> Result<impl Reply, Rejection> {

    let analysis_result = mock_analysis::analyze_image(data.clone()).await;

    let photo = db::models::Photo::new(data, analysis_result);
    match db::repository::insert(photo, &db_pool).await {
        Ok(new_photo) => Ok(warp::reply::json(&new_photo)),
        Err(err) => Err(warp::reject::custom(PhotoError::from(err))),
    }
}
